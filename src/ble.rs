//! BLE transport and high-level BSC200 protocol operations.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use prost::Message;
use tokio::sync::{mpsc, Mutex};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

mod platform;

use platform::{Link, PairingAgent};

use crate::activity;
use crate::cnx::CnxRoute;
use crate::frame::{
    crc8, encode_request, FrameAssembler, FrameHeader, END_FILE_LAST, FILE_TAG, HEADER_LEN,
    RESERVED, TYPE_CONFIRM, TYPE_NOTIFY,
};
use crate::gpx_import;
use crate::proto::{
    op, service, BleMsg, CyclingDataMsg, DevVerInfoMsg, FileDownload, GeneralFileOperation,
    RoutePlanDataMsg,
};

pub const UART1_SERVICE: &str = "6e400001-b5a3-f393-e0a9-e50e24dcca9e";
pub const UART1_RX: &str = "6e400002-b5a3-f393-e0a9-e50e24dcca9e";
pub const UART1_TX: &str = "6e400003-b5a3-f393-e0a9-e50e24dcca9e";
pub const CONTROL_SERVICE: &str = "6e400001-b5a3-f393-e0a9-e50e24dcca8e";
pub const CONTROL_RX: &str = "6e400002-b5a3-f393-e0a9-e50e24dcca8e";
pub const CONTROL_TX: &str = "6e400003-b5a3-f393-e0a9-e50e24dcca8e";
pub const THIRD_SERVICE: &str = "6e400001-b5a3-f393-e0a9-e50e24dcca7e";
pub const THIRD_RX: &str = "6e400002-b5a3-f393-e0a9-e50e24dcca7e";
pub const THIRD_TX: &str = "6e400003-b5a3-f393-e0a9-e50e24dcca7e";
pub const FOURTH_SERVICE: &str = "6e400001-b5a3-f393-e0a9-e50e24dcca6e";
pub const FOURTH_RX: &str = "6e400002-b5a3-f393-e0a9-e50e24dcca6e";
pub const FOURTH_TX: &str = "6e400003-b5a3-f393-e0a9-e50e24dcca6e";
pub const BATTERY_SERVICE: &str = "0000180f-0000-1000-8000-00805f9b34fb";
pub const BATTERY_LEVEL: &str = "00002a19-0000-1000-8000-00805f9b34fb";

// NOTE: CONTROL_SERVICE/RX/TX above: RX UUID is a typo guard; the real control
// UUID ends in ...dcca8e for all three. We normalise by service lookup below.
pub(crate) fn uuid(s: &str) -> Uuid {
    Uuid::parse_str(s).expect("valid UUID")
}

/// Platform-specific pairing guidance for the GUI.
pub fn pairing_hint() -> &'static str {
    platform::pairing_hint()
}

/// Name of the active BLE backend, for logs and diagnostics.
pub fn backend_name() -> &'static str {
    platform::backend_name()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FramingMode {
    /// Generation 4: header + payload in one write on the data channel.
    Gen4,
    /// Generation 2/3: payload then header on the control characteristic.
    Gen3,
}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub address: String,
    pub name: String,
    pub rssi: Option<i16>,
    pub paired: bool,
}

#[derive(Debug, Clone, Default)]
pub struct VersionInfo {
    pub main_boot: Option<i32>,
    pub main_app: Option<i32>,
    pub ble_boot: Option<i32>,
    pub ble_app: Option<i32>,
    pub hardware: Option<i32>,
    pub protocol: Option<i32>,
    pub compile_time: Option<String>,
}

impl VersionInfo {
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();
        if let Some(v) = self.main_app {
            parts.push(format!("main {v}"));
        }
        if let Some(v) = self.ble_app {
            parts.push(format!("BLE {v}"));
        }
        if let Some(v) = self.protocol {
            parts.push(format!("proto {v}"));
        }
        if let Some(v) = self.hardware {
            parts.push(format!("hw {v}"));
        }
        if parts.is_empty() {
            "unknown".to_string()
        } else {
            parts.join(", ")
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActivityInfo {
    pub timestamp: i32,
    pub size: i32,
}

#[derive(Debug, Clone)]
pub struct RouteInfo {
    pub id: i32,
    pub name: String,
    pub distance_m: Option<i32>,
}

#[derive(Debug, Clone)]
pub enum Event {
    ScanUpdate(Vec<DeviceInfo>),
    ScanFinished,
    Connected {
        address: String,
        name: String,
        framing: FramingMode,
    },
    Disconnected,
    Version(VersionInfo),
    Framing(FramingMode),
    Battery(Option<u8>),
    Activities(Vec<ActivityInfo>),
    Routes(Vec<RouteInfo>),
    Progress {
        task: String,
        done: u64,
        total: u64,
        message: String,
    },
    Log {
        direction: &'static str,
        text: String,
    },
    Error(String),
    Done(String),
    PairingRequired {
        address: String,
        passkey: Option<u32>,
    },
}

#[derive(Debug, Clone)]
pub enum Command {
    Scan,
    StopScan,
    Connect(String),
    Disconnect,
    ReadVersion,
    ListActivities,
    DownloadActivities {
        ids: Vec<i32>,
        out_dir: PathBuf,
        export_gpx: bool,
    },
    UploadRoute {
        gpx_path: PathBuf,
        title: String,
        activate: bool,
    },
    ReadRoutes,
}

pub type CommandSender = mpsc::UnboundedSender<Command>;
pub type EventSender = mpsc::UnboundedSender<Event>;
pub type EventReceiver = mpsc::UnboundedReceiver<Event>;
pub type PairingSender = mpsc::UnboundedSender<bool>;

/// Spawn the BLE worker on a dedicated Tokio runtime thread.
pub fn spawn_worker() -> (CommandSender, EventReceiver, PairingSender) {
    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
    let (evt_tx, evt_rx) = mpsc::unbounded_channel();
    let (pair_tx, pair_rx) = mpsc::unbounded_channel();
    std::thread::Builder::new()
        .name("bsc200-ble".into())
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("tokio runtime");
            rt.block_on(worker_main(cmd_rx, evt_tx, pair_rx));
        })
        .expect("spawn BLE worker");
    (cmd_tx, evt_rx, pair_tx)
}

async fn worker_main(
    mut cmds: mpsc::UnboundedReceiver<Command>,
    events: EventSender,
    mut pair_rx: mpsc::UnboundedReceiver<bool>,
) {
    let mut scan_cancel: Option<CancellationToken> = None;
    let mut session: Option<DeviceSession> = None;
    let mut pairing_agent = PairingAgent::new();
    let pairing_pending: platform::PairingPending = Arc::new(Mutex::new(None));

    // Pairing confirmations must be resolvable while the worker is blocked
    // inside device.pair(), so use a dedicated task/channel for them.
    {
        let pending = pairing_pending.clone();
        tokio::spawn(async move {
            while let Some(accept) = pair_rx.recv().await {
                if let Some(tx) = pending.lock().await.take() {
                    let _ = tx.send(accept);
                }
            }
        });
    }

    loop {
        let cmd = match cmds.recv().await {
            Some(c) => c,
            None => break,
        };
        match cmd {
            Command::Scan => {
                if let Some(c) = scan_cancel.take() {
                    c.cancel();
                }
                let token = CancellationToken::new();
                scan_cancel = Some(token.clone());
                let ev = events.clone();
                tokio::spawn(async move {
                    if let Err(e) = platform::scan(ev.clone(), token).await {
                        let _ = ev.send(Event::Error(format!("scan: {e}")));
                        let _ = ev.send(Event::ScanFinished);
                    }
                });
            }
            Command::StopScan => {
                if let Some(c) = scan_cancel.take() {
                    c.cancel();
                }
            }
            Command::Connect(address) => {
                if let Some(c) = scan_cancel.take() {
                    c.cancel();
                }
                if session.is_some() {
                    let _ = events.send(Event::Error("already connected".into()));
                    continue;
                }
                pairing_agent.ensure(&events, &pairing_pending).await;
                match DeviceSession::open(&address, events.clone()).await {
                    Ok(mut s) => {
                        let _ = events.send(Event::Connected {
                            address: address.clone(),
                            name: s.name.clone(),
                            framing: s.framing,
                        });
                        if let Ok(v) = s.read_version(events.clone()).await {
                            let _ = events.send(Event::Version(v));
                            let _ = events.send(Event::Framing(s.framing));
                        }
                        if let Ok(b) = s.read_battery().await {
                            let _ = events.send(Event::Battery(b));
                        }
                        session = Some(s);
                    }
                    Err(e) => {
                        let _ = events.send(Event::Error(format!("connect: {e}")));
                    }
                }
            }
            Command::Disconnect => {
                if let Some(s) = session.take() {
                    let _ = s.disconnect().await;
                }
                let _ = events.send(Event::Disconnected);
            }
            Command::ReadVersion => {
                if let Some(s) = session.as_mut() {
                    match s.read_version(events.clone()).await {
                        Ok(v) => {
                            let _ = events.send(Event::Version(v));
                            let _ = events.send(Event::Done("version read".into()));
                        }
                        Err(e) => {
                            let _ = events.send(Event::Error(format!("version: {e}")));
                        }
                    }
                } else {
                    let _ = events.send(Event::Error("not connected".into()));
                }
            }
            Command::ListActivities => {
                if let Some(s) = session.as_mut() {
                    match s.list_activities(events.clone()).await {
                        Ok(list) => {
                            let _ = events.send(Event::Activities(list));
                            let _ = events.send(Event::Done("activity list read".into()));
                        }
                        Err(e) => {
                            let _ = events.send(Event::Error(format!("activity list: {e}")));
                        }
                    }
                } else {
                    let _ = events.send(Event::Error("not connected".into()));
                }
            }
            Command::DownloadActivities {
                ids,
                out_dir,
                export_gpx,
            } => {
                if let Some(s) = session.as_mut() {
                    match s
                        .download_activities(&ids, &out_dir, export_gpx, events.clone())
                        .await
                    {
                        Ok(n) => {
                            let _ = events.send(Event::Done(format!("saved {n} activity file(s)")));
                        }
                        Err(e) => {
                            let _ = events.send(Event::Error(format!("download: {e}")));
                        }
                    }
                } else {
                    let _ = events.send(Event::Error("not connected".into()));
                }
            }
            Command::UploadRoute {
                gpx_path,
                title,
                activate,
            } => {
                if let Some(s) = session.as_mut() {
                    match s
                        .upload_route(&gpx_path, &title, activate, events.clone())
                        .await
                    {
                        Ok(id) => {
                            let _ = events.send(Event::Done(format!("route uploaded (id {id})")));
                        }
                        Err(e) => {
                            let _ = events.send(Event::Error(format!("route upload: {e}")));
                        }
                    }
                } else {
                    let _ = events.send(Event::Error("not connected".into()));
                }
            }
            Command::ReadRoutes => {
                if let Some(s) = session.as_mut() {
                    match s.read_routes(events.clone()).await {
                        Ok(r) => {
                            let _ = events.send(Event::Routes(r));
                            let _ = events.send(Event::Done("route list read".into()));
                        }
                        Err(e) => {
                            let _ = events.send(Event::Error(format!("route list: {e}")));
                        }
                    }
                } else {
                    let _ = events.send(Event::Error("not connected".into()));
                }
            }
        }
    }
}

pub struct DeviceSession {
    link: Box<dyn Link>,
    pub name: String,
    pub framing: FramingMode,
    events: EventSender,
    notifications: mpsc::UnboundedReceiver<(u8, Vec<u8>)>,
}

impl DeviceSession {
    pub async fn open(address: &str, events: EventSender) -> Result<Self, String> {
        let connected = platform::connect(address, events.clone()).await?;
        let mut s = Self {
            link: connected.link,
            name: connected.name,
            framing: FramingMode::Gen4,
            events: events.clone(),
            notifications: connected.notifications,
        };

        // Default to gen-4 combined framing. read_version() will retry with
        // gen-3 split framing if the device does not answer.
        let _ = events.send(Event::Log {
            direction: "sys",
            text: format!(
                "using gen4 combined framing ({} backend; auto-fallback on version read)",
                platform::backend_name()
            ),
        });
        if let Err(e) = s.app_bind().await {
            let _ = s.events.send(Event::Log {
                direction: "sys",
                text: format!("binding handshake skipped/failed: {e}"),
            });
        }
        Ok(s)
    }

    pub async fn disconnect(&self) -> Result<(), String> {
        self.link.disconnect().await
    }

    fn upload_channel(&self) -> Option<u8> {
        if self.link.has_channel(4) {
            Some(4)
        } else if self.link.has_channel(1) {
            Some(1)
        } else {
            None
        }
    }

    fn log(&self, direction: &'static str, text: impl Into<String>) {
        let _ = self.events.send(Event::Log {
            direction,
            text: text.into(),
        });
    }

    async fn write_control(&self, data: &[u8]) -> Result<(), String> {
        self.link.write_control(data).await
    }

    async fn write_chunks(
        &self,
        channel: u8,
        data: &[u8],
        events: Option<&EventSender>,
        task: Option<&str>,
        total: u64,
    ) -> Result<(), String> {
        let chunk = self.link.max_chunk().max(20);
        let mut done = 0usize;
        for part in data.chunks(chunk) {
            self.link.write_data(channel, part).await?;
            done += part.len();
            if let (Some(ev), Some(t)) = (events, task) {
                let _ = ev.send(Event::Progress {
                    task: t.to_string(),
                    done: done as u64,
                    total,
                    message: format!("{done}/{total} bytes"),
                });
            }
            tokio::time::sleep(Duration::from_millis(4)).await;
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn send_request(
        &self,
        main_service: u8,
        main_op: u8,
        payload: &[u8],
        second_service: u8,
        byte3: u8,
        second_op: u8,
        channel: u8,
    ) -> Result<(), String> {
        if !self.link.has_channel(channel) {
            return Err(format!("channel {channel} unavailable"));
        }
        match self.framing {
            FramingMode::Gen4 => {
                let frame = encode_request(
                    main_service,
                    main_op,
                    payload,
                    second_service,
                    byte3,
                    second_op,
                );
                if let Ok(h) = FrameHeader::parse(&frame[..HEADER_LEN]) {
                    self.log(
                        "tx",
                        format!(
                            "{} | {}",
                            describe_header(&h),
                            hex(&frame[..frame.len().min(64)])
                        ),
                    );
                }
                self.write_chunks(channel, &frame, None, None, 0).await
            }
            FramingMode::Gen3 => {
                let header = FrameHeader::request(
                    main_service,
                    main_op,
                    payload.len(),
                    crc8(payload),
                    second_service,
                    byte3,
                    second_op,
                )
                .to_bytes();
                self.write_chunks(channel, payload, None, None, 0).await?;
                if let Ok(h) = FrameHeader::parse(&header[..HEADER_LEN]) {
                    self.log(
                        "tx",
                        format!(
                            "{} | {} | payload {}",
                            describe_header(&h),
                            hex(&header),
                            hex(&payload[..payload.len().min(32)])
                        ),
                    );
                }
                self.write_control(&header).await?;
                Ok(())
            }
        }
    }

    async fn wait_confirm(
        &mut self,
        main_service: u8,
        main_ops: &[u8],
        timeout: Duration,
    ) -> Result<FrameHeader, String> {
        let deadline = tokio::time::Instant::now() + timeout;
        loop {
            let remain = deadline.saturating_duration_since(tokio::time::Instant::now());
            if remain.is_zero() {
                return Err("timeout waiting for confirmation".into());
            }
            match tokio::time::timeout(remain, self.notifications.recv()).await {
                Ok(Some((_ch, data))) => {
                    if data.len() >= HEADER_LEN {
                        if let Ok(h) = FrameHeader::parse(&data[..HEADER_LEN]) {
                            if h.main_service == main_service
                                && main_ops.contains(&h.main_operation)
                            {
                                self.log("rx", describe_header(&h));
                                return Ok(h);
                            }
                        }
                    }
                }
                Ok(None) => return Err("notification channel closed".into()),
                Err(_) => return Err("timeout waiting for confirmation".into()),
            }
        }
    }

    async fn try_request(
        &mut self,
        main_service: u8,
        main_op: u8,
        payload: &[u8],
        expect_ops: &[u8],
        timeout: Duration,
        channel: u8,
    ) -> Result<(FrameHeader, Vec<u8>), String> {
        self.send_request(
            main_service,
            main_op,
            payload,
            RESERVED,
            RESERVED,
            RESERVED,
            channel,
        )
        .await?;
        let mut assembler = FrameAssembler::new();
        let deadline = tokio::time::Instant::now() + timeout;
        loop {
            let remain = deadline.saturating_duration_since(tokio::time::Instant::now());
            if remain.is_zero() {
                return Err("timeout".into());
            }
            match tokio::time::timeout(remain, self.notifications.recv()).await {
                Ok(Some((_ch, data))) => {
                    for (h, p) in assembler.push(&data).into_iter().flatten() {
                        if h.main_service == main_service && expect_ops.contains(&h.main_operation)
                        {
                            self.log(
                                "rx",
                                format!("{} | {}", describe_header(&h), hex(&p[..p.len().min(64)])),
                            );
                            if h.command_type == TYPE_CONFIRM {
                                if h.status != 0 {
                                    return Err(format!("device status {}", h.status));
                                }
                                // A successful generic ACK may precede the data
                                // frame; keep waiting for the actual response.
                                continue;
                            }
                            return Ok((h, p));
                        }
                    }
                }
                Ok(None) => return Err("notification channel closed".into()),
                Err(_) => return Err("timeout".into()),
            }
        }
    }

    /// Send an arbitrary request and collect raw notifications for `wait`.
    /// Intended for protocol exploration; not used by the GUI.
    pub async fn raw_exchange(
        &mut self,
        main_service: u8,
        main_op: u8,
        payload: &[u8],
        channel: u8,
        wait: Duration,
    ) -> Result<Vec<(u8, Vec<u8>)>, String> {
        self.raw_exchange_opts(
            main_service,
            RESERVED,
            main_op,
            RESERVED,
            RESERVED,
            payload,
            channel,
            wait,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn raw_exchange_opts(
        &mut self,
        main_service: u8,
        second_service: u8,
        main_op: u8,
        second_op: u8,
        byte3: u8,
        payload: &[u8],
        channel: u8,
        wait: Duration,
    ) -> Result<Vec<(u8, Vec<u8>)>, String> {
        self.send_request(
            main_service,
            main_op,
            payload,
            second_service,
            byte3,
            second_op,
            channel,
        )
        .await?;
        let deadline = tokio::time::Instant::now() + wait;
        let mut out = Vec::new();
        loop {
            let remain = deadline.saturating_duration_since(tokio::time::Instant::now());
            if remain.is_zero() {
                break;
            }
            match tokio::time::timeout(remain, self.notifications.recv()).await {
                Ok(Some((ch, data))) => out.push((ch, data)),
                Ok(None) => break,
                Err(_) => break,
            }
        }
        Ok(out)
    }

    /// Read the device's app-level binding record and acknowledge it. Some
    /// BSC200 firmware returns DataError for file operations until this is done.
    pub async fn app_bind(&mut self) -> Result<Option<String>, String> {
        let payload = BleMsg {
            service_type: Some(service::BLE as i32),
            ble_operate_type: Some(op::BLE_BOND_INFO as i32),
            ..Default::default()
        }
        .encode_to_vec();
        let (_h, p) = self
            .try_request(
                10,
                op::BLE_BOND_INFO,
                &payload,
                &[1],
                Duration::from_secs(6),
                1,
            )
            .await?;
        let msg = BleMsg::decode(p.as_slice()).map_err(|e| e.to_string())?;
        let Some(data) = msg.ble_data_msg.first() else {
            self.log("sys", "binding info: no member id");
            return Ok(None);
        };
        let Some(member_id) = data.member_id.clone() else {
            self.log("sys", "binding info: empty member id");
            return Ok(None);
        };
        if member_id.is_empty() {
            self.log("sys", "binding info: empty member id");
            return Ok(None);
        }
        self.log(
            "sys",
            format!("device is bound; acknowledging member id {member_id}"),
        );
        let payload = BleMsg::connect_status(member_id.clone(), 1).encode_to_vec();
        self.send_request(
            10,
            op::BLE_CONNECT_STATUS,
            &payload,
            RESERVED,
            RESERVED,
            RESERVED,
            1,
        )
        .await?;
        let h = self
            .wait_confirm(10, &[op::BLE_CONNECT_STATUS], Duration::from_secs(6))
            .await?;
        if h.status != 0 {
            self.log(
                "sys",
                format!("binding acknowledge returned status {}", h.status),
            );
        }
        Ok(Some(member_id))
    }

    pub fn set_framing(&mut self, framing: FramingMode) {
        self.framing = framing;
    }

    pub async fn read_battery(&self) -> Result<Option<u8>, String> {
        self.link.read_battery().await
    }

    pub async fn read_version(&mut self, events: EventSender) -> Result<VersionInfo, String> {
        let payload = DevVerInfoMsg::get().encode_to_vec();
        let mut last_err = String::new();
        let mut response = None;
        for attempt in 0..2 {
            match self
                .try_request(17, 2, &payload, &[2, 3], Duration::from_secs(8), 1)
                .await
            {
                Ok(v) => {
                    response = Some(v);
                    break;
                }
                Err(e) => {
                    last_err = e;
                    if attempt == 0 {
                        self.framing = match self.framing {
                            FramingMode::Gen4 => FramingMode::Gen3,
                            FramingMode::Gen3 => FramingMode::Gen4,
                        };
                        let _ = events.send(Event::Log {
                            direction: "sys",
                            text: format!(
                                "version request failed; retrying with {:?} framing",
                                self.framing
                            ),
                        });
                    }
                }
            }
        }
        let (_h, p) = response.ok_or(last_err)?;
        let msg =
            DevVerInfoMsg::decode(p.as_slice()).map_err(|e| format!("version protobuf: {e}"))?;
        let v = msg.version_message.unwrap_or_default();
        let info = VersionInfo {
            main_boot: v.main_boot_ver,
            main_app: v.main_app_ver,
            ble_boot: v.ble_boot_ver,
            ble_app: v.ble_app_ver,
            hardware: v.hardware_ver,
            protocol: v.protocol_ver,
            compile_time: v.compile_time,
        };
        let _ = events.send(Event::Log {
            direction: "sys",
            text: format!("version: {}", info.summary()),
        });
        Ok(info)
    }

    pub async fn list_activities(
        &mut self,
        events: EventSender,
    ) -> Result<Vec<ActivityInfo>, String> {
        let num_payload = CyclingDataMsg::list_num_get().encode_to_vec();
        let (_h, p) = match self
            .try_request(
                6,
                op::CYCLING_LIST_NUM_GET,
                &num_payload,
                &[8],
                Duration::from_secs(6),
                1,
            )
            .await
        {
            Ok(v) => v,
            Err(e) => {
                self.log(
                    "sys",
                    format!("activity list on channel 1 failed ({e}); trying channel 3"),
                );
                self.try_request(
                    6,
                    op::CYCLING_LIST_NUM_GET,
                    &num_payload,
                    &[8],
                    Duration::from_secs(8),
                    3,
                )
                .await?
            }
        };
        let count = CyclingDataMsg::decode(p.as_slice())
            .ok()
            .and_then(|m| m.list_msg)
            .and_then(|l| l.file_num)
            .unwrap_or(0);
        let _ = events.send(Event::Log {
            direction: "sys",
            text: format!("activity count: {count}"),
        });
        if count <= 0 {
            return Ok(Vec::new());
        }
        let end = (count - 1).min(9999);
        let payload = CyclingDataMsg::list_get(0, end).encode_to_vec();
        let (_h, p) = match self
            .try_request(
                6,
                op::CYCLING_LIST_GET,
                &payload,
                &[1, 2],
                Duration::from_secs(10),
                1,
            )
            .await
        {
            Ok(v) => v,
            Err(e) => {
                self.log(
                    "sys",
                    format!("activity LIST_GET on channel 1 failed ({e}); trying channel 3"),
                );
                self.try_request(
                    6,
                    op::CYCLING_LIST_GET,
                    &payload,
                    &[1, 2],
                    Duration::from_secs(15),
                    3,
                )
                .await?
            }
        };
        let msg = CyclingDataMsg::decode(p.as_slice())
            .map_err(|e| format!("activity list protobuf: {e}"))?;
        Ok(msg
            .cycling_data_file_flag_msg
            .into_iter()
            .filter_map(|f| {
                Some(ActivityInfo {
                    timestamp: f.timestamp?,
                    size: f.file_size.unwrap_or(0),
                })
            })
            .collect())
    }

    pub async fn download_activities(
        &mut self,
        ids: &[i32],
        out_dir: &std::path::Path,
        export_gpx: bool,
        events: EventSender,
    ) -> Result<usize, String> {
        // The device uses the activity list as session context; refresh it
        // before requesting individual files.
        if let Err(e) = self.list_activities(events.clone()).await {
            let _ = events.send(Event::Log {
                direction: "sys",
                text: format!("activity list refresh failed before download: {e}"),
            });
        }
        let mut saved = 0;
        for &id in ids {
            let _ = events.send(Event::Progress {
                task: "download".into(),
                done: 0,
                total: ids.len() as u64,
                message: format!("activity {id}"),
            });
            let mut bytes = self.download_activity(id, &events).await?;
            match crate::fit::validate(&bytes) {
                Ok(()) => match crate::fit::normalize(&bytes) {
                    Ok((fixed, true)) => {
                        let _ = events.send(Event::Log {
                            direction: "sys",
                            text:
                                "repaired FIT: activity moved to the end and num_sessions corrected"
                                    .into(),
                        });
                        bytes = fixed;
                    }
                    Ok((_, false)) => {}
                    Err(e) => {
                        let _ = events.send(Event::Log {
                            direction: "sys",
                            text: format!("warning: could not normalize FIT message order: {e}"),
                        });
                    }
                },
                Err(e) => {
                    let _ = events.send(Event::Log {
                        direction: "sys",
                        text: format!("warning: downloaded FIT failed integrity check: {e}"),
                    });
                }
            }
            let stem = activity::activity_stem(id);
            let path =
                activity::save_fit(&bytes, out_dir, &stem).map_err(|e| format!("save FIT: {e}"))?;
            let _ = events.send(Event::Log {
                direction: "sys",
                text: format!("saved {}", path.display()),
            });
            if export_gpx {
                let gpx_path = out_dir.join(format!("{stem}.gpx"));
                match activity::fit_to_gpx(&path, &gpx_path, &stem) {
                    Ok(n) => {
                        let _ = events.send(Event::Log {
                            direction: "sys",
                            text: format!("wrote {} ({n} points)", gpx_path.display()),
                        });
                    }
                    Err(e) => {
                        let _ = events.send(Event::Error(format!("GPX conversion: {e}")));
                    }
                }
            }
            saved += 1;
        }
        Ok(saved)
    }

    async fn download_activity(
        &mut self,
        id: i32,
        events: &EventSender,
    ) -> Result<Vec<u8>, String> {
        let payload = CyclingDataMsg::file_get(id).encode_to_vec();
        // Gen-4 file requests use byte3 = 0x55 (see SyncActivityDelegate:
        // `mByteArray[3] = 85`). The device answers cycling-data commands on
        // channel 3 in the app's generation>=3 path.
        let byte3 = match self.framing {
            FramingMode::Gen4 => 0x55,
            FramingMode::Gen3 => RESERVED,
        };
        self.send_request(
            6,
            op::CYCLING_FILE_GET,
            &payload,
            RESERVED,
            byte3,
            RESERVED,
            3,
        )
        .await?;
        match self.framing {
            FramingMode::Gen4 => self.download_activity_gen4(id, events).await,
            FramingMode::Gen3 => self.download_activity_gen3(id, events).await,
        }
    }

    /// Drop complete type-2 ACK frames from the front of a raw notification
    /// stream. Some devices acknowledge a FILE_GET before streaming the file.
    fn strip_leading_acks(raw: &mut Vec<u8>) {
        loop {
            if raw.len() < HEADER_LEN {
                return;
            }
            match FrameHeader::parse(&raw[..HEADER_LEN]) {
                Ok(h) if h.command_type == TYPE_CONFIRM => {
                    raw.drain(..HEADER_LEN);
                }
                _ => return,
            }
        }
    }

    async fn download_activity_gen4(
        &mut self,
        id: i32,
        events: &EventSender,
    ) -> Result<Vec<u8>, String> {
        let mut raw = Vec::new();
        let mut expected: Option<usize> = None;
        let deadline = tokio::time::Instant::now() + Duration::from_secs(120);
        loop {
            if let Some(total) = expected {
                if raw.len() >= total {
                    let pb_len = u32::from_be_bytes([raw[20], raw[21], raw[22], raw[23]]) as usize;
                    return Ok(raw[24 + pb_len..total].to_vec());
                }
            }
            let remain = deadline.saturating_duration_since(tokio::time::Instant::now());
            if remain.is_zero() {
                return Err(format!("activity {id}: download timeout"));
            }
            let data = tokio::time::timeout(remain, self.notifications.recv())
                .await
                .map_err(|_| format!("activity {id}: download timeout"))?
                .ok_or_else(|| "notification channel closed".to_string())?;
            // The device interleaves unsolicited notifications (for example the
            // periodic BACK frame on channel 2) with the file stream. Only
            // channel 3 carries file data.
            if data.0 != 3 {
                let _ = events.send(Event::Log {
                    direction: "sys",
                    text: format!(
                        "download: ignored {} bytes on channel {}",
                        data.1.len(),
                        data.0
                    ),
                });
                continue;
            }
            raw.extend_from_slice(&data.1);
            Self::strip_leading_acks(&mut raw);
            if let Some(total) = expected {
                let _ = events.send(Event::Progress {
                    task: "download".into(),
                    done: raw.len() as u64,
                    total: total as u64,
                    message: format!("activity {id}: {} / {} bytes", raw.len(), total),
                });
            }
            if expected.is_none() && raw.len() >= 24 {
                if let Ok(h) = FrameHeader::parse(&raw[..20]) {
                    if h.main_service == 6
                        && (h.main_operation == op::CYCLING_FILE_SEND
                            || h.main_operation == op::CYCLING_FILE_GET)
                    {
                        let pb_len =
                            u32::from_be_bytes([raw[20], raw[21], raw[22], raw[23]]) as usize;
                        if pb_len > 0 && pb_len < 1_000_000 && raw.len() >= 24 + pb_len {
                            if let Ok(fd) = FileDownload::decode(&raw[24..24 + pb_len]) {
                                let file_size = fd.file_size.unwrap_or(0).max(0) as usize;
                                expected = Some(24 + pb_len + file_size);
                            }
                        }
                    }
                }
            }
        }
    }

    async fn download_activity_gen3(
        &mut self,
        id: i32,
        events: &EventSender,
    ) -> Result<Vec<u8>, String> {
        let mut assembler = FrameAssembler::new();
        let mut out = Vec::new();
        let deadline = tokio::time::Instant::now() + Duration::from_secs(120);
        loop {
            let remain = deadline.saturating_duration_since(tokio::time::Instant::now());
            if remain.is_zero() {
                return Err(format!("activity {id}: download timeout"));
            }
            let data = tokio::time::timeout(remain, self.notifications.recv())
                .await
                .map_err(|_| format!("activity {id}: download timeout"))?
                .ok_or_else(|| "notification channel closed".to_string())?;
            for item in assembler.push(&data.1) {
                let (h, payload) = match item {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                if h.main_service != 6
                    || (h.main_operation != op::CYCLING_FILE_SEND
                        && h.main_operation != op::CYCLING_FILE_GET)
                {
                    continue;
                }
                if let Ok(msg) = CyclingDataMsg::decode(payload.as_slice()) {
                    if let Some(content) = msg.file_content {
                        out.extend_from_slice(&content);
                    }
                    if h.end_type == END_FILE_LAST {
                        return Ok(out);
                    }
                }
                let ack =
                    FrameHeader::confirm(6, h.main_operation, 0, RESERVED, RESERVED).to_bytes();
                let _ = self.write_control(&ack).await;
                let _ = events.send(Event::Progress {
                    task: "download".into(),
                    done: out.len() as u64,
                    total: 0,
                    message: format!("activity {id}: {} bytes", out.len()),
                });
            }
        }
    }

    pub async fn upload_route(
        &mut self,
        gpx_path: &std::path::Path,
        title: &str,
        activate: bool,
        events: EventSender,
    ) -> Result<i32, String> {
        let imported = gpx_import::load_gpx(gpx_path).map_err(|e| format!("GPX import: {e}"))?;
        let mut route = CnxRoute::new(
            if title.trim().is_empty() {
                &imported.name
            } else {
                title
            },
            imported.points,
        )
        .map_err(|e| format!("CNX: {e}"))?;
        if !imported.pois.is_empty() {
            route = route.with_pois(imported.pois);
        }
        let cnx = route.to_xml().into_bytes();
        let file_id = route.id as i32;
        let file_name = if title.trim().is_empty() {
            imported.name.chars().take(32).collect::<String>()
        } else {
            title.chars().take(32).collect::<String>()
        };
        let meta = GeneralFileOperation::route_plan(
            cnx.len() as i32,
            file_id,
            file_name.clone(),
            Some(route.distance_m.round() as i32),
            Some(route.ascent_m),
        )
        .encode_to_vec();
        let mut payload = Vec::with_capacity(4 + meta.len() + cnx.len());
        payload.extend_from_slice(&(meta.len() as u32).to_be_bytes());
        payload.extend_from_slice(&meta);
        payload.extend_from_slice(&cnx);

        let _ = events.send(Event::Progress {
            task: "upload".into(),
            done: 0,
            total: payload.len() as u64,
            message: format!(
                "{} points, {:.1} km",
                route.points.len(),
                route.distance_m / 1000.0
            ),
        });

        match self.framing {
            FramingMode::Gen4 => {
                let header = FrameHeader::request(
                    service::FILE_OPERATION,
                    op::ADD,
                    0,
                    0,
                    RESERVED,
                    FILE_TAG,
                    RESERVED,
                )
                .to_bytes();
                let mut blob = Vec::with_capacity(HEADER_LEN + payload.len());
                blob.extend_from_slice(&header);
                blob.extend_from_slice(&payload);
                let channel = self.upload_channel().ok_or("no upload characteristic")?;
                self.write_chunks(
                    channel,
                    &blob,
                    Some(&events),
                    Some("upload"),
                    blob.len() as u64,
                )
                .await?;
            }
            FramingMode::Gen3 => {
                let channel = self.upload_channel().ok_or("no upload characteristic")?;
                self.write_chunks(
                    channel,
                    &payload,
                    Some(&events),
                    Some("upload"),
                    payload.len() as u64,
                )
                .await?;
                let header = FrameHeader::request(
                    service::FILE_OPERATION,
                    op::ADD,
                    payload.len(),
                    crc8(&payload),
                    RESERVED,
                    FILE_TAG,
                    RESERVED,
                )
                .to_bytes();
                self.write_control(&header).await?;
            }
        }

        // Wait briefly for either a generic file-operation ACK or a
        // route-specific ACK. Some BSC200 firmware versions do not send one at
        // all, so fall back to verifying that the route appears in the list.
        let mut acknowledged = false;
        let deadline = tokio::time::Instant::now() + Duration::from_secs(6);
        loop {
            let remain = deadline.saturating_duration_since(tokio::time::Instant::now());
            if remain.is_zero() {
                break;
            }
            let data = match tokio::time::timeout(remain, self.notifications.recv()).await {
                Ok(Some(data)) => data,
                Ok(None) => return Err("notification channel closed".to_string()),
                Err(_) => break,
            };
            if data.0 == 2 || data.1.len() < HEADER_LEN {
                continue;
            }
            if let Ok(h) = FrameHeader::parse(&data.1[..HEADER_LEN]) {
                let ok = (h.main_service == service::FILE_OPERATION && h.main_operation == op::ADD)
                    || (h.main_service == service::ROUTE_PLAN
                        && h.main_operation == op::ROUTE_FILE_SEND);
                if ok {
                    if h.status != 0 {
                        return Err(format!("route upload rejected: status {}", h.status));
                    }
                    acknowledged = true;
                    break;
                }
            }
        }

        if !acknowledged {
            self.log(
                "sys",
                "route upload: no ACK received; verifying route list".to_string(),
            );
            match self.read_routes(events.clone()).await {
                Ok(routes) if routes.iter().any(|r| r.id == file_id) => {
                    self.log(
                        "sys",
                        format!("route {file_id} verified in device route list"),
                    );
                    let _ = events.send(Event::Routes(routes));
                }
                Ok(_) => return Err("route upload failed: route not found on device".into()),
                Err(e) => return Err(format!("route upload unverified: {e}")),
            }
        }

        if activate {
            let payload = RoutePlanDataMsg::use_file(
                file_id,
                file_name,
                Some(route.distance_m.round() as i32),
            )
            .encode_to_vec();
            self.send_request(
                service::ROUTE_PLAN,
                op::ROUTE_FILE_USE,
                &payload,
                RESERVED,
                RESERVED,
                RESERVED,
                4,
            )
            .await?;
            let h = self
                .wait_confirm(
                    service::ROUTE_PLAN,
                    &[op::ROUTE_FILE_USE],
                    Duration::from_secs(10),
                )
                .await?;
            if h.status != 0 {
                return Err(format!("route activation rejected: status {}", h.status));
            }
        }
        Ok(file_id)
    }

    pub async fn read_routes(&mut self, events: EventSender) -> Result<Vec<RouteInfo>, String> {
        let payload = RoutePlanDataMsg::list_num_get().encode_to_vec();
        let (_h, p) = match self
            .try_request(
                7,
                op::ROUTE_LIST_NUM_GET,
                &payload,
                &[7],
                Duration::from_secs(5),
                1,
            )
            .await
        {
            Ok(v) => v,
            Err(e) => {
                self.log(
                    "sys",
                    format!("route list on channel 1 failed ({e}); trying channel 4"),
                );
                self.try_request(
                    7,
                    op::ROUTE_LIST_NUM_GET,
                    &payload,
                    &[7],
                    Duration::from_secs(6),
                    4,
                )
                .await?
            }
        };
        let count = RoutePlanDataMsg::decode(p.as_slice())
            .ok()
            .and_then(|m| m.route_list_get_msg)
            .and_then(|l| l.file_num)
            .unwrap_or(0);
        let _ = events.send(Event::Log {
            direction: "sys",
            text: format!("route count: {count}"),
        });
        if count <= 0 {
            return Ok(Vec::new());
        }
        let payload = RoutePlanDataMsg::list_get(0, (count - 1).min(9999)).encode_to_vec();
        let (_h, p) = match self
            .try_request(
                7,
                op::ROUTE_LIST_GET,
                &payload,
                &[1, 2],
                Duration::from_secs(8),
                1,
            )
            .await
        {
            Ok(v) => v,
            Err(e) => {
                self.log(
                    "sys",
                    format!("route LIST_GET on channel 1 failed ({e}); trying channel 4"),
                );
                self.try_request(
                    7,
                    op::ROUTE_LIST_GET,
                    &payload,
                    &[1, 2],
                    Duration::from_secs(10),
                    4,
                )
                .await?
            }
        };
        let msg = RoutePlanDataMsg::decode(p.as_slice())
            .map_err(|e| format!("route list protobuf: {e}"))?;
        Ok(msg
            .route_plan_info_msg
            .into_iter()
            .filter_map(|r| {
                Some(RouteInfo {
                    id: r.id?,
                    name: r.name.unwrap_or_default(),
                    distance_m: r.total_distance,
                })
            })
            .collect())
    }

    /// Read the BLE binding status (best-effort; the device may not require it).
    pub async fn read_bond_status(&mut self) -> Result<bool, String> {
        let payload = BleMsg::bond_status_get().encode_to_vec();
        let (_h, p) = self
            .try_request(
                10,
                op::BLE_BOND_STATUS_GET,
                &payload,
                &[9],
                Duration::from_secs(5),
                1,
            )
            .await?;
        let msg = BleMsg::decode(p.as_slice()).map_err(|e| e.to_string())?;
        Ok(msg
            .ble_bond_status_msg
            .and_then(|s| s.bond_status)
            .map(|v| v == 1)
            .unwrap_or(false))
    }
}

/// Hex dump for the UI log.
pub fn hex(data: &[u8]) -> String {
    data.iter()
        .map(|b| format!("{b:02x}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Frame header summary for the UI log.
pub fn describe_header(h: &FrameHeader) -> String {
    match h.command_type {
        TYPE_CONFIRM => format!(
            "ACK svc={} op={} status={}",
            h.main_service, h.main_operation, h.status
        ),
        TYPE_NOTIFY => format!(
            "notify svc={} op={} len={} end={}",
            h.main_service, h.main_operation, h.data_size, h.end_type
        ),
        _ => format!(
            "req svc={} op={} len={} second={} end={}",
            h.main_service, h.main_operation, h.data_size, h.second_operation, h.end_type
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_leading_ack_frames() {
        let mut raw = Vec::new();
        raw.extend_from_slice(&FrameHeader::confirm(6, 4, 0, RESERVED, RESERVED).to_bytes());
        raw.extend_from_slice(&encode_request(6, 4, &[0xAA], RESERVED, RESERVED, RESERVED));
        DeviceSession::strip_leading_acks(&mut raw);
        assert_eq!(raw.len(), HEADER_LEN + 1);
        assert_eq!(
            FrameHeader::parse(&raw[..HEADER_LEN]).unwrap().command_type,
            1
        );
    }
}
