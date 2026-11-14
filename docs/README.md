# iGPSPORT BSC200 reverse-engineering notes

This directory documents the Bluetooth LE protocol used by the iGPSPORT BSC200
bike computer, reconstructed by reverse-engineering the `iGPSPORT Ride`
Android app (v8.06.28). The decompiled app itself is not included in this
repository.

| File | Description |
|---|---|
| [`BSC200_BLE_PROTOCOL.md`](BSC200_BLE_PROTOCOL.md) | Main protocol reference: GATT layout, 20-byte framing, CRCs, command engine, services/operations, workflows, file transfer |
| [`reference_codec.py`](reference_codec.py) | Dependency-free Python encoder/decoder for the framing layer, with self-checks |
| [`protobuf/`](protobuf/README.md) | Extracted protobuf message and enum schemas, one file per service/bean |
| [`schema_raw.json`](schema_raw.json) | Machine-readable schema (messages, field numbers, types, enums) |
| [`../PROTOCOL.md`](../PROTOCOL.md) | Short protocol index and quick facts |
| [`../`](../README.md) | Cross-platform Rust/egui app implementing route import and activity export |

## Quick start

```bash
python3 docs/reference_codec.py   # run the framing self-checks
```

Read `BSC200_BLE_PROTOCOL.md` first. The high-confidence parts are:

* Nordic UART-style GATT channels (`6e400001-…`).
* 20-byte header with CRC-8/MAXIM (not CRC-16).
* Big-endian payload length and standard protobuf payloads.
* Service IDs and operation IDs in §3–§4.
* Generic file upload format with `byte3 = 0xAA` and a 4-byte big-endian
  protobuf length.

The main unresolved detail is the exact notification-reassembly logic in one
JADX-failed method; see §12 of the main document.
