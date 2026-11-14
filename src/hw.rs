//! Hardware test CLI (headless). Useful when no GUI display is available.

use std::path::PathBuf;

use tokio::sync::mpsc;

use crate::activity;
use crate::ble::{self, ActivityInfo, DeviceSession, Event};

#[derive(Debug, Clone)]
pub enum HwCommand {
    Scan,
    Info {
        address: String,
    },
    Activities {
        address: String,
    },
    Routes {
        address: String,
    },
    Download {
        address: String,
        out_dir: PathBuf,
        all: bool,
        ids: Vec<i32>,
        export_gpx: bool,
    },
    Upload {
        address: String,
        gpx: PathBuf,
        title: Option<String>,
        activate: bool,
    },
    Raw {
        address: String,
        service: u8,
        op: u8,
        payload: Vec<u8>,
        channel: u8,
        wait_secs: u64,
        framing: Option<crate::ble::FramingMode>,
        second_service: u8,
        second_op: u8,
        byte3: u8,
    },
}

fn print_event(ev: Event) {
    match ev {
        Event::Log { direction, text } => println!("[{direction}] {text}"),
        Event::Progress {
            task,
            done,
            total,
            message,
        } => {
            if total == 0 {
                println!("[progress] {task}: {message} ({done})");
            } else {
                println!("[progress] {task}: {done}/{total} {message}");
            }
        }
        Event::Version(v) => println!("[version] {}", v.summary()),
        Event::Battery(b) => println!("[battery] {b:?}"),
        Event::Activities(list) => {
            println!("[activities] {} file(s)", list.len());
            for a in list {
                println!(
                    "  {} ({})  {} bytes",
                    activity::activity_stem(a.timestamp),
                    a.timestamp,
                    a.size
                );
            }
        }
        Event::Routes(list) => {
            println!("[routes] {} route(s)", list.len());
            for r in list {
                println!("  {}  {}  {:?} m", r.id, r.name, r.distance_m);
            }
        }
        Event::Error(e) => eprintln!("[error] {e}"),
        Event::Done(m) => println!("[done] {m}"),
        Event::Connected {
            address,
            name,
            framing,
        } => println!("[connected] {name} {address} framing={framing:?}"),
        Event::Disconnected => println!("[disconnected]"),
        Event::ScanUpdate(list) => {
            for d in list {
                println!(
                    "  {}  {}  rssi={:?} paired={}",
                    d.address, d.name, d.rssi, d.paired
                );
            }
        }
        Event::ScanFinished => println!("[scan] finished"),
        Event::Framing(f) => println!("[framing] {f:?}"),
        Event::PairingRequired { address, passkey } => {
            println!("[pairing] {address} passkey={passkey:?}")
        }
    }
}

async fn open(address: &str) -> Result<(DeviceSession, mpsc::UnboundedSender<Event>), String> {
    let (tx, mut rx) = mpsc::unbounded_channel::<Event>();
    tokio::spawn(async move {
        while let Some(ev) = rx.recv().await {
            print_event(ev);
        }
    });
    let dev = DeviceSession::open(address, tx.clone()).await?;
    Ok((dev, tx))
}

async fn run_hw(cmd: HwCommand) {
    match cmd {
        HwCommand::Scan => {
            let (cmds, mut rx, _pairing) = ble::spawn_worker();
            let _ = cmds.send(ble::Command::Scan);
            while let Some(ev) = rx.recv().await {
                let done = matches!(ev, Event::ScanFinished);
                print_event(ev);
                if done {
                    break;
                }
            }
        }
        HwCommand::Info { address } => match open(&address).await {
            Ok((mut dev, tx)) => {
                if let Err(e) = dev.read_version(tx.clone()).await {
                    eprintln!("[error] version: {e}");
                }
                match dev.read_battery().await {
                    Ok(b) => println!("[battery] {b:?}"),
                    Err(e) => eprintln!("[error] battery: {e}"),
                }
                println!("[framing] {:?}", dev.framing);
                let _ = dev.disconnect().await;
            }
            Err(e) => eprintln!("[error] connect: {e}"),
        },
        HwCommand::Activities { address } => match open(&address).await {
            Ok((mut dev, tx)) => {
                match dev.list_activities(tx.clone()).await {
                    Ok(list) => {
                        println!("[activities] {} file(s)", list.len());
                        for a in list {
                            println!(
                                "  {} ({})  {} bytes",
                                activity::activity_stem(a.timestamp),
                                a.timestamp,
                                a.size
                            );
                        }
                    }
                    Err(e) => eprintln!("[error] activities: {e}"),
                }
                let _ = dev.disconnect().await;
            }
            Err(e) => eprintln!("[error] connect: {e}"),
        },
        HwCommand::Routes { address } => match open(&address).await {
            Ok((mut dev, tx)) => {
                match dev.read_routes(tx.clone()).await {
                    Ok(list) => {
                        println!("[routes] {} route(s)", list.len());
                        for r in list {
                            println!("  {}  {}  {:?} m", r.id, r.name, r.distance_m);
                        }
                    }
                    Err(e) => eprintln!("[error] routes: {e}"),
                }
                let _ = dev.disconnect().await;
            }
            Err(e) => eprintln!("[error] connect: {e}"),
        },
        HwCommand::Download {
            address,
            out_dir,
            all,
            ids,
            export_gpx,
        } => match open(&address).await {
            Ok((mut dev, tx)) => {
                let mut ids = ids;
                if all {
                    match dev.list_activities(tx.clone()).await {
                        Ok(list) => {
                            ids = list
                                .into_iter()
                                .map(|a: ActivityInfo| a.timestamp)
                                .collect()
                        }
                        Err(e) => {
                            eprintln!("[error] list: {e}");
                            let _ = dev.disconnect().await;
                            return;
                        }
                    }
                }
                if ids.is_empty() {
                    eprintln!("[error] no activity ids (use --all or --id N)");
                } else {
                    match dev
                        .download_activities(&ids, &out_dir, export_gpx, tx.clone())
                        .await
                    {
                        Ok(n) => println!("[done] saved {n} file(s) to {}", out_dir.display()),
                        Err(e) => eprintln!("[error] download: {e}"),
                    }
                }
                let _ = dev.disconnect().await;
            }
            Err(e) => eprintln!("[error] connect: {e}"),
        },
        HwCommand::Raw {
            address,
            service,
            op,
            payload,
            channel,
            wait_secs,
            framing,
            second_service,
            second_op,
            byte3,
        } => match open(&address).await {
            Ok((mut dev, _tx)) => {
                if let Some(f) = framing {
                    dev.set_framing(f);
                    println!("[raw] forcing framing {f:?}");
                }
                match dev
                    .raw_exchange_opts(
                        service,
                        second_service,
                        op,
                        second_op,
                        byte3,
                        &payload,
                        channel,
                        std::time::Duration::from_secs(wait_secs),
                    )
                    .await
                {
                    Ok(list) => {
                        println!("[raw] {} notification(s)", list.len());
                        for (ch, data) in list {
                            println!("  ch={ch} len={} {}", data.len(), crate::ble::hex(&data));
                            if data.len() >= 20 {
                                if let Ok(h) = crate::frame::FrameHeader::parse(&data[..20]) {
                                    println!("    {}", crate::ble::describe_header(&h));
                                    if h.command_type != 2 && data.len() >= 20 + h.data_size {
                                        println!(
                                            "    payload {}",
                                            crate::ble::hex(&data[20..20 + h.data_size])
                                        );
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("[error] raw: {e}"),
                }
                let _ = dev.disconnect().await;
            }
            Err(e) => eprintln!("[error] connect: {e}"),
        },
        HwCommand::Upload {
            address,
            gpx,
            title,
            activate,
        } => match open(&address).await {
            Ok((mut dev, tx)) => {
                let title = title.unwrap_or_else(|| {
                    gpx.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("Route")
                        .to_string()
                });
                match dev.upload_route(&gpx, &title, activate, tx.clone()).await {
                    Ok(id) => println!("[done] route uploaded id={id}"),
                    Err(e) => eprintln!("[error] upload: {e}"),
                }
                let _ = dev.disconnect().await;
            }
            Err(e) => eprintln!("[error] connect: {e}"),
        },
    }
}

pub fn run(cmd: HwCommand) -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    rt.block_on(run_hw(cmd));
    Ok(())
}
