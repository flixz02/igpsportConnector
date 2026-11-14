//! Native iGPSPORT `.cnx` route format (XML).
//!
//! Encoding rules (validated against native iGPSPORT files and two
//! independent reverse-engineered converters):
//!   * `<Tracks>` first point is `lat,lon,ele_mm` (lat first, elevation in mm).
//!   * Following triples: lat/lon are first/second-order deltas on a 1e-7 deg
//!     grid; elevation is a first-order delta in cm.
//!   * `<Encode>` is 2, `<Lang>` is 0.
//!   * `<Distance>` is metres with 2 decimals, ascent/descent are rounded
//!     metres (descent is negative).

use quick_xml::events::Event;
use quick_xml::Reader;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct RoutePoint {
    pub lat: f64,
    pub lon: f64,
    pub ele: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CnxPoi {
    pub lat: f64,
    pub lon: f64,
    pub kind: i32,
    pub descr: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CnxRoute {
    pub id: u32,
    pub name: String,
    pub points: Vec<RoutePoint>,
    pub pois: Vec<CnxPoi>,
    pub distance_m: f64,
    pub ascent_m: i32,
    pub descent_m: i32,
}

#[derive(Debug, Error)]
pub enum CnxError {
    #[error("route needs at least 2 points")]
    TooFewPoints,
    #[error("XML error: {0}")]
    Xml(String),
    #[error("missing or invalid field: {0}")]
    Field(String),
}

/// Haversine distance in metres.
pub fn haversine_m(a: (f64, f64), b: (f64, f64)) -> f64 {
    let r = 6_371_000.0_f64;
    let (lat1, lon1) = (a.0.to_radians(), a.1.to_radians());
    let (lat2, lon2) = (b.0.to_radians(), b.1.to_radians());
    let dlat = lat2 - lat1;
    let dlon = lon2 - lon1;
    let h = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    2.0 * r * h.sqrt().asin()
}

fn fill_elevations(points: &[RoutePoint]) -> Vec<RoutePoint> {
    let known: Vec<usize> = points
        .iter()
        .enumerate()
        .filter_map(|(i, p)| p.ele.map(|_| i))
        .collect();
    if known.is_empty() {
        return points
            .iter()
            .map(|p| RoutePoint {
                ele: Some(0.0),
                ..p.clone()
            })
            .collect();
    }
    let mut cum = vec![0.0];
    for i in 1..points.len() {
        cum.push(
            cum[i - 1]
                + haversine_m(
                    (points[i - 1].lat, points[i - 1].lon),
                    (points[i].lat, points[i].lon),
                ),
        );
    }
    let mut out = points.to_vec();
    for i in 0..out.len() {
        if out[i].ele.is_some() {
            continue;
        }
        let pos = known.partition_point(|&k| k < i);
        let a = if pos > 0 { Some(known[pos - 1]) } else { None };
        let b = if pos < known.len() {
            Some(known[pos])
        } else {
            None
        };
        out[i].ele = Some(match (a, b) {
            (Some(a), Some(b)) => {
                let span = cum[b] - cum[a];
                let f = if span == 0.0 {
                    0.0
                } else {
                    (cum[i] - cum[a]) / span
                };
                points[a].ele.unwrap() + (points[b].ele.unwrap() - points[a].ele.unwrap()) * f
            }
            (Some(a), None) => points[a].ele.unwrap(),
            (None, Some(b)) => points[b].ele.unwrap(),
            (None, None) => 0.0,
        });
    }
    out
}

fn escape_xml(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

fn content_id(tracks: &str, points: &[CnxPoi]) -> u32 {
    // Stable FNV-1a over the track and POIs, shaped as an 8-digit number.
    let mut h: u64 = 0xcbf29ce484222325;
    for byte in tracks.bytes() {
        h ^= byte as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    for p in points {
        for byte in format!("{}|{}|{}|{}", p.lat, p.lon, p.kind, p.descr).bytes() {
            h ^= byte as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
    }
    10_000_000 + (h % 90_000_000) as u32
}

impl CnxRoute {
    pub fn new(name: impl Into<String>, points: Vec<RoutePoint>) -> Result<Self, CnxError> {
        if points.len() < 2 {
            return Err(CnxError::TooFewPoints);
        }
        let points = fill_elevations(&points);
        let mut distance = 0.0;
        let mut ascent = 0.0;
        let mut descent = 0.0;
        for i in 1..points.len() {
            distance += haversine_m(
                (points[i - 1].lat, points[i - 1].lon),
                (points[i].lat, points[i].lon),
            );
            let d = points[i].ele.unwrap_or(0.0) - points[i - 1].ele.unwrap_or(0.0);
            if d > 0.0 {
                ascent += d;
            } else {
                descent += d;
            }
        }
        let ascent_m = ascent.round() as i32;
        let descent_m = descent.round() as i32;
        let mut route = Self {
            id: 0,
            name: name.into(),
            points,
            pois: Vec::new(),
            distance_m: distance,
            ascent_m,
            descent_m,
        };
        route.id = content_id(&route.encode_tracks(), &route.pois);
        Ok(route)
    }

    pub fn with_pois(mut self, pois: Vec<CnxPoi>) -> Self {
        self.pois = pois;
        self.id = content_id(&self.encode_tracks(), &self.pois);
        self
    }

    pub fn encode_tracks(&self) -> String {
        let n = self.points.len();
        let lat: Vec<i64> = self
            .points
            .iter()
            .map(|p| (p.lat * 1e7).round() as i64)
            .collect();
        let lon: Vec<i64> = self
            .points
            .iter()
            .map(|p| (p.lon * 1e7).round() as i64)
            .collect();
        let ele_cm: Vec<i64> = self
            .points
            .iter()
            .map(|p| (p.ele.unwrap_or(0.0) * 100.0).round() as i64)
            .collect();
        let first_ele_mm = (self.points[0].ele.unwrap_or(0.0) * 1000.0).round() as i64;

        let mut parts = Vec::with_capacity(n);
        parts.push(format!(
            "{:.7},{:.7},{}",
            self.points[0].lat, self.points[0].lon, first_ele_mm
        ));

        let mut prev_dlat = 0i64;
        let mut prev_dlon = 0i64;
        for i in 1..n {
            let dlat = lat[i] - lat[i - 1];
            let dlon = lon[i] - lon[i - 1];
            let tlat = dlat - prev_dlat;
            let tlon = dlon - prev_dlon;
            let tele = ele_cm[i] - ele_cm[i - 1];
            parts.push(format!("{tlat},{tlon},{tele}"));
            prev_dlat = dlat;
            prev_dlon = dlon;
        }
        parts.join(";") + ";"
    }

    pub fn decode_tracks(tracks: &str) -> Result<Vec<RoutePoint>, CnxError> {
        let toks: Vec<&str> = tracks.split(';').filter(|s| !s.trim().is_empty()).collect();
        if toks.is_empty() {
            return Err(CnxError::Field("Tracks".into()));
        }
        let first: Vec<&str> = toks[0].split(',').collect();
        if first.len() != 3 {
            return Err(CnxError::Field("first track token".into()));
        }
        let mut lat: f64 = first[0]
            .parse()
            .map_err(|_| CnxError::Field("lat".into()))?;
        let mut lon: f64 = first[1]
            .parse()
            .map_err(|_| CnxError::Field("lon".into()))?;
        let mut ele_mm: f64 = first[2]
            .parse()
            .map_err(|_| CnxError::Field("ele".into()))?;
        let mut vlat = 0.0f64;
        let mut vlon = 0.0f64;
        let mut out = vec![RoutePoint {
            lat,
            lon,
            ele: Some(ele_mm / 1000.0),
        }];
        for tok in &toks[1..] {
            let p: Vec<&str> = tok.split(',').collect();
            if p.len() != 3 {
                return Err(CnxError::Field("track delta".into()));
            }
            let dlat: f64 = p[0].parse().map_err(|_| CnxError::Field("dlat".into()))?;
            let dlon: f64 = p[1].parse().map_err(|_| CnxError::Field("dlon".into()))?;
            let dele: f64 = p[2].parse().map_err(|_| CnxError::Field("dele".into()))?;
            vlat += dlat;
            vlon += dlon;
            lat += vlat / 1e7;
            lon += vlon / 1e7;
            ele_mm += dele * 10.0;
            out.push(RoutePoint {
                lat,
                lon,
                ele: Some(ele_mm / 1000.0),
            });
        }
        Ok(out)
    }

    pub fn to_xml(&self) -> String {
        let tracks = self.encode_tracks();
        let mut points_xml = String::new();
        for p in &self.pois {
            points_xml.push_str(&format!(
                "<Point><Lat>{:.7}</Lat><Lng>{:.7}</Lng><Type>{}</Type><Descr>{}</Descr></Point>",
                p.lat,
                p.lon,
                p.kind,
                escape_xml(&p.descr)
            ));
        }
        format!(
            "<?xml version='1.0' encoding='UTF-8' standalone='yes' ?>\n\
             <Route><Id>{}</Id><Distance>{:.2}</Distance><Duration></Duration>\
             <Ascent>{}</Ascent><Descent>{}</Descent><Encode>2</Encode><Lang>0</Lang>\
             <TracksCount>{}</TracksCount><Tracks>{}</Tracks><Navs/>\
             <Points>{}</Points><PointsCount>{}</PointsCount></Route>\n",
            self.id,
            self.distance_m,
            self.ascent_m,
            self.descent_m,
            self.points.len(),
            tracks,
            points_xml,
            self.pois.len()
        )
    }

    pub fn from_xml(xml: &str) -> Result<Self, CnxError> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(false);
        let mut buf = Vec::new();
        let mut id = None;
        let mut distance = 0.0;
        let mut ascent = 0;
        let mut descent = 0;
        let mut tracks = String::new();
        let mut pois = Vec::new();
        let mut current_tag = String::new();
        let mut text_buf = String::new();
        let mut point: Option<CnxPoi> = None;
        let mut point_tag = String::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    text_buf.clear();
                    if tag == "Point" {
                        point = Some(CnxPoi {
                            lat: 0.0,
                            lon: 0.0,
                            kind: 0,
                            descr: String::new(),
                        });
                    } else if point.is_some() {
                        point_tag = tag;
                    } else {
                        current_tag = tag;
                    }
                }
                Ok(Event::Text(e)) => {
                    if let Ok(text) = e.decode() {
                        text_buf.push_str(&text);
                    }
                }
                Ok(Event::GeneralRef(e)) => {
                    if let Ok(name) = e.decode() {
                        match name.as_ref() {
                            "amp" => text_buf.push('&'),
                            "lt" => text_buf.push('<'),
                            "gt" => text_buf.push('>'),
                            "quot" => text_buf.push('"'),
                            "apos" => text_buf.push('\''),
                            _ => {
                                if let Some(hex) =
                                    name.strip_prefix("#x").or_else(|| name.strip_prefix("#X"))
                                {
                                    if let Ok(v) = u32::from_str_radix(hex, 16) {
                                        if let Some(c) = char::from_u32(v) {
                                            text_buf.push(c);
                                        }
                                    }
                                } else if let Some(dec) = name.strip_prefix('#') {
                                    if let Ok(v) = dec.parse::<u32>() {
                                        if let Some(c) = char::from_u32(v) {
                                            text_buf.push(c);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(Event::End(e)) => {
                    let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    if tag == "Point" {
                        if let Some(p) = point.take() {
                            pois.push(p);
                        }
                    } else if let Some(p) = point.as_mut() {
                        match point_tag.as_str() {
                            "Lat" => p.lat = text_buf.trim().parse().unwrap_or(0.0),
                            "Lng" => p.lon = text_buf.trim().parse().unwrap_or(0.0),
                            "Type" => p.kind = text_buf.trim().parse().unwrap_or(0),
                            "Descr" => p.descr = text_buf.clone(),
                            _ => {}
                        }
                        point_tag.clear();
                    } else {
                        match current_tag.as_str() {
                            "Id" => id = text_buf.trim().parse().ok(),
                            "Distance" => distance = text_buf.trim().parse().unwrap_or(0.0),
                            "Ascent" => ascent = text_buf.trim().parse().unwrap_or(0),
                            "Descent" => descent = text_buf.trim().parse().unwrap_or(0),
                            "Tracks" => tracks = text_buf.clone(),
                            _ => {}
                        }
                        current_tag.clear();
                    }
                    text_buf.clear();
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(CnxError::Xml(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        let points = Self::decode_tracks(&tracks)?;
        let mut route = Self {
            id: id.unwrap_or(0),
            name: String::new(),
            points,
            pois,
            distance_m: distance,
            ascent_m: ascent,
            descent_m: descent,
        };
        if route.id == 0 {
            route.id = content_id(&route.encode_tracks(), &route.pois);
        }
        Ok(route)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_points() -> Vec<RoutePoint> {
        vec![
            RoutePoint {
                lat: 45.4460729,
                lon: 10.9774572,
                ele: Some(120.25),
            },
            RoutePoint {
                lat: 45.4461729,
                lon: 10.9775572,
                ele: Some(121.75),
            },
            RoutePoint {
                lat: 45.4462729,
                lon: 10.9776572,
                ele: Some(121.20),
            },
            RoutePoint {
                lat: 45.4463729,
                lon: 10.9777572,
                ele: Some(122.80),
            },
        ]
    }

    #[test]
    fn roundtrip_precision() {
        let r = CnxRoute::new("Test", sample_points()).unwrap();
        let xml = r.to_xml();
        let d = CnxRoute::from_xml(&xml).unwrap();
        assert_eq!(d.points.len(), r.points.len());
        for (a, b) in r.points.iter().zip(d.points.iter()) {
            assert!((a.lat - b.lat).abs() < 1e-7);
            assert!((a.lon - b.lon).abs() < 1e-7);
            assert!((a.ele.unwrap() - b.ele.unwrap()).abs() < 0.02);
        }
        assert_eq!(d.ascent_m, r.ascent_m);
        assert_eq!(d.descent_m, r.descent_m);
    }

    #[test]
    fn missing_elevation_is_interpolated() {
        let pts = vec![
            RoutePoint {
                lat: 45.0,
                lon: 10.0,
                ele: None,
            },
            RoutePoint {
                lat: 45.001,
                lon: 10.001,
                ele: Some(110.0),
            },
            RoutePoint {
                lat: 45.002,
                lon: 10.002,
                ele: None,
            },
        ];
        let r = CnxRoute::new("Test", pts).unwrap();
        assert_eq!(r.points[0].ele, Some(110.0));
        assert_eq!(r.points[2].ele, Some(110.0));
    }

    #[test]
    fn poi_roundtrip() {
        let r = CnxRoute::new("Test", sample_points())
            .unwrap()
            .with_pois(vec![CnxPoi {
                lat: 45.4461,
                lon: 10.9775,
                kind: 7,
                descr: "Water & food".into(),
            }]);
        let d = CnxRoute::from_xml(&r.to_xml()).unwrap();
        assert_eq!(d.pois.len(), 1);
        assert_eq!(d.pois[0].kind, 7);
        assert_eq!(d.pois[0].descr, "Water & food");
    }

    #[test]
    fn id_is_eight_digits() {
        let r = CnxRoute::new("Test", sample_points()).unwrap();
        assert!((10_000_000..100_000_000).contains(&r.id));
    }

    #[test]
    fn first_token_is_lat_lon_mm() {
        let r = CnxRoute::new("Test", sample_points()).unwrap();
        let tracks = r.encode_tracks();
        let first = tracks.split(';').next().unwrap();
        let parts: Vec<&str> = first.split(',').collect();
        assert_eq!(parts.len(), 3);
        assert!(parts[0].starts_with("45.4460729"));
        assert!(parts[1].starts_with("10.9774572"));
        assert_eq!(parts[2], "120250");
    }

    #[test]
    fn native_track_encoding() {
        let r = CnxRoute::new("Test", sample_points()[..3].to_vec()).unwrap();
        assert_eq!(
            r.encode_tracks(),
            "45.4460729,10.9774572,120250;1000,1000,150;0,0,-55;"
        );
    }

    #[test]
    fn xml_contains_native_fields() {
        let r = CnxRoute::new("Test & Ride", sample_points()).unwrap();
        let xml = r.to_xml();
        assert!(xml.contains("<Encode>2</Encode>"));
        assert!(xml.contains("<Lang>0</Lang>"));
        assert!(xml.contains("<TracksCount>4</TracksCount>"));
        assert!(xml.contains("<Navs/>"));
        assert!(xml.contains("<PointsCount>0</PointsCount>"));
    }
}
