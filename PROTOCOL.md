# iGPSPORT BSC200 BLE protocol

Documentation reconstructed from the decompiled `iGPSPORT Ride` Android app in
the parent `igpsDecomp` workspace. The Rust/egui application, protocol
reference, codec and protobuf schemas live in this repository.

- **[docs/BSC200_BLE_PROTOCOL.md](docs/BSC200_BLE_PROTOCOL.md)** — full
  protocol reference (GATT layout, 20-byte frames, CRC-8/MAXIM, command
  engine, service/operation IDs, pairing, live data, file transfer, firmware).
- **[docs/reference_codec.py](docs/reference_codec.py)** — dependency-free
  Python encoder/decoder for the framing layer, with self-checks
  (`python3 docs/reference_codec.py`).
- **[docs/protobuf/](docs/protobuf/README.md)** — extracted protobuf message
  and enum schemas (field numbers and types) for every service.
- **[docs/schema_raw.json](docs/schema_raw.json)** — the same schema in
  machine-readable JSON.
- **[README.md](README.md)** — cross-platform Rust/egui app (Linux BlueZ and
  Windows WinRT) implementing GPX→CNX route import and FIT/GPX activity export
  over BLE, including HR/cadence/power/speed/distance GPX extensions.

Quick facts:

- BLE transport is Nordic-UART-style:
  `6e400001-b5a3-f393-e0a9-e50e24dcca9e` (main) and optional `…8e`, `…7e`,
  `…6e` channel pairs.
- Frames are a 20-byte header + protobuf payload. Header length is big-endian;
  header bytes 9 and 19 are CRC-8/MAXIM (poly 0x31 reflected, init 0,
  check `"123456789"` = `0xA1`).
- `command_type`: 1 request, 2 confirm/ACK (status in byte 7), 3 device
  notification/stream.
- BSC200 is BLE-only (no Wi-Fi) and supports MTU 247.
