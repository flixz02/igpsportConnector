//! Linux BLE backend built on BlueZ via `bluer`.

use std::collections::BTreeMap;
use std::time::Duration;

use bluer::agent::{Agent, AgentHandle, ReqError};
use bluer::gatt::remote::{Characteristic, CharacteristicWriteRequest};
use bluer::gatt::WriteOp;
use bluer::{Adapter, AdapterEvent, Address, Device, Session};
use futures::{pin_mut, StreamExt};
use tokio::sync::{mpsc, oneshot};
use tokio_util::sync::CancellationToken;

use super::{ConnectedLink, Link, PairingPending};
use crate::ble::{
    uuid, DeviceInfo, Event, EventSender, BATTERY_LEVEL, BATTERY_SERVICE, CONTROL_RX,
    CONTROL_SERVICE, CONTROL_TX, FOURTH_RX, FOURTH_SERVICE, FOURTH_TX, THIRD_RX, THIRD_SERVICE,
    THIRD_TX, UART1_RX, UART1_SERVICE, UART1_TX,
};

pub fn platform_name() -> &'static str {
    "Linux (BlueZ)"
}

pub fn pairing_hint() -> &'static str {
    "Pairing uses BlueZ. If the device requires a passkey, confirm it on the BSC200 \
     and accept the system prompt. The app also asks BlueZ to trust the device."
}

/// Registers a BlueZ pairing agent for the lifetime of the worker.
pub struct PairingAgent {
    handle: Option<AgentHandle>,
    session: Option<Session>,
}

impl PairingAgent {
    pub fn new() -> Self {
        Self {
            handle: None,
            session: None,
        }
    }

    pub async fn ensure(&mut self, events: &EventSender, pending: &PairingPending) {
        if self.handle.is_some() {
            return;
        }
        if let Ok(session) = Session::new().await {
            let agent = make_agent(events.clone(), pending.clone());
            match session.register_agent(agent).await {
                Ok(handle) => {
                    self.handle = Some(handle);
                    self.session = Some(session);
                }
                Err(e) => {
                    let _ = events.send(Event::Log {
                        direction: "sys",
                        text: format!("pairing agent unavailable: {e}"),
                    });
                }
            }
        }
    }
}

pub async fn scan(events: EventSender, cancel: CancellationToken) -> Result<(), String> {
    let session = Session::new().await.map_err(|e| e.to_string())?;
    let adapter = session.default_adapter().await.map_err(|e| e.to_string())?;
    adapter.set_powered(true).await.map_err(|e| e.to_string())?;
    scan_task(adapter, events, cancel).await
}

async fn scan_task(
    adapter: Adapter,
    events: EventSender,
    cancel: CancellationToken,
) -> Result<(), String> {
    let _ = adapter
        .set_discovery_filter(bluer::DiscoveryFilter {
            transport: bluer::DiscoveryTransport::Le,
            ..Default::default()
        })
        .await;
    let stream = adapter
        .discover_devices()
        .await
        .map_err(|e| e.to_string())?;
    pin_mut!(stream);
    let mut seen: BTreeMap<String, DeviceInfo> = Default::default();
    let deadline = tokio::time::sleep(Duration::from_secs(12));
    tokio::pin!(deadline);
    loop {
        tokio::select! {
            _ = cancel.cancelled() => break,
            _ = &mut deadline => break,
            ev = stream.next() => {
                match ev {
                    Some(AdapterEvent::DeviceAdded(addr)) => {
                        if let Ok(dev) = adapter.device(addr) {
                            let alias = dev.alias().await.unwrap_or_default();
                            let name = if alias.trim().is_empty() {
                                dev.name().await.ok().flatten().unwrap_or_default()
                            } else { alias };
                            if name.trim().is_empty() { continue; }
                            let info = DeviceInfo {
                                address: addr.to_string(),
                                name,
                                rssi: dev.rssi().await.ok().flatten(),
                                paired: dev.is_paired().await.unwrap_or(false),
                            };
                            seen.insert(info.address.clone(), info);
                            let _ = events.send(Event::ScanUpdate(seen.values().cloned().collect()));
                        }
                    }
                    Some(_) => {}
                    None => break,
                }
            }
        }
    }
    let _ = events.send(Event::ScanFinished);
    Ok(())
}

pub async fn connect(address: &str, events: EventSender) -> Result<ConnectedLink, String> {
    let session = Session::new().await.map_err(|e| e.to_string())?;
    let adapter = session.default_adapter().await.map_err(|e| e.to_string())?;
    adapter.set_powered(true).await.map_err(|e| e.to_string())?;

    let addr: Address = address.parse().map_err(|e| format!("bad address: {e}"))?;
    let device = adapter.device(addr).map_err(|e| e.to_string())?;
    if !device.is_connected().await.map_err(|e| e.to_string())? {
        device.connect().await.map_err(|e| e.to_string())?;
    }
    let _ = device.set_trusted(true).await;
    if !device.is_paired().await.unwrap_or(false) {
        let _ = events.send(Event::Log {
            direction: "sys",
            text: "device not paired; requesting pairing".into(),
        });
        if let Err(e) = device.pair().await {
            let _ = events.send(Event::Log {
                direction: "sys",
                text: format!("pairing failed (continuing anyway): {e}"),
            });
        }
    }
    let name = device.alias().await.unwrap_or_default();

    let mut main_rx = None;
    let mut main_tx = None;
    let mut control_tx = None;
    let mut control_rx = None;
    let mut third_tx = None;
    let mut third_rx = None;
    let mut fourth_tx = None;
    let mut fourth_rx = None;
    let mut battery_char = None;

    for svc in device.services().await.map_err(|e| e.to_string())? {
        let su = svc.uuid().await.map_err(|e| e.to_string())?;
        if su == uuid(BATTERY_SERVICE) {
            for ch in svc.characteristics().await.map_err(|e| e.to_string())? {
                if ch.uuid().await.map_err(|e| e.to_string())? == uuid(BATTERY_LEVEL) {
                    battery_char = Some(ch);
                }
            }
            continue;
        }
        let chars = svc.characteristics().await.map_err(|e| e.to_string())?;
        let mut rx = None;
        let mut tx = None;
        for ch in chars {
            let cu = ch.uuid().await.map_err(|e| e.to_string())?;
            if cu == uuid(UART1_RX)
                || cu == uuid(CONTROL_RX)
                || cu == uuid(THIRD_RX)
                || cu == uuid(FOURTH_RX)
            {
                rx = Some(ch);
            } else if cu == uuid(UART1_TX)
                || cu == uuid(CONTROL_TX)
                || cu == uuid(THIRD_TX)
                || cu == uuid(FOURTH_TX)
            {
                tx = Some(ch);
            }
        }
        if su == uuid(UART1_SERVICE) {
            main_rx = rx;
            main_tx = tx;
        } else if su == uuid(CONTROL_SERVICE) {
            control_rx = rx;
            control_tx = tx;
        } else if su == uuid(THIRD_SERVICE) {
            third_rx = rx;
            third_tx = tx;
        } else if su == uuid(FOURTH_SERVICE) {
            fourth_rx = rx;
            fourth_tx = tx;
        }
    }

    let rx_main = main_rx.ok_or("main RX characteristic missing")?;
    if main_tx.is_none() {
        return Err("main TX (notify) characteristic missing".into());
    }
    let max_chunk = rx_main.mtu().await.unwrap_or(23).saturating_sub(3).max(20) as usize;

    let (ntx, nrx) = mpsc::unbounded_channel();
    let subscribe = |channel: u8, ch: Option<Characteristic>| {
        if let Some(ch) = ch {
            let tx = ntx.clone();
            tokio::spawn(async move {
                if let Ok(stream) = ch.notify().await {
                    pin_mut!(stream);
                    while let Some(v) = stream.next().await {
                        if tx.send((channel, v)).is_err() {
                            break;
                        }
                    }
                }
            });
        }
    };
    subscribe(1, main_tx);
    subscribe(2, control_tx);
    subscribe(3, third_tx);
    subscribe(4, fourth_tx);

    Ok(ConnectedLink {
        link: Box::new(BluerLink {
            device,
            rx_main,
            rx_control: control_rx,
            rx_third: third_rx,
            rx_fourth: fourth_rx,
            battery: battery_char,
            max_chunk,
        }),
        name,
        notifications: nrx,
    })
}

struct BluerLink {
    device: Device,
    rx_main: Characteristic,
    rx_control: Option<Characteristic>,
    rx_third: Option<Characteristic>,
    rx_fourth: Option<Characteristic>,
    battery: Option<Characteristic>,
    max_chunk: usize,
}

impl BluerLink {
    fn data_char(&self, channel: u8) -> Option<&Characteristic> {
        match channel {
            1 => Some(&self.rx_main),
            3 => self.rx_third.as_ref(),
            4 => self.rx_fourth.as_ref(),
            _ => None,
        }
    }

    async fn write_char(&self, ch: &Characteristic, data: &[u8]) -> Result<(), String> {
        let flags = ch.flags().await.unwrap_or_default();
        let op_type = if flags.write_without_response {
            WriteOp::Command
        } else {
            WriteOp::Request
        };
        let req = CharacteristicWriteRequest {
            op_type,
            ..Default::default()
        };
        ch.write_ext(data, &req)
            .await
            .map_err(|e| format!("write: {e}"))
    }
}

#[async_trait::async_trait]
impl Link for BluerLink {
    async fn write_data(&self, channel: u8, data: &[u8]) -> Result<(), String> {
        let ch = self
            .data_char(channel)
            .ok_or_else(|| format!("channel {channel} unavailable"))?;
        self.write_char(ch, data).await
    }

    async fn write_control(&self, data: &[u8]) -> Result<(), String> {
        let ch = self
            .rx_control
            .as_ref()
            .ok_or("control channel unavailable")?;
        self.write_char(ch, data).await
    }

    async fn read_battery(&self) -> Result<Option<u8>, String> {
        if let Some(ch) = &self.battery {
            let v = ch.read().await.map_err(|e| e.to_string())?;
            Ok(v.first().copied())
        } else {
            Ok(None)
        }
    }

    async fn disconnect(&self) -> Result<(), String> {
        self.device.disconnect().await.map_err(|e| e.to_string())
    }

    fn max_chunk(&self) -> usize {
        self.max_chunk
    }

    fn has_channel(&self, channel: u8) -> bool {
        self.data_char(channel).is_some()
    }
}

fn make_agent(events: EventSender, pending: PairingPending) -> Agent {
    // Confirmations (numeric comparison / just-works authorization) are shown
    // in the UI. Display-only passkeys are logged; BSC200 normally uses
    // Just Works or numeric comparison.
    let confirm_events = events.clone();
    let confirm_pending = pending.clone();
    let request_confirmation = Box::new(move |req: bluer::agent::RequestConfirmation| {
        let events = confirm_events.clone();
        let pending = confirm_pending.clone();
        Box::pin(async move {
            let (tx, rx) = oneshot::channel();
            *pending.lock().await = Some(tx);
            let _ = events.send(Event::PairingRequired {
                address: req.device.to_string(),
                passkey: Some(req.passkey),
            });
            match rx.await {
                Ok(true) => Ok(()),
                _ => Err(ReqError::Rejected),
            }
        })
            as std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), ReqError>> + Send>>
    });

    let auth_events = events.clone();
    let auth_pending = pending.clone();
    let request_authorization = Box::new(move |req: bluer::agent::RequestAuthorization| {
        let events = auth_events.clone();
        let pending = auth_pending.clone();
        Box::pin(async move {
            let (tx, rx) = oneshot::channel();
            *pending.lock().await = Some(tx);
            let _ = events.send(Event::PairingRequired {
                address: req.device.to_string(),
                passkey: None,
            });
            match rx.await {
                Ok(true) => Ok(()),
                _ => Err(ReqError::Rejected),
            }
        })
            as std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), ReqError>> + Send>>
    });

    let display_events = events.clone();
    let display_passkey = Box::new(move |req: bluer::agent::DisplayPasskey| {
        let events = display_events.clone();
        Box::pin(async move {
            let _ = events.send(Event::PairingRequired {
                address: req.device.to_string(),
                passkey: Some(req.passkey),
            });
            Ok(())
        })
            as std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), ReqError>> + Send>>
    });

    Agent {
        request_default: true,
        request_confirmation: Some(request_confirmation),
        request_authorization: Some(request_authorization),
        display_passkey: Some(display_passkey),
        ..Default::default()
    }
}
