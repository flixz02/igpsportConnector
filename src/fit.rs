//! Minimal FIT activity parser.
//!
//! The BSC200 writes standard FIT files. We only need the `record` messages
//! (global message 20) to build GPX tracks, so this parser implements just the
//! parts of the FIT protocol required for that: file header validation,
//! definition messages, data messages, compressed timestamp headers and the
//! field base types used by the device.

use chrono::{DateTime, Local, TimeZone};
use thiserror::Error;

/// Seconds between the Unix epoch and the FIT epoch (1989-12-31T00:00:00Z).
pub const FIT_EPOCH_OFFSET: i64 = 631_065_600;

const RECORD_MESSAGE: u16 = 20;
const FIELD_TIMESTAMP: u8 = 253;

#[derive(Debug, Error)]
pub enum FitError {
    #[error("file is too short to be a FIT file")]
    TooShort,
    #[error("missing .FIT signature")]
    BadSignature,
    #[error("truncated FIT data at offset {0}")]
    Truncated(usize),
    #[error("invalid FIT definition at offset {0}")]
    BadDefinition(usize),
    #[error("data message references undefined local type {0}")]
    UnknownLocalType(u8),
    #[error("FIT header CRC mismatch")]
    BadHeaderCrc,
    #[error("FIT file CRC mismatch")]
    BadCrc,
}

/// One GPS track point decoded from a FIT `record` message.
#[derive(Debug, Default, Clone)]
pub struct Record {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub ele: Option<f64>,
    pub time: Option<DateTime<Local>>,
    pub speed: Option<f64>,
    pub distance: Option<f64>,
    pub hr: Option<u8>,
    pub cadence: Option<u8>,
    pub power: Option<u16>,
    /// Temperature in degrees Celsius (FIT record field 13).
    pub temperature: Option<f64>,
}

#[derive(Debug, Clone)]
struct FieldDef {
    number: u8,
    size: u8,
    base_type: u8,
}

#[derive(Debug, Clone)]
struct Definition {
    big_endian: bool,
    global: u16,
    fields: Vec<FieldDef>,
    /// Sizes of developer fields; their values are not used.
    developer_field_sizes: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    Invalid,
}

impl Value {
    fn unsigned(self) -> Option<u64> {
        match self {
            Value::Unsigned(v) => Some(v),
            Value::Signed(v) if v >= 0 => Some(v as u64),
            _ => None,
        }
    }

    fn signed(self) -> Option<i64> {
        match self {
            Value::Signed(v) => Some(v),
            Value::Unsigned(v) if v <= i64::MAX as u64 => Some(v as i64),
            _ => None,
        }
    }

    fn float(self) -> Option<f64> {
        match self {
            Value::Signed(v) => Some(v as f64),
            Value::Unsigned(v) => Some(v as f64),
            Value::Float(v) => Some(v),
            Value::Invalid => None,
        }
    }
}

/// Validate the FIT header and trailing CRC.
///
/// Some devices omit the optional header CRC (header size 12); the trailing
/// file CRC is always checked.
pub fn validate(bytes: &[u8]) -> Result<(), FitError> {
    if bytes.len() < 14 {
        return Err(FitError::TooShort);
    }
    let header_size = bytes[0] as usize;
    if header_size < 12 || header_size > bytes.len() {
        return Err(FitError::TooShort);
    }
    if &bytes[8..12] != b".FIT" {
        return Err(FitError::BadSignature);
    }
    if header_size >= 14 {
        let stored = u16::from_le_bytes([bytes[12], bytes[13]]);
        // A zero header CRC means "not set" in some third-party writers.
        if stored != 0 && crc16(&bytes[..12]) != stored {
            return Err(FitError::BadHeaderCrc);
        }
    }
    let data_size = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as usize;
    let end = header_size
        .checked_add(data_size)
        .filter(|&end| end + 2 <= bytes.len())
        .ok_or(FitError::Truncated(header_size))?;
    let stored = u16::from_le_bytes([bytes[end], bytes[end + 1]]);
    if crc16(&bytes[..end]) != stored {
        return Err(FitError::BadCrc);
    }
    Ok(())
}

fn crc16(data: &[u8]) -> u16 {
    // CRC-16/ARC as used by the FIT protocol.
    const TABLE: [u16; 16] = [
        0x0000, 0xCC01, 0xD801, 0x1400, 0xF001, 0x3C00, 0x2800, 0xE401, 0xA001, 0x6C00, 0x7800,
        0xB401, 0x5000, 0x9C01, 0x8801, 0x4400,
    ];
    let mut crc = 0u16;
    for &byte in data {
        let mut tmp = TABLE[(crc & 0xF) as usize];
        crc = (crc >> 4) & 0x0FFF;
        crc = crc ^ tmp ^ TABLE[(byte & 0xF) as usize];
        tmp = TABLE[(crc & 0xF) as usize];
        crc = (crc >> 4) & 0x0FFF;
        crc = crc ^ tmp ^ TABLE[(byte >> 4) as usize];
    }
    crc
}

/// Byte range and identity of one FIT definition or data message.
#[derive(Debug, Clone, Copy)]
struct MessageSpan {
    start: usize,
    end: usize,
    is_definition: bool,
    global: u16,
    local: u8,
}

/// Walk the FIT data section and record every message boundary without
/// decoding field values.
fn scan_messages(data: &[u8]) -> Result<Vec<MessageSpan>, FitError> {
    let mut definitions: Vec<Option<Definition>> = vec![None; 16];
    let mut messages = Vec::new();
    let mut pos = 0usize;

    while pos < data.len() {
        let start = pos;
        let header = data[pos];
        pos += 1;

        if header & 0x80 != 0 {
            // Compressed timestamp data message.
            let local = ((header >> 5) & 0x03) as usize;
            let definition = definitions[local]
                .as_ref()
                .ok_or(FitError::UnknownLocalType(local as u8))?
                .clone();
            for field in &definition.fields {
                if field.number == FIELD_TIMESTAMP {
                    continue;
                }
                read_bytes(data, &mut pos, field.size as usize)?;
            }
            pos = skip_developer_fields(data, pos, &definition)?;
            messages.push(MessageSpan {
                start,
                end: pos,
                is_definition: false,
                global: definition.global,
                local: local as u8,
            });
        } else if header & 0x40 != 0 {
            // Definition message.
            let local = (header & 0x0F) as usize;
            let has_developer_fields = header & 0x20 != 0;
            let _reserved = read_bytes(data, &mut pos, 1)?;
            let arch = read_bytes(data, &mut pos, 1)?[0];
            if arch > 1 {
                return Err(FitError::BadDefinition(start));
            }
            let big_endian = arch == 1;
            let global = read_u16(data, &mut pos, big_endian)?;
            let count = read_bytes(data, &mut pos, 1)?[0];
            let mut fields = Vec::with_capacity(count as usize);
            for _ in 0..count {
                let descriptor = read_bytes(data, &mut pos, 3)?;
                fields.push(FieldDef {
                    number: descriptor[0],
                    size: descriptor[1],
                    base_type: descriptor[2],
                });
            }
            let mut developer_field_sizes = Vec::new();
            if has_developer_fields {
                let count = read_bytes(data, &mut pos, 1)?[0];
                for _ in 0..count {
                    let descriptor = read_bytes(data, &mut pos, 3)?;
                    developer_field_sizes.push(descriptor[1]);
                }
            }
            definitions[local] = Some(Definition {
                big_endian,
                global,
                fields,
                developer_field_sizes,
            });
            messages.push(MessageSpan {
                start,
                end: pos,
                is_definition: true,
                global,
                local: local as u8,
            });
        } else {
            // Data message.
            let local = (header & 0x0F) as usize;
            let definition = definitions[local]
                .as_ref()
                .ok_or(FitError::UnknownLocalType(local as u8))?
                .clone();
            for field in &definition.fields {
                read_bytes(data, &mut pos, field.size as usize)?;
            }
            pos = skip_developer_fields(data, pos, &definition)?;
            messages.push(MessageSpan {
                start,
                end: pos,
                is_definition: false,
                global: definition.global,
                local: local as u8,
            });
        }
    }

    Ok(messages)
}

/// Ensure the FIT `activity` message is the last message and that its
/// `num_sessions` field matches the number of session messages.
///
/// BSC200 firmware appends the `sport` message after the `activity` message
/// and writes `num_sessions = 0`. The FIT specification requires `activity`
/// to be last and `num_sessions` to match the session count, and strict
/// viewers reject the file. This moves any trailing messages before the
/// activity message, re-emits the activity definition, patches
/// `num_sessions`, and recomputes both CRCs.
///
/// Returns `(bytes, repaired)`.
pub fn normalize(bytes: &[u8]) -> Result<(Vec<u8>, bool), FitError> {
    if bytes.len() < 14 {
        return Err(FitError::TooShort);
    }
    let header_size = bytes[0] as usize;
    if header_size < 12 || header_size > bytes.len() {
        return Err(FitError::TooShort);
    }
    if &bytes[8..12] != b".FIT" {
        return Err(FitError::BadSignature);
    }
    let data_size = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as usize;
    let data_end = header_size
        .checked_add(data_size)
        .filter(|&end| end + 2 <= bytes.len())
        .ok_or(FitError::Truncated(header_size))?;
    let data = &bytes[header_size..data_end];
    let messages = scan_messages(data)?;

    // FIT message 34 is `activity`; it must be the final message.
    let Some(activity_index) = messages
        .iter()
        .rposition(|message| !message.is_definition && message.global == 34)
    else {
        return Ok((bytes.to_vec(), false));
    };
    let activity = messages[activity_index];
    let trailing = activity_index + 1 < messages.len();
    let activity_definition = messages[..activity_index]
        .iter()
        .rev()
        .find(|message| message.is_definition && message.local == activity.local);

    // The BSC200 writes num_sessions = 0 even though the file contains a
    // session message. Patch it to the actual session count.
    let mut activity_bytes = data[activity.start..activity.end].to_vec();
    let mut patched = false;
    if let Some(definition_span) = activity_definition {
        let definition = parse_definition(data, *definition_span)?;
        let session_count = messages
            .iter()
            .filter(|message| !message.is_definition && message.global == 18)
            .count() as u16;
        if session_count > 0 {
            let compressed = activity_bytes[0] & 0x80 != 0;
            patched = patch_num_sessions(
                &mut activity_bytes,
                &definition.fields,
                definition.big_endian,
                compressed,
                session_count,
            );
        }
    }

    if !trailing && !patched {
        return Ok((bytes.to_vec(), false));
    }

    let tail = if trailing {
        &data[activity.end..]
    } else {
        &[][..]
    };
    let mut new_data =
        Vec::with_capacity(data.len() + activity_definition.map_or(0, |d| d.end - d.start));
    new_data.extend_from_slice(&data[..activity.start]);
    new_data.extend_from_slice(tail);
    if trailing {
        if let Some(definition) = activity_definition {
            // Re-emit the definition in case the moved tail redefined the
            // local message type used by the activity message.
            new_data.extend_from_slice(&data[definition.start..definition.end]);
        }
    }
    new_data.extend_from_slice(&activity_bytes);

    let trailing_bytes = &bytes[data_end + 2..];
    let mut out = Vec::with_capacity(header_size + new_data.len() + 2 + trailing_bytes.len());
    out.extend_from_slice(&bytes[..header_size]);
    out[4..8].copy_from_slice(&(new_data.len() as u32).to_le_bytes());
    if header_size >= 14 {
        let header_crc = crc16(&out[..12]);
        out[12..14].copy_from_slice(&header_crc.to_le_bytes());
    }
    out.extend_from_slice(&new_data);
    let file_crc = crc16(&out);
    out.extend_from_slice(&file_crc.to_le_bytes());
    out.extend_from_slice(trailing_bytes);
    Ok((out, true))
}

fn parse_definition(data: &[u8], span: MessageSpan) -> Result<Definition, FitError> {
    let mut pos = span.start;
    let header = data[pos];
    pos += 1;
    let has_developer_fields = header & 0x20 != 0;
    let _reserved = read_bytes(data, &mut pos, 1)?;
    let arch = read_bytes(data, &mut pos, 1)?[0];
    if arch > 1 {
        return Err(FitError::BadDefinition(span.start));
    }
    let big_endian = arch == 1;
    let global = read_u16(data, &mut pos, big_endian)?;
    let count = read_bytes(data, &mut pos, 1)?[0];
    let mut fields = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let descriptor = read_bytes(data, &mut pos, 3)?;
        fields.push(FieldDef {
            number: descriptor[0],
            size: descriptor[1],
            base_type: descriptor[2],
        });
    }
    let mut developer_field_sizes = Vec::new();
    if has_developer_fields {
        let count = read_bytes(data, &mut pos, 1)?[0];
        for _ in 0..count {
            let descriptor = read_bytes(data, &mut pos, 3)?;
            developer_field_sizes.push(descriptor[1]);
        }
    }
    Ok(Definition {
        big_endian,
        global,
        fields,
        developer_field_sizes,
    })
}

fn patch_num_sessions(
    message: &mut [u8],
    fields: &[FieldDef],
    big_endian: bool,
    compressed_timestamp: bool,
    session_count: u16,
) -> bool {
    let mut offset = 1usize;
    for field in fields {
        if compressed_timestamp && field.number == FIELD_TIMESTAMP {
            continue;
        }
        if field.number == 1 {
            return match field.size {
                1 => {
                    let value = session_count.min(u8::MAX as u16) as u8;
                    if message[offset] != value {
                        message[offset] = value;
                        true
                    } else {
                        false
                    }
                }
                2 => {
                    let value = if big_endian {
                        session_count.to_be_bytes()
                    } else {
                        session_count.to_le_bytes()
                    };
                    if message[offset..offset + 2] != value {
                        message[offset..offset + 2].copy_from_slice(&value);
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            };
        }
        offset += field.size as usize;
    }
    false
}

/// Parse all GPS `record` messages in a FIT file.
pub fn parse_records(bytes: &[u8]) -> Result<Vec<Record>, FitError> {
    if bytes.len() < 12 {
        return Err(FitError::TooShort);
    }
    let header_size = bytes[0] as usize;
    if header_size < 12 || header_size > bytes.len() {
        return Err(FitError::TooShort);
    }
    if &bytes[8..12] != b".FIT" {
        return Err(FitError::BadSignature);
    }

    let data_size = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as usize;
    let end = header_size
        .checked_add(data_size)
        .filter(|&end| end <= bytes.len())
        .ok_or(FitError::Truncated(header_size))?;
    let data = &bytes[header_size..end];

    let mut definitions: Vec<Option<Definition>> = vec![None; 16];
    let mut records = Vec::new();
    let mut last_timestamp: u32 = 0;
    let mut pos = 0usize;

    while pos < data.len() {
        let header = data[pos];
        let header_pos = pos;
        pos += 1;

        if header & 0x80 != 0 {
            // Compressed timestamp header. The timestamp field is not stored in
            // the payload; all other fields are.
            let local = ((header >> 5) & 0x03) as usize;
            let time_offset = (header & 0x1F) as u32;
            let definition = definitions[local]
                .as_ref()
                .ok_or(FitError::UnknownLocalType(local as u8))?
                .clone();

            let mut timestamp = last_timestamp & !0x1F | time_offset;
            if time_offset < (last_timestamp & 0x1F) {
                timestamp = timestamp.wrapping_add(0x20);
            }
            last_timestamp = timestamp;

            let mut values = Vec::new();
            for field in &definition.fields {
                if field.number == FIELD_TIMESTAMP {
                    continue;
                }
                let offset = field.size as usize;
                let raw = read_bytes(data, &mut pos, offset)?;
                values.push((field.number, decode(field, raw, definition.big_endian)));
            }
            pos = skip_developer_fields(data, pos, &definition)?;

            if definition.global == RECORD_MESSAGE {
                emit_record(&values, Some(timestamp), &mut records);
            }
        } else if header & 0x40 != 0 {
            // Definition message.
            let local = (header & 0x0F) as usize;
            let has_developer_fields = header & 0x20 != 0;
            let _reserved = read_bytes(data, &mut pos, 1)?;
            let arch = read_bytes(data, &mut pos, 1)?[0];
            if arch > 1 {
                return Err(FitError::BadDefinition(header_pos));
            }
            let big_endian = arch == 1;
            let global = read_u16(data, &mut pos, big_endian)?;
            let count = read_bytes(data, &mut pos, 1)?[0];
            let mut fields = Vec::with_capacity(count as usize);
            for _ in 0..count {
                let descriptor = read_bytes(data, &mut pos, 3)?;
                fields.push(FieldDef {
                    number: descriptor[0],
                    size: descriptor[1],
                    base_type: descriptor[2],
                });
            }
            let mut developer_field_sizes = Vec::new();
            if has_developer_fields {
                let count = read_bytes(data, &mut pos, 1)?[0];
                for _ in 0..count {
                    let descriptor = read_bytes(data, &mut pos, 3)?;
                    developer_field_sizes.push(descriptor[1]);
                }
            }
            definitions[local] = Some(Definition {
                big_endian,
                global,
                fields,
                developer_field_sizes,
            });
        } else {
            // Data message.
            let local = (header & 0x0F) as usize;
            let definition = definitions[local]
                .as_ref()
                .ok_or(FitError::UnknownLocalType(local as u8))?
                .clone();

            let mut values = Vec::with_capacity(definition.fields.len());
            for field in &definition.fields {
                let raw = read_bytes(data, &mut pos, field.size as usize)?;
                values.push((field.number, decode(field, raw, definition.big_endian)));
            }
            pos = skip_developer_fields(data, pos, &definition)?;

            if let Some(value) = values
                .iter()
                .find(|(number, _)| *number == FIELD_TIMESTAMP)
                .and_then(|(_, value)| value.unsigned())
            {
                if value < u32::MAX as u64 {
                    last_timestamp = value as u32;
                }
            }

            if definition.global == RECORD_MESSAGE {
                emit_record(&values, None, &mut records);
            }
        }
    }

    Ok(records)
}

fn read_bytes<'a>(data: &'a [u8], pos: &mut usize, len: usize) -> Result<&'a [u8], FitError> {
    let end = pos.checked_add(len).ok_or(FitError::Truncated(*pos))?;
    if end > data.len() {
        return Err(FitError::Truncated(*pos));
    }
    let slice = &data[*pos..end];
    *pos = end;
    Ok(slice)
}

fn read_u16(data: &[u8], pos: &mut usize, big_endian: bool) -> Result<u16, FitError> {
    let raw = read_bytes(data, pos, 2)?;
    Ok(if big_endian {
        u16::from_be_bytes([raw[0], raw[1]])
    } else {
        u16::from_le_bytes([raw[0], raw[1]])
    })
}

fn skip_developer_fields(
    data: &[u8],
    mut pos: usize,
    definition: &Definition,
) -> Result<usize, FitError> {
    for &size in &definition.developer_field_sizes {
        read_bytes(data, &mut pos, size as usize)?;
    }
    Ok(pos)
}

fn decode(field: &FieldDef, raw: &[u8], big_endian: bool) -> Value {
    if raw.is_empty() {
        return Value::Invalid;
    }
    // Arrays are possible; only the first element is relevant here, but the
    // whole field was already consumed by the caller.
    let b = raw;
    let u16_at = |b: &[u8]| -> u16 {
        if big_endian {
            u16::from_be_bytes([b[0], b[1]])
        } else {
            u16::from_le_bytes([b[0], b[1]])
        }
    };
    let u32_at = |b: &[u8]| -> u32 {
        if big_endian {
            u32::from_be_bytes([b[0], b[1], b[2], b[3]])
        } else {
            u32::from_le_bytes([b[0], b[1], b[2], b[3]])
        }
    };
    let u64_at = |b: &[u8]| -> u64 {
        if big_endian {
            u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
        } else {
            u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
        }
    };

    match field.base_type {
        0x00 | 0x02 => {
            if b[0] == 0xFF {
                Value::Invalid
            } else {
                Value::Unsigned(b[0] as u64)
            }
        }
        0x01 => {
            if b[0] == 0x7F {
                Value::Invalid
            } else {
                Value::Signed(b[0] as i8 as i64)
            }
        }
        0x83 => {
            if b.len() < 2 {
                return Value::Invalid;
            }
            let v = u16_at(b);
            if v == 0x7FFF {
                Value::Invalid
            } else {
                Value::Signed(v as i16 as i64)
            }
        }
        0x84 => {
            if b.len() < 2 {
                return Value::Invalid;
            }
            let v = u16_at(b);
            if v == 0xFFFF {
                Value::Invalid
            } else {
                Value::Unsigned(v as u64)
            }
        }
        0x85 => {
            if b.len() < 4 {
                return Value::Invalid;
            }
            let v = u32_at(b);
            if v == 0x7FFF_FFFF {
                Value::Invalid
            } else {
                Value::Signed(v as i32 as i64)
            }
        }
        0x86 => {
            if b.len() < 4 {
                return Value::Invalid;
            }
            let v = u32_at(b);
            if v == u32::MAX {
                Value::Invalid
            } else {
                Value::Unsigned(v as u64)
            }
        }
        0x88 => {
            if b.len() < 4 {
                return Value::Invalid;
            }
            let bits = u32_at(b);
            if bits == u32::MAX {
                Value::Invalid
            } else {
                Value::Float(f32::from_bits(bits) as f64)
            }
        }
        0x89 => {
            if b.len() < 8 {
                return Value::Invalid;
            }
            let bits = u64_at(b);
            if bits == u64::MAX {
                Value::Invalid
            } else {
                Value::Float(f64::from_bits(bits))
            }
        }
        0x0A => {
            if b[0] == 0xFF {
                Value::Invalid
            } else {
                Value::Unsigned(b[0] as u64)
            }
        }
        0x8B => {
            if b.len() < 2 {
                return Value::Invalid;
            }
            let v = u16_at(b);
            if v == 0 {
                Value::Invalid
            } else {
                Value::Unsigned(v as u64)
            }
        }
        0x8C => {
            if b.len() < 4 {
                return Value::Invalid;
            }
            let v = u32_at(b);
            if v == 0 {
                Value::Invalid
            } else {
                Value::Unsigned(v as u64)
            }
        }
        0x0D => Value::Unsigned(b[0] as u64), // byte
        0x8E => {
            if b.len() < 8 {
                return Value::Invalid;
            }
            let v = u64_at(b);
            if v == 0x7FFF_FFFF_FFFF_FFFF {
                Value::Invalid
            } else {
                Value::Signed(v as i64)
            }
        }
        0x8F => {
            if b.len() < 8 {
                return Value::Invalid;
            }
            let v = u64_at(b);
            if v == u64::MAX {
                Value::Invalid
            } else {
                Value::Unsigned(v)
            }
        }
        0x90 => {
            if b.len() < 8 {
                return Value::Invalid;
            }
            let v = u64_at(b);
            if v == 0 {
                Value::Invalid
            } else {
                Value::Unsigned(v)
            }
        }
        _ => {
            // Unknown base type: treat the first byte as unsigned so alignment
            // is preserved.
            Value::Unsigned(b[0] as u64)
        }
    }
}

fn emit_record(values: &[(u8, Value)], compressed_timestamp: Option<u32>, out: &mut Vec<Record>) {
    let mut record = Record::default();
    let mut timestamp = compressed_timestamp;

    for &(number, value) in values {
        match number {
            0 => record.lat = value.signed().map(semicircles_to_degrees),
            1 => record.lon = value.signed().map(semicircles_to_degrees),
            2 => {
                record.ele = value
                    .unsigned()
                    .map(|v| v as f64 / 5.0 - 500.0)
                    .or_else(|| value.float());
            }
            3 => {
                // 0 means "no reading" on many devices and is outside the
                // Garmin TrackPointExtension schema (hr >= 1).
                record.hr = value
                    .unsigned()
                    .filter(|&v| v != 0xFF && v != 0)
                    .map(|v| v as u8);
            }
            4 => {
                record.cadence = value.unsigned().filter(|&v| v != 0xFF).map(|v| v as u8);
            }
            5 => record.distance = value.unsigned().map(|v| v as f64 / 100.0),
            6 => {
                record.speed = value
                    .unsigned()
                    .map(|v| v as f64 / 1000.0)
                    .or_else(|| value.float());
            }
            7 => {
                record.power = value.unsigned().filter(|&v| v != 0xFFFF).map(|v| v as u16);
            }
            13 => record.temperature = value.signed().map(|v| v as f64),
            FIELD_TIMESTAMP => {
                if let Some(raw) = value.unsigned() {
                    if raw < u32::MAX as u64 {
                        timestamp = Some(raw as u32);
                    }
                }
            }
            _ => {}
        }
    }

    if record.lat.is_none() || record.lon.is_none() {
        return;
    }
    if let Some(raw) = timestamp {
        let unix = raw as i64 + FIT_EPOCH_OFFSET;
        record.time = Local.timestamp_opt(unix, 0).single();
    }
    out.push(record);
}

fn semicircles_to_degrees(value: i64) -> f64 {
    value as f64 * 180.0 / 2_147_483_648.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fit_with_one_record() -> Vec<u8> {
        // Header + file_id definition/message + record definition/message.
        let mut file = Vec::new();
        file.extend_from_slice(&[0x0E, 0x10, 0x54, 0x08]); // header size/protocol/profile
        file.extend_from_slice(&[0, 0, 0, 0]); // data size placeholder
        file.extend_from_slice(b".FIT");
        file.extend_from_slice(&[0, 0]); // header CRC (not validated)
        let data_start = file.len();

        // file_id definition, local 0.
        file.extend_from_slice(&[0x40, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00]);
        file.push(0x00);
        file.push(0x04);

        // record definition, local 5, with timestamp/lat/lon/altitude/speed.
        file.extend_from_slice(&[
            0x45, 0x00, 0x00, 0x14, 0x00, 0x05, 0xFD, 0x04, 0x86, 0x00, 0x04, 0x85, 0x01, 0x04,
            0x85, 0x02, 0x02, 0x84, 0x06, 0x02, 0x84,
        ]);
        // record data
        file.push(0x05);
        file.extend_from_slice(&1_000_000u32.to_le_bytes());
        file.extend_from_slice(&610_501_578i32.to_le_bytes());
        file.extend_from_slice(&172_071_649i32.to_le_bytes());
        file.extend_from_slice(&3560u16.to_le_bytes());
        file.extend_from_slice(&8715u16.to_le_bytes());

        let data_size = (file.len() - data_start) as u32;
        file[4..8].copy_from_slice(&data_size.to_le_bytes());
        file
    }

    fn wrap_fit(data: &[u8]) -> Vec<u8> {
        let mut file = vec![0x0E, 0x10, 0x54, 0x08];
        file.extend_from_slice(&(data.len() as u32).to_le_bytes());
        file.extend_from_slice(b".FIT");
        file.extend_from_slice(&[0, 0]);
        let header_crc = crc16(&file[..12]);
        file[12..14].copy_from_slice(&header_crc.to_le_bytes());
        file.extend_from_slice(data);
        let file_crc = crc16(&file);
        file.extend_from_slice(&file_crc.to_le_bytes());
        file
    }

    fn fit_with_activity_order_bug() -> Vec<u8> {
        let mut data = Vec::new();

        // session definition (local 3, global 18) + data
        data.extend_from_slice(&[0x43, 0x00, 0x00, 0x12, 0x00, 0x01, 0xFD, 0x04, 0x86]);
        data.push(0x03);
        data.extend_from_slice(&1_000_000u32.to_le_bytes());

        // activity definition (local 4, global 34) with timestamp,
        // num_sessions and type.
        data.extend_from_slice(&[
            0x44, 0x00, 0x00, 0x22, 0x00, 0x03, 0xFD, 0x04, 0x86, 0x01, 0x02, 0x84, 0x02, 0x01,
            0x00,
        ]);
        // activity data: timestamp, num_sessions = 0 (the bug), type
        data.push(0x04);
        data.extend_from_slice(&1_000_001u32.to_le_bytes());
        data.extend_from_slice(&0u16.to_le_bytes());
        data.push(0);

        // sport definition (local 10, global 12) + data, after the activity.
        data.extend_from_slice(&[0x4A, 0x00, 0x00, 0x0C, 0x00, 0x01, 0x00, 0x01, 0x00]);
        data.push(0x0A);
        data.push(2);

        wrap_fit(&data)
    }

    #[test]
    fn normalizes_activity_order_and_num_sessions() {
        let original = fit_with_activity_order_bug();
        validate(&original).unwrap();

        let (fixed, repaired) = normalize(&original).unwrap();
        assert!(repaired);
        validate(&fixed).unwrap();

        let header_size = fixed[0] as usize;
        let data_size = u32::from_le_bytes([fixed[4], fixed[5], fixed[6], fixed[7]]) as usize;
        let data = &fixed[header_size..header_size + data_size];
        let messages = scan_messages(data).unwrap();
        let last = messages.last().unwrap();
        assert!(!last.is_definition);
        assert_eq!(last.global, 34, "activity message must be last");

        let activity = messages
            .iter()
            .rev()
            .find(|message| !message.is_definition && message.global == 34)
            .unwrap();
        let num_sessions_offset = header_size + activity.start + 1 + 4;
        assert_eq!(
            u16::from_le_bytes([fixed[num_sessions_offset], fixed[num_sessions_offset + 1]]),
            1
        );

        // Running the repair again must be a no-op.
        let (again, repaired_again) = normalize(&fixed).unwrap();
        assert!(!repaired_again);
        assert_eq!(again, fixed);
    }

    #[test]
    fn parses_record_fields() {
        let records = parse_records(&fit_with_one_record()).unwrap();
        assert_eq!(records.len(), 1);
        let record = &records[0];
        let lat = record.lat.unwrap();
        let lon = record.lon.unwrap();
        assert!((lat - 51.17).abs() < 0.01, "lat {lat}");
        assert!((lon - 14.42).abs() < 0.01, "lon {lon}");
        assert!((record.ele.unwrap() - 212.0).abs() < 1e-9);
        assert!((record.speed.unwrap() - 8.715).abs() < 1e-9);
        assert!(record.time.is_some());
    }

    #[test]
    fn crc_matches_known_vector() {
        assert_eq!(crc16(b"123456789"), 0xBB3D);
    }

    #[test]
    fn rejects_non_fit_data() {
        let mut bytes = vec![0u8; 14];
        bytes[0] = 14;
        bytes[8..12].copy_from_slice(b"NOPE");
        assert!(matches!(parse_records(&bytes), Err(FitError::BadSignature)));
        assert!(matches!(parse_records(b"tiny"), Err(FitError::TooShort)));
    }
}
