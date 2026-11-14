//! iGPSPORT 20-byte frame header and CRC-8/MAXIM.

use thiserror::Error;

pub const HEADER_LEN: usize = 20;
pub const TYPE_REQUEST: u8 = 0x01;
pub const TYPE_CONFIRM: u8 = 0x02;
pub const TYPE_NOTIFY: u8 = 0x03;

pub const END_PROTOBUF: u8 = 0x01;
pub const END_FILE_LAST: u8 = 0x03;
pub const RESERVED: u8 = 0xFF;
pub const FILE_TAG: u8 = 0xAA;

/// CRC-8/MAXIM (a.k.a. CRC-8/DALLAS):
/// poly 0x31 reflected (0x8C), init 0, refin/refout true, xorout 0.
pub fn crc8(data: &[u8]) -> u8 {
    let mut crc = 0u8;
    for &b in data {
        crc ^= b;
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0x8C
            } else {
                crc >> 1
            };
        }
    }
    crc
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum FrameError {
    #[error("frame shorter than {HEADER_LEN} bytes")]
    Short,
    #[error("header CRC mismatch: got {got:#04x}, expected {expected:#04x}")]
    HeaderCrc { got: u8, expected: u8 },
    #[error("payload CRC mismatch: got {got:#04x}, expected {expected:#04x}")]
    PayloadCrc { got: u8, expected: u8 },
    #[error("payload truncated: expected {expected} bytes, got {got}")]
    Truncated { expected: usize, got: usize },
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FrameHeader {
    pub command_type: u8,
    pub main_service: u8,
    pub second_service: u8,
    pub byte3: u8,
    pub main_operation: u8,
    pub second_operation: u8,
    pub data_size: usize,
    pub payload_crc: u8,
    pub end_type: u8,
    pub status: u8,
    pub header_crc: u8,
}

impl FrameHeader {
    pub fn request(
        main_service: u8,
        main_operation: u8,
        payload_len: usize,
        payload_crc: u8,
        second_service: u8,
        byte3: u8,
        second_operation: u8,
    ) -> Self {
        Self {
            command_type: TYPE_REQUEST,
            main_service,
            second_service,
            byte3,
            main_operation,
            second_operation,
            data_size: payload_len,
            payload_crc,
            end_type: END_PROTOBUF,
            ..Default::default()
        }
    }

    pub fn confirm(
        main_service: u8,
        main_operation: u8,
        status: u8,
        second_service: u8,
        second_operation: u8,
    ) -> Self {
        Self {
            command_type: TYPE_CONFIRM,
            main_service,
            second_service,
            byte3: RESERVED,
            main_operation,
            second_operation,
            status,
            ..Default::default()
        }
    }

    /// Serialise bytes 0..19, then append the header CRC as byte 19.
    pub fn to_bytes(&self) -> [u8; HEADER_LEN] {
        let mut h = [RESERVED; HEADER_LEN];
        h[0] = self.command_type;
        h[1] = self.main_service;
        h[2] = self.second_service;
        h[3] = self.byte3;
        h[4] = self.main_operation;
        h[5] = self.second_operation;
        h[6] = RESERVED;
        if self.command_type == TYPE_CONFIRM {
            h[7] = self.status;
            // bytes 8..18 remain 0xFF
        } else {
            h[7] = ((self.data_size >> 8) & 0xFF) as u8;
            h[8] = (self.data_size & 0xFF) as u8;
            h[9] = self.payload_crc;
            h[10] = self.end_type;
        }
        h[19] = crc8(&h[..19]);
        h
    }

    /// Parse and validate a 20-byte header.
    pub fn parse(data: &[u8]) -> Result<Self, FrameError> {
        if data.len() < HEADER_LEN {
            return Err(FrameError::Short);
        }
        let expected = crc8(&data[..19]);
        if data[19] != expected {
            return Err(FrameError::HeaderCrc {
                got: data[19],
                expected,
            });
        }
        let command_type = data[0];
        let mut h = Self {
            command_type,
            main_service: data[1],
            second_service: data[2],
            byte3: data[3],
            main_operation: data[4],
            second_operation: data[5],
            header_crc: data[19],
            ..Default::default()
        };
        if command_type == TYPE_CONFIRM {
            h.status = data[7];
        } else {
            h.data_size = ((data[7] as usize) << 8) | data[8] as usize;
            h.payload_crc = data[9];
            h.end_type = data[10];
        }
        Ok(h)
    }

    pub fn service_op(&self) -> (u8, u8) {
        (self.main_service, self.main_operation)
    }
}

/// Encode a complete frame (header + payload).
pub fn encode_frame(header: &FrameHeader, payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_LEN + payload.len());
    out.extend_from_slice(&header.to_bytes());
    out.extend_from_slice(payload);
    out
}

/// Encode a request frame, calculating the payload CRC and length automatically.
pub fn encode_request(
    main_service: u8,
    main_operation: u8,
    payload: &[u8],
    second_service: u8,
    byte3: u8,
    second_operation: u8,
) -> Vec<u8> {
    let h = FrameHeader::request(
        main_service,
        main_operation,
        payload.len(),
        crc8(payload),
        second_service,
        byte3,
        second_operation,
    );
    encode_frame(&h, payload)
}

/// Parse one complete frame from a byte slice. Returns the header, payload and
/// the number of bytes consumed.
pub fn parse_frame(data: &[u8]) -> Result<(FrameHeader, Vec<u8>, usize), FrameError> {
    let h = FrameHeader::parse(data)?;
    let total = if h.command_type == TYPE_CONFIRM {
        HEADER_LEN
    } else {
        HEADER_LEN + h.data_size
    };
    if data.len() < total {
        return Err(FrameError::Truncated {
            expected: total,
            got: data.len(),
        });
    }
    let payload = data[HEADER_LEN..total].to_vec();
    if h.command_type != TYPE_CONFIRM {
        let expected = crc8(&payload);
        if expected != h.payload_crc {
            return Err(FrameError::PayloadCrc {
                got: h.payload_crc,
                expected,
            });
        }
    }
    Ok((h, payload, total))
}

/// Accumulates BLE notifications into complete frames.
#[derive(Default)]
pub struct FrameAssembler {
    buf: Vec<u8>,
}

impl FrameAssembler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, data: &[u8]) -> Vec<Result<(FrameHeader, Vec<u8>), FrameError>> {
        self.buf.extend_from_slice(data);
        let mut out = Vec::new();
        loop {
            if self.buf.len() < HEADER_LEN {
                break;
            }
            match FrameHeader::parse(&self.buf[..HEADER_LEN]) {
                Ok(h) => {
                    let total = if h.command_type == TYPE_CONFIRM {
                        HEADER_LEN
                    } else {
                        HEADER_LEN + h.data_size
                    };
                    if self.buf.len() < total {
                        break;
                    }
                    let payload = self.buf[HEADER_LEN..total].to_vec();
                    if h.command_type != TYPE_CONFIRM {
                        let expected = crc8(&payload);
                        if expected != h.payload_crc {
                            // Drop the bad header and report it; keep the rest.
                            self.buf.drain(..HEADER_LEN);
                            out.push(Err(FrameError::PayloadCrc {
                                got: h.payload_crc,
                                expected,
                            }));
                            continue;
                        }
                    }
                    self.buf.drain(..total);
                    out.push(Ok((h, payload)));
                }
                Err(e) => {
                    // We are not at a frame boundary. Drop one byte and retry,
                    // which recovers from stray bytes/continuation fragments.
                    self.buf.drain(..1);
                    out.push(Err(e));
                }
            }
        }
        out
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }

    pub fn buffered(&self) -> usize {
        self.buf.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc8_check_value() {
        assert_eq!(crc8(b"123456789"), 0xA1);
    }

    #[test]
    fn version_get_example() {
        let payload = [0x08, 0x11, 0x10, 0x02];
        let frame = encode_request(17, 2, &payload, RESERVED, RESERVED, RESERVED);
        assert_eq!(
            frame,
            vec![
                0x01, 0x11, 0xff, 0xff, 0x02, 0xff, 0xff, 0x00, 0x04, 0xad, 0x01, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xae, 0x08, 0x11, 0x10, 0x02
            ]
        );
        let (h, p, n) = parse_frame(&frame).unwrap();
        assert_eq!((h.main_service, h.main_operation), (17, 2));
        assert_eq!(p, payload);
        assert_eq!(n, frame.len());
    }

    #[test]
    fn confirm_example() {
        let frame = FrameHeader::confirm(17, 2, 0, RESERVED, RESERVED).to_bytes();
        assert_eq!(
            frame,
            [
                0x02, 0x11, 0xff, 0xff, 0x02, 0xff, 0xff, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0x44
            ]
        );
    }

    #[test]
    fn assembler_reassembles_fragments() {
        let payload = [0x08, 0x11, 0x10, 0x02];
        let frame = encode_request(17, 2, &payload, RESERVED, RESERVED, RESERVED);
        let mut a = FrameAssembler::new();
        assert!(a.push(&frame[..5]).is_empty());
        let out = a.push(&frame[5..]);
        assert_eq!(out.len(), 1);
        let (h, p) = out.into_iter().next().unwrap().unwrap();
        assert_eq!(h.main_service, 17);
        assert_eq!(p, payload);
    }
}
