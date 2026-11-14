#!/usr/bin/env python3
"""Reference codec for the iGPSPORT BSC200 BLE protocol.

Reconstructed from the decompiled iGPSPORT Ride app (v8.06.28,
com.igpsport.blelib).  This file only implements the transport/framing layer;
the payload is ordinary protobuf and the message schemas are documented in
protobuf/.

The implementation is intentionally dependency-free so it can be used to
sanity-check captures.
"""

from __future__ import annotations

import struct
from dataclasses import dataclass
from typing import Optional

# ---------------------------------------------------------------------------
# CRC-8/MAXIM (a.k.a. CRC-8/DALLAS)
#   poly 0x31 reflected (0x8C), init 0x00, refin/refout true, xorout 0x00
# This matches com.igpsport.blelib.utils.CRC8 exactly.
# ---------------------------------------------------------------------------

def crc8(data: bytes) -> int:
    crc = 0
    for b in data:
        crc ^= b
        for _ in range(8):
            crc = (crc >> 1) ^ 0x8C if crc & 1 else crc >> 1
    return crc & 0xFF


# ---------------------------------------------------------------------------
# The (unusual) CRC-16 used only in the firmware sub-package header
# (com.igpsport.blelib.utils.CRC16Utils).  It is a nibble table algorithm and
# is NOT CRC-16/ARC even though its table is the reflected 0xA001 nibble table.
# Check value: crc16(b"123456789") == 0x1404
# ---------------------------------------------------------------------------

_CRC16_TABLE = (
    0x0000, 0xCC01, 0xD801, 0x1400,
    0xF001, 0x3C00, 0x2800, 0xE401,
    0xA001, 0x6C00, 0x7800, 0xB401,
    0x5000, 0x9C01, 0x8801, 0x4400,
)


def crc16(data: bytes) -> int:
    crc = 0
    for b in data:
        i = ((crc >> 4) & 0xFF) ^ _CRC16_TABLE[crc & 0x0F] ^ _CRC16_TABLE[b & 0x0F]
        crc = ((i >> 4) & 0xFF) ^ _CRC16_TABLE[i & 0x0F] ^ _CRC16_TABLE[(b >> 4) & 0x0F]
    return crc & 0xFFFF


# ---------------------------------------------------------------------------
# 20-byte frame header
# ---------------------------------------------------------------------------

# command_type (header byte 0)
TYPE_REQUEST = 0x01       # app -> device
TYPE_CONFIRM = 0x02       # ACK/confirm/error, status in byte 7, no payload
TYPE_NOTIFY = 0x03        # device-initiated notification / file stream

# end_type (header byte 10)
END_PROTOBUF = 0x01       # a complete protobuf message follows
END_FILE_LAST = 0x03      # last block of a streamed file

# reserved fill byte
RESERVED = 0xFF


@dataclass
class Frame:
    command_type: int
    main_service: int
    second_service: int = RESERVED
    byte3: int = RESERVED
    main_operation: int = RESERVED
    second_operation: int = RESERVED
    data_size: int = 0
    payload_crc: int = 0
    end_type: int = END_PROTOBUF
    status: int = 0
    header_crc: int = 0
    payload: bytes = b""

    def build_header(self) -> bytes:
        """Serialise the 20-byte header (bytes 0..19)."""
        if self.command_type == TYPE_CONFIRM:
            # ACK: byte 7 is the status, bytes 8..18 are reserved, no length.
            body = bytes([
                TYPE_CONFIRM,
                self.main_service,
                self.second_service,
                self.byte3,
                self.main_operation,
                self.second_operation,
                RESERVED,
                self.status,
            ]) + b"\xff" * 11
        else:
            size = len(self.payload)
            body = bytes([
                self.command_type,
                self.main_service,
                self.second_service,
                self.byte3,
                self.main_operation,
                self.second_operation,
                RESERVED,
                (size >> 8) & 0xFF,
                size & 0xFF,
                crc8(self.payload),
                self.end_type,
            ]) + b"\xff" * 8
        assert len(body) == 19
        return body + bytes([crc8(body)])

    def build(self) -> bytes:
        return self.build_header() + self.payload


def parse_header(data: bytes) -> Frame:
    if len(data) < 20:
        raise ValueError("need at least 20 bytes")
    if crc8(data[:19]) != data[19]:
        raise ValueError("header CRC mismatch")
    cmd = data[0]
    f = Frame(
        command_type=cmd,
        main_service=data[1],
        second_service=data[2],
        byte3=data[3],
        main_operation=data[4],
        second_operation=data[5],
        header_crc=data[19],
    )
    if cmd == TYPE_CONFIRM:
        f.status = data[7]
    else:
        f.data_size = (data[7] << 8) | data[8]
        f.payload_crc = data[9]
        f.end_type = data[10]
    return f


def parse_frame(data: bytes) -> tuple[Frame, bytes]:
    f = parse_header(data)
    payload = data[20:20 + f.data_size]
    if f.command_type != TYPE_CONFIRM and len(payload) != f.data_size:
        raise ValueError("truncated payload")
    return f, payload


# ---------------------------------------------------------------------------
# Request builder
# ---------------------------------------------------------------------------

def request(main_service: int, operation: int, payload: bytes,
            second_service: int = RESERVED, byte3: int = RESERVED,
            second_operation: int = RESERVED) -> bytes:
    return Frame(
        command_type=TYPE_REQUEST,
        main_service=main_service,
        second_service=second_service,
        byte3=byte3,
        main_operation=operation,
        second_operation=second_operation,
        payload=payload,
    ).build()


def confirm(main_service: int, operation: int, status: int,
            second_service: int = RESERVED,
            second_operation: int = RESERVED) -> bytes:
    return Frame(
        command_type=TYPE_CONFIRM,
        main_service=main_service,
        second_service=second_service,
        main_operation=operation,
        second_operation=second_operation,
        status=status,
    ).build()


# ---------------------------------------------------------------------------
# Firmware sub-package header (FILE_OPERATION firmware / MCU update)
#   5A A5 C3 3C | crc16(chunk) LE16 | len(chunk) LE16 | index LE32
# ---------------------------------------------------------------------------

def firmware_subpackage_header(chunk: bytes, index: int) -> bytes:
    return (b"\x5a\xa5\xc3\x3c"
            + struct.pack("<H", crc16(chunk))
            + struct.pack("<H", len(chunk) & 0xFFFF)
            + struct.pack("<I", index))


if __name__ == "__main__":
    # Examples from BSC200_BLE_PROTOCOL.md
    assert crc8(b"123456789") == 0xA1
    assert crc16(b"123456789") == 0x1404

    # service 17 (DEV_VER_INFO), operation GET (2), protobuf {1:17, 2:2}
    version_get = request(17, 2, bytes([0x08, 0x11, 0x10, 0x02]))
    assert version_get.hex(" ") == (
        "01 11 ff ff 02 ff ff 00 04 ad 01 ff ff ff ff ff ff ff ff ae"
        " 08 11 10 02"
    )

    f, payload = parse_frame(version_get)
    assert (f.main_service, f.main_operation) == (17, 2)
    assert payload == bytes([0x08, 0x11, 0x10, 0x02])

    ack = confirm(17, 2, 0)
    assert ack.hex(" ") == (
        "02 11 ff ff 02 ff ff 00 ff ff ff ff ff ff ff ff ff ff ff 44"
    )
    af, _ = parse_frame(ack)
    assert af.status == 0

    print("all self-checks passed")
