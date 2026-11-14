//! Activity export: save raw FIT and optionally convert to GPX.

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use chrono::{DateTime, TimeZone, Utc};
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use thiserror::Error;

use crate::fit::{parse_records, FitError, Record as FitRecord, FIT_EPOCH_OFFSET};

const GPX_NAMESPACE: &str = "http://www.topografix.com/GPX/1/1";
const GPXTPX_NAMESPACE: &str = "http://www.garmin.com/xmlschemas/TrackPointExtension/v2";
const GPXX_NAMESPACE: &str = "http://www.garmin.com/xmlschemas/ActivityExtension/v2";
const BSC200_NAMESPACE: &str = "urn:bsc200-tool:extensions:v1";

#[derive(Debug, Error)]
pub enum ExportError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("FIT parse error: {0}")]
    Fit(#[from] FitError),
    #[error("GPX write error: {0}")]
    Xml(#[from] quick_xml::Error),
    #[error("no GPS records in FIT file")]
    NoRecords,
}

/// Save a raw activity FIT file. Returns the path written.
pub fn save_fit(bytes: &[u8], dir: &Path, stem: &str) -> Result<PathBuf, ExportError> {
    std::fs::create_dir_all(dir)?;
    let path = dir.join(format!("{stem}.fit"));
    let mut f = File::create(&path)?;
    f.write_all(bytes)?;
    Ok(path)
}

/// Convert a FIT activity file to GPX 1.1. Returns the number of track points.
pub fn fit_to_gpx(fit_path: &Path, gpx_path: &Path, title: &str) -> Result<usize, ExportError> {
    let bytes = std::fs::read(fit_path)?;
    crate::fit::validate(&bytes)?;
    let points = parse_records(&bytes)?;
    write_gpx(&points, gpx_path, title)
}

fn write_gpx(points: &[FitRecord], gpx_path: &Path, title: &str) -> Result<usize, ExportError> {
    if points.is_empty() {
        return Err(ExportError::NoRecords);
    }
    if let Some(parent) = gpx_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut writer = Writer::new_with_indent(BufWriter::new(File::create(gpx_path)?), b' ', 2);
    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
    writer.write_event(Event::Start(gpx_root()))?;
    writer.write_event(Event::Start(BytesStart::new("metadata")))?;
    if let Some(first) = points.first().and_then(|p| p.time) {
        write_text_element(&mut writer, "time", &gpx_time(first))?;
    }
    writer.write_event(Event::End(BytesEnd::new("metadata")))?;

    writer.write_event(Event::Start(BytesStart::new("trk")))?;
    write_text_element(&mut writer, "name", title)?;
    writer.write_event(Event::Start(BytesStart::new("trkseg")))?;
    for point in points {
        write_track_point(&mut writer, point)?;
    }
    writer.write_event(Event::End(BytesEnd::new("trkseg")))?;
    writer.write_event(Event::End(BytesEnd::new("trk")))?;
    writer.write_event(Event::End(BytesEnd::new("gpx")))?;
    writer.into_inner().flush()?;
    Ok(points.len())
}

fn gpx_root() -> BytesStart<'static> {
    let mut root = BytesStart::new("gpx");
    root.push_attribute(("version", "1.1"));
    root.push_attribute(("creator", "bsc200-tool"));
    root.push_attribute(("xmlns", GPX_NAMESPACE));
    root.push_attribute(("xmlns:gpxtpx", GPXTPX_NAMESPACE));
    root.push_attribute(("xmlns:gpxx", GPXX_NAMESPACE));
    root.push_attribute(("xmlns:bsc200", BSC200_NAMESPACE));
    root
}

fn write_track_point<W: Write>(
    writer: &mut Writer<W>,
    point: &FitRecord,
) -> Result<(), quick_xml::Error> {
    let lat = format!("{:.7}", point.lat.unwrap_or_default());
    let lon = format!("{:.7}", point.lon.unwrap_or_default());
    let mut trkpt = BytesStart::new("trkpt");
    trkpt.push_attribute(("lat", lat.as_str()));
    trkpt.push_attribute(("lon", lon.as_str()));
    writer.write_event(Event::Start(trkpt))?;

    if let Some(ele) = point.ele {
        write_text_element(writer, "ele", &format!("{ele:.3}"))?;
    }
    if let Some(time) = point.time {
        write_text_element(writer, "time", &gpx_time(time))?;
    }

    let has_track_point_extension = point.temperature.is_some()
        || point.hr.is_some()
        || point.cadence.is_some()
        || point.speed.is_some();
    let has_activity_extension = point.power.is_some();
    let has_bsc200_extension = point.distance.is_some();

    if has_track_point_extension || has_activity_extension || has_bsc200_extension {
        writer.write_event(Event::Start(BytesStart::new("extensions")))?;
        if has_track_point_extension {
            writer.write_event(Event::Start(BytesStart::new("gpxtpx:TrackPointExtension")))?;
            if let Some(temp) = point.temperature {
                write_text_element(writer, "gpxtpx:atemp", &format!("{temp:.1}"))?;
            }
            if let Some(hr) = point.hr {
                write_text_element(writer, "gpxtpx:hr", &hr.to_string())?;
            }
            if let Some(cadence) = point.cadence {
                write_text_element(writer, "gpxtpx:cad", &cadence.to_string())?;
            }
            if let Some(speed) = point.speed {
                write_text_element(writer, "gpxtpx:speed", &format!("{speed:.3}"))?;
            }
            writer.write_event(Event::End(BytesEnd::new("gpxtpx:TrackPointExtension")))?;
        }
        if let Some(power) = point.power {
            writer.write_event(Event::Start(BytesStart::new("gpxx:TPX")))?;
            write_text_element(writer, "gpxx:Watts", &power.to_string())?;
            writer.write_event(Event::End(BytesEnd::new("gpxx:TPX")))?;
        }
        if let Some(distance) = point.distance {
            write_text_element(writer, "bsc200:distance", &format!("{distance:.2}"))?;
        }
        writer.write_event(Event::End(BytesEnd::new("extensions")))?;
    }

    writer.write_event(Event::End(BytesEnd::new("trkpt")))?;
    Ok(())
}

fn write_text_element<W: Write>(
    writer: &mut Writer<W>,
    name: &str,
    text: &str,
) -> Result<(), quick_xml::Error> {
    writer.write_event(Event::Start(BytesStart::new(name)))?;
    writer.write_event(Event::Text(BytesText::new(text)))?;
    writer.write_event(Event::End(BytesEnd::new(name)))?;
    Ok(())
}

fn gpx_time(time: DateTime<chrono::Local>) -> String {
    time.with_timezone(&Utc)
        .format("%Y-%m-%dT%H:%M:%SZ")
        .to_string()
}

/// Derive a filename stem from the device activity timestamp.
/// Device timestamps are seconds since the FIT epoch (1989-12-31).
pub fn activity_stem(timestamp: i32) -> String {
    // The device stores local wall-clock time as FIT-epoch seconds, so format
    // the resulting instant as UTC to recover the time shown on the device.
    let unix = timestamp as i64 + FIT_EPOCH_OFFSET;
    match Utc.timestamp_opt(unix, 0).single() {
        Some(dt) => dt.format("%Y-%m-%d_%H-%M-%S").to_string(),
        None => format!("activity_{timestamp}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn writes_readable_gpx() {
        let pts = vec![
            FitRecord {
                lat: Some(45.0),
                lon: Some(10.0),
                ele: Some(100.0),
                time: Some(
                    Utc.timestamp_opt(1_700_000_000, 0)
                        .unwrap()
                        .with_timezone(&chrono::Local),
                ),
                speed: Some(5.0),
                hr: Some(142),
                cadence: Some(88),
                power: Some(210),
                distance: Some(1234.56),
                temperature: Some(19.5),
            },
            FitRecord {
                lat: Some(45.001),
                lon: Some(10.001),
                ele: Some(101.0),
                time: Some(
                    Utc.timestamp_opt(1_700_000_005, 0)
                        .unwrap()
                        .with_timezone(&chrono::Local),
                ),
                speed: None,
                ..Default::default()
            },
        ];
        let path = std::env::temp_dir().join("bsc200_tool_test.gpx");
        let n = write_gpx(&pts, &path, "Test").unwrap();
        assert_eq!(n, 2);
        let f = File::open(&path).unwrap();
        let gpx = gpx::read(BufReader::new(f)).unwrap();
        assert_eq!(gpx.tracks.len(), 1);
        assert_eq!(gpx.tracks[0].segments[0].points.len(), 2);
        assert!((gpx.tracks[0].segments[0].points[0].point().y() - 45.0).abs() < 1e-9);
        let xml = std::fs::read_to_string(&path).unwrap();
        assert!(xml.contains("<gpxtpx:hr>142</gpxtpx:hr>"));
        assert!(xml.contains("<gpxtpx:cad>88</gpxtpx:cad>"));
        assert!(xml.contains("<gpxtpx:speed>5.000</gpxtpx:speed>"));
        assert!(xml.contains("<gpxtpx:atemp>19.5</gpxtpx:atemp>"));
        assert!(xml.contains("<gpxx:Watts>210</gpxx:Watts>"));
        assert!(xml.contains("<bsc200:distance>1234.56</bsc200:distance>"));
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn stem_uses_fit_epoch() {
        // Raw FIT timestamp from a real BSC200 activity start.
        assert_eq!(activity_stem(1_159_893_328), "2026-10-02_16-35-28");
        assert!(!activity_stem(1_159_893_328).contains('/'));
    }
}
