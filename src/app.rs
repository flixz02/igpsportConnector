use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use eframe::egui;
use egui_file_dialog::{FileDialog, Filter};

use bsc200_tool::ble::{
    self, ActivityInfo, Command, DeviceInfo, Event, FramingMode, RouteInfo, VersionInfo,
};
use bsc200_tool::cnx::CnxRoute;
use bsc200_tool::fit::FIT_EPOCH_OFFSET;
use bsc200_tool::gpx_import;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct Config {
    output_dir: Option<PathBuf>,
    export_gpx: Option<bool>,
    activate_route: Option<bool>,
}

fn config_path() -> Option<PathBuf> {
    directories::ProjectDirs::from("", "", "bsc200-tool")
        .map(|p| p.config_dir().join("config.json"))
}

fn load_config() -> Config {
    config_path()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Device,
    Routes,
    Activities,
    Log,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DialogPurpose {
    None,
    RouteFile,
    OutputDir,
    SaveCnx,
}

struct ConnectedInfo {
    address: String,
    name: String,
    framing: FramingMode,
}

struct RoutePreview {
    path: PathBuf,
    title: String,
    route: CnxRoute,
}

struct LogLine {
    direction: String,
    text: String,
}

struct Progress {
    task: String,
    done: u64,
    total: u64,
    message: String,
}

pub struct App {
    cmds: ble::CommandSender,
    events: ble::EventReceiver,
    pairing_tx: ble::PairingSender,
    file_dialog: FileDialog,
    dialog_purpose: DialogPurpose,
    tab: Tab,

    devices: Vec<DeviceInfo>,
    selected_device: Option<String>,
    manual_address: String,
    scanning: bool,
    connected: Option<ConnectedInfo>,
    version: Option<VersionInfo>,
    battery: Option<u8>,

    activities: Vec<ActivityInfo>,
    selected_activities: BTreeSet<i32>,
    routes: Vec<RouteInfo>,

    output_dir: PathBuf,
    export_gpx: bool,
    activate_route: bool,

    route_preview: Option<RoutePreview>,
    cnx_preview_open: bool,

    logs: Vec<LogLine>,
    progress: BTreeMap<String, Progress>,
    status: String,
    error: Option<String>,
    pairing_prompt: Option<(String, Option<u32>)>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (cmds, events, pairing_tx) = ble::spawn_worker();
        let output_dir = directories::UserDirs::new()
            .and_then(|u| u.document_dir().map(|p| p.join("bsc200-export")))
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
        let config = load_config();
        let output_dir = config.output_dir.clone().unwrap_or(output_dir);
        let export_gpx = config.export_gpx.unwrap_or(true);
        let activate_route = config.activate_route.unwrap_or(true);
        let mut file_dialog = FileDialog::new();
        file_dialog = file_dialog.add_file_filter(
            "GPX tracks",
            Filter::new(|p: &std::path::Path| {
                p.extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s.eq_ignore_ascii_case("gpx"))
                    == Some(true)
            }),
        );
        file_dialog = file_dialog.add_file_filter(
            "CNX routes",
            Filter::new(|p: &std::path::Path| {
                p.extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s.eq_ignore_ascii_case("cnx"))
                    == Some(true)
            }),
        );

        Self {
            cmds,
            events,
            pairing_tx,
            file_dialog,
            dialog_purpose: DialogPurpose::None,
            tab: Tab::Device,
            devices: Vec::new(),
            selected_device: None,
            manual_address: String::new(),
            scanning: false,
            connected: None,
            version: None,
            battery: None,
            activities: Vec::new(),
            selected_activities: BTreeSet::new(),
            routes: Vec::new(),
            output_dir,
            export_gpx,
            activate_route,
            route_preview: None,
            cnx_preview_open: false,
            logs: Vec::new(),
            progress: BTreeMap::new(),
            status: "Ready".to_string(),
            error: None,
            pairing_prompt: None,
        }
    }

    fn send(&self, cmd: Command) {
        let _ = self.cmds.send(cmd);
    }

    fn poll_events(&mut self) {
        while let Ok(ev) = self.events.try_recv() {
            match ev {
                Event::ScanUpdate(list) => {
                    self.devices = list;
                }
                Event::ScanFinished => {
                    self.scanning = false;
                    self.status = format!("Scan finished, {} device(s)", self.devices.len());
                }
                Event::Connected {
                    address,
                    name,
                    framing,
                } => {
                    self.connected = Some(ConnectedInfo {
                        address: address.clone(),
                        name: name.clone(),
                        framing,
                    });
                    self.status = format!("Connected to {name} ({address})");
                    self.push_log("sys", format!("connected; framing = {framing:?}"));
                }
                Event::Disconnected => {
                    self.connected = None;
                    self.version = None;
                    self.battery = None;
                    self.activities.clear();
                    self.selected_activities.clear();
                    self.routes.clear();
                    self.status = "Disconnected".to_string();
                }
                Event::Version(v) => {
                    self.status = format!("Firmware: {}", v.summary());
                    self.version = Some(v);
                }
                Event::Framing(f) => {
                    if let Some(c) = &mut self.connected {
                        c.framing = f;
                    }
                }
                Event::Battery(b) => {
                    self.battery = b;
                }
                Event::Activities(list) => {
                    self.activities = list;
                    self.selected_activities =
                        self.activities.iter().map(|a| a.timestamp).collect();
                }
                Event::Routes(list) => {
                    self.routes = list;
                }
                Event::Progress {
                    task,
                    done,
                    total,
                    message,
                } => {
                    self.progress.insert(
                        task.clone(),
                        Progress {
                            task,
                            done,
                            total,
                            message,
                        },
                    );
                }
                Event::Log { direction, text } => {
                    self.push_log(direction, text);
                }
                Event::Error(e) => {
                    self.error = Some(e.clone());
                    self.push_log("err", e);
                    self.status = "Error".to_string();
                }
                Event::Done(msg) => {
                    self.status = msg.clone();
                    self.progress.clear();
                    self.push_log("ok", msg);
                }
                Event::PairingRequired { address, passkey } => {
                    self.push_log(
                        "pair",
                        format!(
                            "pairing requested by {address}{}",
                            passkey
                                .map(|p| format!(", passkey {p:06}"))
                                .unwrap_or_default()
                        ),
                    );
                    self.pairing_prompt = Some((address, passkey));
                }
            }
        }
    }

    fn push_log(&mut self, direction: &str, text: impl Into<String>) {
        self.logs.push(LogLine {
            direction: direction.to_string(),
            text: text.into(),
        });
        if self.logs.len() > 2000 {
            self.logs.drain(..500);
        }
    }

    fn load_route(&mut self, path: PathBuf) {
        match gpx_import::load_gpx(&path) {
            Ok(imported) => {
                let title = imported.name.clone();
                match CnxRoute::new(&title, imported.points) {
                    Ok(mut route) => {
                        if !imported.pois.is_empty() {
                            route = route.with_pois(imported.pois);
                        }
                        self.push_log(
                            "sys",
                            format!(
                                "loaded {}: {} points, {:.1} km",
                                path.display(),
                                route.points.len(),
                                route.distance_m / 1000.0
                            ),
                        );
                        self.route_preview = Some(RoutePreview { path, title, route });
                    }
                    Err(e) => self.error = Some(format!("CNX: {e}")),
                }
            }
            Err(e) => self.error = Some(format!("GPX: {e}")),
        }
    }

    fn device_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Device");
        ui.horizontal(|ui| {
            if ui
                .add_enabled(!self.scanning, egui::Button::new("Scan"))
                .clicked()
            {
                self.scanning = true;
                self.devices.clear();
                self.send(Command::Scan);
            }
            if ui
                .add_enabled(self.scanning, egui::Button::new("Stop"))
                .clicked()
            {
                self.send(Command::StopScan);
                self.scanning = false;
            }
            if let Some(c) = &self.connected {
                if ui.button("Disconnect").clicked() {
                    self.send(Command::Disconnect);
                    let _ = c;
                }
            }
        });
        ui.separator();

        if let Some(c) = &self.connected {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.label(format!("Connected: {} ({})", c.name, c.address));
                ui.label(format!("Framing: {:?}", c.framing));
                if let Some(v) = &self.version {
                    ui.label(format!("Firmware: {}", v.summary()));
                    if let Some(t) = &v.compile_time {
                        ui.label(format!("Compiled: {t}"));
                    }
                }
                if let Some(b) = self.battery {
                    ui.label(format!("Battery: {b}%"));
                }
                if ui.button("Read firmware version").clicked() {
                    self.send(Command::ReadVersion);
                }
            });
            ui.add_space(8.0);
        }

        ui.label(if self.scanning {
            "Scanning for BLE devices…"
        } else {
            "Nearby devices"
        });
        egui::ScrollArea::vertical()
            .max_height(320.0)
            .show(ui, |ui| {
                for d in &self.devices {
                    let selected = self.selected_device.as_deref() == Some(d.address.as_str());
                    let label = format!(
                        "{}{}  {}  {}{}",
                        d.name,
                        if d.paired { " [paired]" } else { "" },
                        d.address,
                        d.rssi.map(|r| format!("{r} dBm")).unwrap_or_default(),
                        if d.name.to_ascii_uppercase().contains("BSC200") {
                            "  ← BSC200"
                        } else {
                            ""
                        }
                    );
                    if ui.selectable_label(selected, label).clicked() {
                        self.selected_device = Some(d.address.clone());
                        self.manual_address = d.address.clone();
                    }
                }
            });
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label("Address:");
            ui.add(
                egui::TextEdit::singleline(&mut self.manual_address)
                    .hint_text("AA:BB:CC:DD:EE:FF")
                    .desired_width(170.0),
            );
            let can_connect = !self.manual_address.trim().is_empty() && self.connected.is_none();
            if ui
                .add_enabled(can_connect, egui::Button::new("Connect by address"))
                .clicked()
            {
                self.send(Command::Connect(self.manual_address.trim().to_string()));
            }
        });
        ui.horizontal(|ui| {
            let can_connect = self.selected_device.is_some() && self.connected.is_none();
            if ui
                .add_enabled(can_connect, egui::Button::new("Connect"))
                .clicked()
            {
                if let Some(addr) = &self.selected_device {
                    self.send(Command::Connect(addr.clone()));
                }
            }
            if ui
                .add_enabled(self.connected.is_some(), egui::Button::new("Read routes"))
                .clicked()
            {
                self.send(Command::ReadRoutes);
            }
        });
        ui.add_space(8.0);
        ui.weak(format!("BLE backend: {}", ble::backend_name()));
        ui.label(ble::pairing_hint());
    }

    fn routes_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Routes");
        ui.horizontal(|ui| {
            if ui.button("Choose GPX…").clicked() {
                self.dialog_purpose = DialogPurpose::RouteFile;
                self.file_dialog.pick_file();
            }
            if ui
                .add_enabled(
                    self.route_preview.is_some(),
                    egui::Button::new("Preview CNX"),
                )
                .clicked()
            {
                self.cnx_preview_open = true;
            }
            if ui
                .add_enabled(
                    self.route_preview.is_some() && self.connected.is_some(),
                    egui::Button::new("Upload to BSC200"),
                )
                .clicked()
            {
                if let Some(p) = &self.route_preview {
                    self.send(Command::UploadRoute {
                        gpx_path: p.path.clone(),
                        title: p.title.clone(),
                        activate: self.activate_route,
                    });
                }
            }
            if ui
                .add_enabled(
                    self.connected.is_some(),
                    egui::Button::new("Read device routes"),
                )
                .clicked()
            {
                self.send(Command::ReadRoutes);
            }
        });
        ui.checkbox(&mut self.activate_route, "Activate route after upload");
        ui.separator();

        if let Some(p) = &mut self.route_preview {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Title:");
                    ui.text_edit_singleline(&mut p.title);
                });
                ui.label(format!("File: {}", p.path.display()));
                ui.label(format!("Points: {}", p.route.points.len()));
                ui.label(format!("Distance: {:.2} km", p.route.distance_m / 1000.0));
                ui.label(format!(
                    "Ascent / descent: {} m / {} m",
                    p.route.ascent_m, p.route.descent_m
                ));
                ui.label(format!("CNX id: {}", p.route.id));
                ui.label(format!("POIs: {}", p.route.pois.len()));
                if ui.button("Save CNX…").clicked() {
                    self.dialog_purpose = DialogPurpose::SaveCnx;
                    self.file_dialog.save_file();
                }
            });
        } else {
            ui.label("Choose a GPX track to generate a native BSC200 .cnx route.");
        }

        ui.add_space(8.0);
        ui.label("Routes currently on the device");
        if self.routes.is_empty() {
            ui.weak("No routes loaded. Use “Read device routes”.");
        } else {
            egui::Grid::new("routes_grid").striped(true).show(ui, |ui| {
                ui.strong("ID");
                ui.strong("Name");
                ui.strong("Distance");
                ui.end_row();
                for r in &self.routes {
                    ui.label(r.id.to_string());
                    ui.label(&r.name);
                    ui.label(
                        r.distance_m
                            .map(|d| format!("{:.1} km", d as f64 / 1000.0))
                            .unwrap_or_default(),
                    );
                    ui.end_row();
                }
            });
        }
    }

    fn activities_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Activities");
        ui.horizontal(|ui| {
            if ui
                .add_enabled(self.connected.is_some(), egui::Button::new("Refresh list"))
                .clicked()
            {
                self.send(Command::ListActivities);
            }
            if ui.button("Select all").clicked() {
                self.selected_activities = self.activities.iter().map(|a| a.timestamp).collect();
            }
            if ui.button("Select none").clicked() {
                self.selected_activities.clear();
            }
        });
        ui.horizontal(|ui| {
            ui.label("Output folder:");
            let mut dir = self.output_dir.display().to_string();
            if ui.text_edit_singleline(&mut dir).changed() {
                self.output_dir = PathBuf::from(dir);
            }
            if ui.button("Choose…").clicked() {
                self.dialog_purpose = DialogPurpose::OutputDir;
                self.file_dialog.pick_directory();
            }
            if ui.button("Open").clicked() {
                open_in_file_manager(&self.output_dir);
            }
        });
        ui.checkbox(&mut self.export_gpx, "Also convert to GPX");
        ui.separator();

        if self.activities.is_empty() {
            ui.label("No activities loaded. Connect and press “Refresh list”.");
        } else {
            egui::ScrollArea::vertical()
                .max_height(360.0)
                .show(ui, |ui| {
                    egui::Grid::new("activities_grid")
                        .striped(true)
                        .show(ui, |ui| {
                            ui.strong("");
                            ui.strong("Device time");
                            ui.strong("Size");
                            ui.end_row();
                            for a in &self.activities {
                                let mut selected = self.selected_activities.contains(&a.timestamp);
                                if ui.checkbox(&mut selected, "").changed() {
                                    if selected {
                                        self.selected_activities.insert(a.timestamp);
                                    } else {
                                        self.selected_activities.remove(&a.timestamp);
                                    }
                                }
                                ui.label(format_timestamp(a.timestamp));
                                ui.label(format!("{:.1} KB", a.size as f64 / 1024.0));
                                ui.end_row();
                            }
                        });
                });
        }
        ui.add_space(6.0);
        let ids: Vec<i32> = self.selected_activities.iter().copied().collect();
        ui.horizontal(|ui| {
            if ui
                .add_enabled(
                    self.connected.is_some() && !ids.is_empty(),
                    egui::Button::new(format!("Download {} selected", ids.len())),
                )
                .clicked()
            {
                self.send(Command::DownloadActivities {
                    ids: ids.clone(),
                    out_dir: self.output_dir.clone(),
                    export_gpx: self.export_gpx,
                });
            }
            if ui
                .add_enabled(
                    self.connected.is_some() && !self.activities.is_empty(),
                    egui::Button::new("Download all"),
                )
                .clicked()
            {
                let all: Vec<i32> = self.activities.iter().map(|a| a.timestamp).collect();
                self.send(Command::DownloadActivities {
                    ids: all,
                    out_dir: self.output_dir.clone(),
                    export_gpx: self.export_gpx,
                });
            }
        });
    }

    fn log_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Protocol log");
        ui.horizontal(|ui| {
            if ui.button("Clear").clicked() {
                self.logs.clear();
            }
            if ui.button("Copy to clipboard").clicked() {
                let text = self
                    .logs
                    .iter()
                    .map(|l| format!("[{}] {}", l.direction, l.text))
                    .collect::<Vec<_>>()
                    .join("\n");
                ui.ctx().copy_text(text);
            }
        });
        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for l in &self.logs {
                    ui.monospace(format!("[{:>4}] {}", l.direction, l.text));
                }
            });
    }

    fn settings_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.label("Output directory:");
        ui.monospace(self.output_dir.display().to_string());
        if ui.button("Choose output directory…").clicked() {
            self.dialog_purpose = DialogPurpose::OutputDir;
            self.file_dialog.pick_directory();
        }
        ui.add_space(8.0);
        ui.checkbox(&mut self.export_gpx, "Export GPX alongside FIT");
        ui.checkbox(&mut self.activate_route, "Activate route after upload");
        ui.add_space(12.0);
        ui.heading("Protocol notes");
        ui.label(
            "CNX is generated locally with native iGPSPORT encoding:\n\
             • first track point: lat,lon,elevation_mm\n\
             • later lat/lon: 1st/2nd-order deltas (1e-7°)\n\
             • later elevation: 1st-order delta in cm\n\
             • Encode=2, Lang=0, 8-digit content-derived Id\n\n\
             The app auto-probes gen-4 combined framing and gen-3 split framing.",
        );
        ui.add_space(12.0);
        if let Some(e) = &self.error {
            ui.colored_label(egui::Color32::RED, e);
        }
    }

    fn show_progress(&self, ui: &mut egui::Ui) {
        for p in self.progress.values() {
            let frac = if p.total == 0 {
                0.0
            } else {
                (p.done as f32 / p.total as f32).clamp(0.0, 1.0)
            };
            ui.add(
                egui::ProgressBar::new(frac)
                    .text(format!("{}: {}", p.task, p.message))
                    .desired_width(360.0),
            );
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let cfg = Config {
            output_dir: Some(self.output_dir.clone()),
            export_gpx: Some(self.export_gpx),
            activate_route: Some(self.activate_route),
        };
        if let Some(path) = config_path() {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Ok(json) = serde_json::to_string_pretty(&cfg) {
                let _ = std::fs::write(path, json);
            }
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.poll_events();
        ui.ctx().request_repaint_after(Duration::from_millis(100));

        self.file_dialog.update(ui);
        if let Some(path) = self.file_dialog.take_picked() {
            match self.dialog_purpose {
                DialogPurpose::RouteFile => {
                    let path = path.to_path_buf();
                    self.load_route(path);
                }
                DialogPurpose::OutputDir => {
                    self.output_dir = path.to_path_buf();
                }
                DialogPurpose::SaveCnx => {
                    if let Some(p) = &self.route_preview {
                        let xml = p.route.to_xml();
                        match std::fs::write(&path, xml) {
                            Ok(()) => {
                                self.status = format!("Saved {}", path.display());
                                self.push_log("ok", self.status.clone());
                            }
                            Err(e) => self.error = Some(format!("save CNX: {e}")),
                        }
                    }
                }
                DialogPurpose::None => {}
            }
            self.dialog_purpose = DialogPurpose::None;
        }

        egui::Panel::top("top").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("iGPSPORT BSC200");
                ui.separator();
                ui.label(&self.status);
                if let Some(c) = &self.connected {
                    ui.separator();
                    ui.label(format!("{} / {:?}", c.address, c.framing));
                }
            });
        });

        egui::Panel::left("nav")
            .resizable(false)
            .exact_size(150.0)
            .show(ui, |ui| {
                ui.add_space(8.0);
                for (tab, name) in [
                    (Tab::Device, "Device"),
                    (Tab::Routes, "Routes"),
                    (Tab::Activities, "Activities"),
                    (Tab::Log, "Log"),
                    (Tab::Settings, "Settings"),
                ] {
                    if ui.selectable_label(self.tab == tab, name).clicked() {
                        self.tab = tab;
                    }
                }
                ui.add_space(12.0);
                if let Some(c) = &self.connected {
                    ui.separator();
                    ui.label("Connected");
                    ui.monospace(&c.name);
                    ui.weak(&c.address);
                } else {
                    ui.separator();
                    ui.weak("Not connected");
                }
            });

        egui::Panel::bottom("footer").show(ui, |ui| {
            if let Some(e) = self.error.clone() {
                let mut dismiss = false;
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::RED, &e);
                    if ui.small_button("dismiss").clicked() {
                        dismiss = true;
                    }
                });
                if dismiss {
                    self.error = None;
                }
            } else {
                self.show_progress(ui);
                ui.weak("BSC200 protocol: 20-byte header, CRC-8/MAXIM, protobuf payloads");
            }
        });

        egui::CentralPanel::default().show(ui, |ui| match self.tab {
            Tab::Device => self.device_tab(ui),
            Tab::Routes => self.routes_tab(ui),
            Tab::Activities => self.activities_tab(ui),
            Tab::Log => self.log_tab(ui),
            Tab::Settings => self.settings_tab(ui),
        });

        if let Some((address, passkey)) = self.pairing_prompt.clone() {
            let mut open = true;
            egui::Window::new("Bluetooth pairing")
                .collapsible(false)
                .resizable(false)
                .open(&mut open)
                .show(ui.ctx(), |ui| {
                    ui.label(format!("Pairing request from {address}"));
                    if let Some(p) = passkey {
                        ui.heading(format!("{p:06}"));
                        ui.label("Confirm this number matches the one shown on the BSC200.");
                    } else {
                        ui.label("Accept the pairing request?");
                    }
                    ui.horizontal(|ui| {
                        if ui.button("Accept").clicked() {
                            let _ = self.pairing_tx.send(true);
                            self.pairing_prompt = None;
                        }
                        if ui.button("Reject").clicked() {
                            let _ = self.pairing_tx.send(false);
                            self.pairing_prompt = None;
                        }
                    });
                });
            if !open {
                self.pairing_prompt = None;
            }
        }

        if self.cnx_preview_open {
            let mut open = self.cnx_preview_open;
            if let Some(p) = &self.route_preview {
                egui::Window::new("CNX preview")
                    .open(&mut open)
                    .default_size([700.0, 450.0])
                    .show(ui.ctx(), |ui| {
                        egui::ScrollArea::both().show(ui, |ui| {
                            ui.monospace(p.route.to_xml());
                        });
                    });
            }
            self.cnx_preview_open = open;
        }
    }
}

fn open_in_file_manager(path: &std::path::Path) {
    #[cfg(target_os = "windows")]
    let program = "explorer";
    #[cfg(target_os = "macos")]
    let program = "open";
    #[cfg(all(unix, not(target_os = "macos")))]
    let program = "xdg-open";
    let _ = std::process::Command::new(program).arg(path).spawn();
}

fn format_timestamp(ts: i32) -> String {
    use chrono::{TimeZone, Utc};
    Utc.timestamp_opt(ts as i64 + FIT_EPOCH_OFFSET, 0)
        .single()
        .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| ts.to_string())
}
