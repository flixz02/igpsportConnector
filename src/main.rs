mod app;

use app::App;
use bsc200_tool::hw::{run as run_hw, HwCommand};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        match args[1].as_str() {
            "--make-cnx" if args.len() == 4 => {
                let imported = bsc200_tool::gpx_import::load_gpx(&PathBuf::from(&args[2]))?;
                let mut route =
                    bsc200_tool::cnx::CnxRoute::new(imported.name.clone(), imported.points)?;
                if !imported.pois.is_empty() {
                    route = route.with_pois(imported.pois);
                }
                std::fs::write(&args[3], route.to_xml())?;
                println!(
                    "wrote {}: {} points, {:.2} km, id {}",
                    args[3],
                    route.points.len(),
                    route.distance_m / 1000.0,
                    route.id
                );
                return Ok(());
            }
            "--repair-fit" if args.len() == 4 => {
                let bytes = std::fs::read(&args[2])?;
                bsc200_tool::fit::validate(&bytes)?;
                let (fixed, repaired) = bsc200_tool::fit::normalize(&bytes)?;
                std::fs::write(&args[3], &fixed)?;
                println!(
                    "wrote {}: {} bytes, repaired={}",
                    args[3],
                    fixed.len(),
                    repaired
                );
                return Ok(());
            }
            "--hw-scan" => {
                run_hw(HwCommand::Scan)?;
                return Ok(());
            }
            "--hw-info" if args.len() == 3 => {
                run_hw(HwCommand::Info {
                    address: args[2].clone(),
                })?;
                return Ok(());
            }
            "--hw-activities" if args.len() == 3 => {
                run_hw(HwCommand::Activities {
                    address: args[2].clone(),
                })?;
                return Ok(());
            }
            "--hw-routes" if args.len() == 3 => {
                run_hw(HwCommand::Routes {
                    address: args[2].clone(),
                })?;
                return Ok(());
            }
            "--hw-download" if args.len() >= 4 => {
                let address = args[2].clone();
                let out_dir = PathBuf::from(&args[3]);
                let mut all = false;
                let mut ids = Vec::new();
                let mut export_gpx = false;
                let mut i = 4;
                while i < args.len() {
                    match args[i].as_str() {
                        "--all" => all = true,
                        "--gpx" => export_gpx = true,
                        "--id" if i + 1 < args.len() => {
                            if let Ok(id) = args[i + 1].parse() {
                                ids.push(id);
                            }
                            i += 1;
                        }
                        other => eprintln!("ignoring unknown download option {other}"),
                    }
                    i += 1;
                }
                run_hw(HwCommand::Download {
                    address,
                    out_dir,
                    all,
                    ids,
                    export_gpx,
                })?;
                return Ok(());
            }
            "--hw-upload" if args.len() >= 4 => {
                let address = args[2].clone();
                let gpx = PathBuf::from(&args[3]);
                let mut title = None;
                let mut activate = false;
                let mut i = 4;
                while i < args.len() {
                    match args[i].as_str() {
                        "--activate" => activate = true,
                        "--title" if i + 1 < args.len() => {
                            title = Some(args[i + 1].clone());
                            i += 1;
                        }
                        other => eprintln!("ignoring unknown upload option {other}"),
                    }
                    i += 1;
                }
                run_hw(HwCommand::Upload {
                    address,
                    gpx,
                    title,
                    activate,
                })?;
                return Ok(());
            }
            "--hw-raw" if args.len() >= 5 => {
                let address = args[2].clone();
                let service: u8 = args[3].parse()?;
                let op: u8 = args[4].parse()?;
                let mut payload = Vec::new();
                let mut channel = 1u8;
                let mut wait_secs = 5u64;
                let mut framing = None;
                let mut second_service = 0xffu8;
                let mut second_op = 0xffu8;
                let mut byte3 = 0xffu8;
                let mut i = 5;
                while i < args.len() {
                    match args[i].as_str() {
                        "--channel" if i + 1 < args.len() => {
                            channel = args[i + 1].parse()?;
                            i += 1;
                        }
                        "--wait" if i + 1 < args.len() => {
                            wait_secs = args[i + 1].parse()?;
                            i += 1;
                        }
                        "--gen3" => framing = Some(bsc200_tool::ble::FramingMode::Gen3),
                        "--gen4" => framing = Some(bsc200_tool::ble::FramingMode::Gen4),
                        "--second-service" if i + 1 < args.len() => {
                            second_service = args[i + 1].parse()?;
                            i += 1;
                        }
                        "--second-op" if i + 1 < args.len() => {
                            second_op = args[i + 1].parse()?;
                            i += 1;
                        }
                        "--byte3" if i + 1 < args.len() => {
                            byte3 = args[i + 1].parse()?;
                            i += 1;
                        }
                        "--payload" if i + 1 < args.len() => {
                            payload = (0..args[i + 1].len() / 2)
                                .map(|j| u8::from_str_radix(&args[i + 1][j * 2..j * 2 + 2], 16))
                                .collect::<Result<Vec<_>, _>>()?;
                            i += 1;
                        }
                        other => eprintln!("ignoring unknown raw option {other}"),
                    }
                    i += 1;
                }
                run_hw(HwCommand::Raw {
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
                })?;
                return Ok(());
            }
            "--dump-cnx" if args.len() == 3 => {
                let xml = std::fs::read_to_string(&args[2])?;
                let route = bsc200_tool::cnx::CnxRoute::from_xml(&xml)?;
                println!(
                    "id={} points={} distance={:.2} m ascent={} m descent={} m pois={}",
                    route.id,
                    route.points.len(),
                    route.distance_m,
                    route.ascent_m,
                    route.descent_m,
                    route.pois.len()
                );
                return Ok(());
            }
            "--fit-to-gpx" if args.len() == 4 => {
                let n = bsc200_tool::activity::fit_to_gpx(
                    &PathBuf::from(&args[2]),
                    &PathBuf::from(&args[3]),
                    "activity",
                )?;
                println!("wrote {} with {n} points", args[3]);
                return Ok(());
            }
            "--help" | "-h" => {
                println!(
                    "bsc200-tool — iGPSPORT BSC200 companion\n\n\
                    GUI:                 bsc200-tool\n\
                    Make CNX:            bsc200-tool --make-cnx route.gpx route.cnx\n\
                    Dump CNX:            bsc200-tool --dump-cnx route.cnx\n\
                    Convert FIT to GPX:  bsc200-tool --fit-to-gpx activity.fit activity.gpx\n\
                    Repair FIT order:    bsc200-tool --repair-fit activity.fit activity-fixed.fit\n\
                    Hardware test:       bsc200-tool --hw-scan\n\
                                         bsc200-tool --hw-info <addr>\n\
                                         bsc200-tool --hw-activities <addr>\n\
                                         bsc200-tool --hw-routes <addr>\n\
                                         bsc200-tool --hw-download <addr> <dir> --all [--gpx]\n\
                                         bsc200-tool --hw-upload <addr> route.gpx [--activate]\n\
                                         bsc200-tool --hw-raw <addr> <svc> <op> [--payload HEX] [--channel N] [--wait S]"
                );
                return Ok(());
            }
            _ => {}
        }
    }
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "bsc200_tool=info".into()),
        )
        .with_target(false)
        .init();

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 720.0])
            .with_min_inner_size([800.0, 500.0])
            .with_title("iGPSPORT BSC200"),
        ..Default::default()
    };
    eframe::run_native(
        "iGPSPORT BSC200",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )?;
    Ok(())
}
