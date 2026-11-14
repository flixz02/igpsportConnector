//! Windows BLE backend built on WinRT via `btleplug`.
//!
//! Pairing is handled by Windows itself. If the BSC200 is not paired yet,
//! pair it once in *Settings → Bluetooth & devices*; the app then connects to
//! the already-paired device.

use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;
use std::time::Duration;

use btleplug::api::{
    BDAddr, Central, CharPropFlags, Characteristic, Manager as _, Peripheral as _,
    RetrievePeripheralsOptions, ScanFilter, WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral, PeripheralId};
use futures::StreamExt;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use super::{ConnectedLink, Link, PairingPending};
use crate::ble::{
    uuid, DeviceInfo, Event, EventSender, BATTERY_LEVEL, CONTROL_RX, CONTROL_TX, FOURTH_RX,
    FOURTH_TX, THIRD_RX, THIRD_TX, UART1_RX, UART1_TX,
};

pub fn platform_name() -> &'static str {
    "Windows (WinRT)"
}

pub fn pairing_hint() -> &'static str {
    "Windows handles pairing. If the BSC200 is not paired yet, open \
     Settings → Bluetooth & devices, pair it once, then press Connect again."
}

/// Windows has no app-level pairing agent; the OS owns pairing.
pub struct PairingAgent;

impl PairingAgent {
    pub fn new() -> Self {
        Self
    }

    pub async fn ensure(&mut self, _events: &EventSender, _pending: &PairingPending) {}
}

pub async fn scan(events: EventSender, cancel: CancellationToken) -> Result<(), String> {
    let manager = Manager::new().await.map_err(|e| e.to_string())?;
    let adapters = manager.adapters().await.map_err(|e| e.to_string())?;
    let central = adapters
        .into_iter()
        .next()
        .ok_or("no Bluetooth adapter found")?;

    central
        .start_scan(ScanFilter::default())
        .await
        .map_err(|e| format!("start scan: {e}"))?;
    let mut stream = central
        .events()
        .await
        .map_err(|e| format!("scan events: {e}"))?;

    let deadline = tokio::time::sleep(Duration::from_secs(12));
    tokio::pin!(deadline);
    let mut tick = tokio::time::interval(Duration::from_millis(800));
    let mut seen: BTreeMap<String, DeviceInfo> = BTreeMap::new();

    loop {
        tokio::select! {
            _ = cancel.cancelled() => break,
            _ = &mut deadline => break,
            _ = tick.tick() => {}
            event = stream.next() => {
                if event.is_none() {
                    break;
                }
            }
        }

        if let Ok(peripherals) = central.peripherals().await {
            for peripheral in peripherals {
                let Ok(Some(props)) = peripheral.properties().await else {
                    continue;
                };
                let name = props
                    .local_name
                    .or(props.advertisement_name)
                    .unwrap_or_default();
                if name.trim().is_empty() {
                    continue;
                }
                let address = peripheral.address().to_string();
                seen.insert(
                    address.clone(),
                    DeviceInfo {
                        address,
                        name,
                        rssi: props.rssi,
                        // WinRT does not expose the bond state here.
                        paired: false,
                    },
                );
            }
            let _ = events.send(Event::ScanUpdate(seen.values().cloned().collect()));
        }
    }

    let _ = central.stop_scan().await;
    let _ = events.send(Event::ScanFinished);
    Ok(())
}

pub async fn connect(address: &str, events: EventSender) -> Result<ConnectedLink, String> {
    let manager = Manager::new().await.map_err(|e| e.to_string())?;
    let adapters = manager.adapters().await.map_err(|e| e.to_string())?;
    let central = adapters
        .into_iter()
        .next()
        .ok_or("no Bluetooth adapter found")?;

    let peripheral = find_peripheral(&central, address).await?;
    if !peripheral.is_connected().await.unwrap_or(false) {
        peripheral
            .connect_with_timeout(Duration::from_secs(20))
            .await
            .map_err(|e| format!("connect: {e}. {}", pairing_hint()))?;
    }
    peripheral
        .discover_services_with_timeout(Duration::from_secs(20))
        .await
        .map_err(|e| format!("service discovery: {e}"))?;

    let channels = [
        (1u8, uuid(UART1_RX), uuid(UART1_TX)),
        (2, uuid(CONTROL_RX), uuid(CONTROL_TX)),
        (3, uuid(THIRD_RX), uuid(THIRD_TX)),
        (4, uuid(FOURTH_RX), uuid(FOURTH_TX)),
    ];
    let mut write_chars: HashMap<u8, Characteristic> = HashMap::new();
    let mut notify_chars: HashMap<u8, Characteristic> = HashMap::new();
    let mut battery = None;

    for characteristic in peripheral.characteristics() {
        for (channel, rx_uuid, tx_uuid) in channels {
            if characteristic.uuid == rx_uuid {
                write_chars.insert(channel, characteristic.clone());
            }
            if characteristic.uuid == tx_uuid {
                notify_chars.insert(channel, characteristic.clone());
            }
        }
        if characteristic.uuid == uuid(BATTERY_LEVEL) {
            battery = Some(characteristic);
        }
    }

    if !write_chars.contains_key(&1) {
        return Err("main RX characteristic missing".into());
    }
    if !notify_chars.contains_key(&1) {
        return Err("main TX (notify) characteristic missing".into());
    }

    for characteristic in notify_chars.values() {
        if characteristic.properties.contains(CharPropFlags::NOTIFY)
            || characteristic.properties.contains(CharPropFlags::INDICATE)
        {
            peripheral
                .subscribe(characteristic)
                .await
                .map_err(|e| format!("subscribe: {e}"))?;
        }
    }

    let mut uuid_to_channel: HashMap<Uuid, u8> = HashMap::new();
    for (channel, characteristic) in &notify_chars {
        uuid_to_channel.insert(characteristic.uuid, *channel);
    }

    let (ntx, nrx) = mpsc::unbounded_channel();
    let notification_peripheral = peripheral.clone();
    tokio::spawn(async move {
        if let Ok(mut stream) = notification_peripheral.notifications().await {
            while let Some(notification) = stream.next().await {
                let Some(channel) = uuid_to_channel.get(&notification.uuid).copied() else {
                    continue;
                };
                if ntx.send((channel, notification.value)).is_err() {
                    break;
                }
            }
        }
    });

    let name = peripheral
        .properties()
        .await
        .ok()
        .flatten()
        .and_then(|p| p.local_name)
        .filter(|n| !n.trim().is_empty())
        .unwrap_or_else(|| address.to_string());
    let max_chunk = (peripheral.mtu() as usize).saturating_sub(3).max(20);

    let _ = events.send(Event::Log {
        direction: "sys",
        text: format!("connected via Windows WinRT, MTU-derived chunk {max_chunk}"),
    });

    Ok(ConnectedLink {
        link: Box::new(BtleplugLink {
            peripheral,
            write_chars,
            battery,
            max_chunk,
        }),
        name,
        notifications: nrx,
    })
}

async fn find_peripheral(central: &Adapter, address: &str) -> Result<Peripheral, String> {
    let matches = |peripheral: &Peripheral| {
        peripheral
            .address()
            .to_string()
            .eq_ignore_ascii_case(address)
    };

    // Paired devices are not always advertising. WinRT can open a GATT
    // connection directly from the Bluetooth address.
    if let Ok(addr) = BDAddr::from_str(address) {
        let id = PeripheralId::from(addr);
        let options = RetrievePeripheralsOptions {
            identifiers: Some(vec![id]),
            services: None,
        };
        if let Ok(peripherals) = central.retrieve_peripherals(options).await {
            if let Some(peripheral) = peripherals.into_iter().next() {
                return Ok(peripheral);
            }
        }
    }

    if let Ok(peripherals) = central.peripherals().await {
        for peripheral in peripherals {
            if matches(&peripheral) {
                return Ok(peripheral);
            }
        }
    }

    let _ = central.start_scan(ScanFilter::default()).await;
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while tokio::time::Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(500)).await;
        if let Ok(peripherals) = central.peripherals().await {
            for peripheral in peripherals {
                if matches(&peripheral) {
                    let _ = central.stop_scan().await;
                    return Ok(peripheral);
                }
            }
        }
    }
    let _ = central.stop_scan().await;
    Err(format!(
        "device {address} not found. Make sure the BSC200 is awake and paired in \
         Windows Bluetooth settings."
    ))
}

struct BtleplugLink {
    peripheral: Peripheral,
    write_chars: HashMap<u8, Characteristic>,
    battery: Option<Characteristic>,
    max_chunk: usize,
}

#[async_trait::async_trait]
impl Link for BtleplugLink {
    async fn write_data(&self, channel: u8, data: &[u8]) -> Result<(), String> {
        let characteristic = self
            .write_chars
            .get(&channel)
            .ok_or_else(|| format!("channel {channel} unavailable"))?;
        let write_type = if characteristic
            .properties
            .contains(CharPropFlags::WRITE_WITHOUT_RESPONSE)
        {
            WriteType::WithoutResponse
        } else {
            WriteType::WithResponse
        };
        self.peripheral
            .write(characteristic, data, write_type)
            .await
            .map_err(|e| format!("write: {e}"))
    }

    async fn write_control(&self, data: &[u8]) -> Result<(), String> {
        self.write_data(2, data).await
    }

    async fn read_battery(&self) -> Result<Option<u8>, String> {
        if let Some(characteristic) = &self.battery {
            let value = self
                .peripheral
                .read(characteristic)
                .await
                .map_err(|e| e.to_string())?;
            Ok(value.first().copied())
        } else {
            Ok(None)
        }
    }

    async fn disconnect(&self) -> Result<(), String> {
        self.peripheral
            .disconnect()
            .await
            .map_err(|e| e.to_string())
    }

    fn max_chunk(&self) -> usize {
        self.max_chunk
    }

    fn has_channel(&self, channel: u8) -> bool {
        self.write_chars.contains_key(&channel)
    }
}
