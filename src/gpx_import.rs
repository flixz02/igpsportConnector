//! GPX import for route creation.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::cnx::{CnxPoi, RoutePoint};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GpxImportError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("GPX parse error: {0}")]
    Parse(#[from] gpx::errors::GpxError),
    #[error("GPX has no track or route points")]
    Empty,
}

#[derive(Debug, Clone)]
pub struct ImportedRoute {
    pub name: String,
    pub points: Vec<RoutePoint>,
    pub pois: Vec<CnxPoi>,
}

pub fn load_gpx(path: &Path) -> Result<ImportedRoute, GpxImportError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let fallback_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Route")
        .to_string();
    parse_gpx(reader, fallback_name)
}

pub fn parse_gpx<R: std::io::BufRead>(
    reader: R,
    fallback_name: String,
) -> Result<ImportedRoute, GpxImportError> {
    let gpx: gpx::Gpx = gpx::read(reader)?;

    let mut name = fallback_name.clone();
    let mut points = Vec::new();

    for track in &gpx.tracks {
        if name == fallback_name {
            if let Some(n) = &track.name {
                if !n.trim().is_empty() {
                    name = n.clone();
                }
            }
        }
        for seg in &track.segments {
            for pt in &seg.points {
                points.push(RoutePoint {
                    lat: pt.point().y(),
                    lon: pt.point().x(),
                    ele: pt.elevation,
                });
            }
        }
    }

    if points.is_empty() {
        if let Some(route) = gpx.routes.first() {
            if let Some(n) = &route.name {
                if !n.trim().is_empty() {
                    name = n.clone();
                }
            }
            for pt in &route.points {
                points.push(RoutePoint {
                    lat: pt.point().y(),
                    lon: pt.point().x(),
                    ele: pt.elevation,
                });
            }
        }
    }

    if points.is_empty() {
        return Err(GpxImportError::Empty);
    }

    let pois = gpx
        .waypoints
        .iter()
        .map(|w| CnxPoi {
            lat: w.point().y(),
            lon: w.point().x(),
            kind: 0,
            descr: w.name.clone().unwrap_or_default(),
        })
        .filter(|p| !p.descr.is_empty())
        .collect();

    Ok(ImportedRoute { name, points, pois })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const GPX: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<gpx version="1.1" creator="test" xmlns="http://www.topografix.com/GPX/1/1">
  <trk>
    <name>Test Track</name>
    <trkseg>
      <trkpt lat="45.0" lon="10.0"><ele>100.0</ele></trkpt>
      <trkpt lat="45.001" lon="10.001"><ele>101.0</ele></trkpt>
    </trkseg>
  </trk>
  <wpt lat="45.0005" lon="10.0005"><name>Water</name></wpt>
</gpx>"#;

    #[test]
    fn reads_track_and_waypoint() {
        let route = parse_gpx(Cursor::new(GPX.as_bytes()), "fallback".into()).unwrap();
        assert_eq!(route.name, "Test Track");
        assert_eq!(route.points.len(), 2);
        assert_eq!(route.points[0].lat, 45.0);
        assert_eq!(route.points[0].ele, Some(100.0));
        assert_eq!(route.pois.len(), 1);
        assert_eq!(route.pois[0].descr, "Water");
    }
}
