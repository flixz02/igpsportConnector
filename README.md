# bsc200-tool

Cross-platform desktop companion for the **iGPSPORT BSC200** bike computer,
written in Rust with an `egui` GUI.

* **Linux** uses BlueZ through `bluer` (D-Bus).
* **Windows 10/11** uses the native WinRT Bluetooth stack through `btleplug`.

It implements the protocol documented in
[`../docs/BSC200_BLE_PROTOCOL.md`](../docs/BSC200_BLE_PROTOCOL.md), with a focus
on two workflows:

* **Import a GPX route** by generating a native iGPSPORT `.cnx` file locally
  (no iGPSPORT cloud/account required) and uploading it over BLE.
* **Export recorded activities** by downloading FIT files from the device and
  saving them as FIT and/or GPX.

## Status

This implementation has been validated against a physical BSC200
(`BLE 144`, `proto 101`, `hw 100`). GPX route import (CNX generation and
upload), route activation, activity listing, FIT download and FIT→GPX export
all work on real hardware. See §11 of
[`../docs/BSC200_BLE_PROTOCOL.md`](../docs/BSC200_BLE_PROTOCOL.md) for the
hardware findings.

The Linux backend has been validated on physical BSC200 hardware. The Windows
backend compiles and uses the Windows WinRT BLE stack, but was not tested on
BSC200 hardware in this development environment.

The protocol/format core is covered by unit tests:

```bash
cargo test
```

## Build requirements

Common:

* Rust stable (edition 2021; tested with Rust 1.100)
* An OpenGL-capable graphics driver for `eframe`

Linux:

* BlueZ and a running `bluetoothd`
* D-Bus
* X11 or Wayland

The crate enables the vendored `libdbus` build, so `libdbus-1-dev`/pkg-config
headers are not required just to compile. At runtime BlueZ still needs its
normal D-Bus service.

Windows:

* Windows 10 or 11
* Rust with the `x86_64-pc-windows-msvc` toolchain and the Visual Studio
  Build Tools "Desktop development with C++" workload (or a mingw-w64
  toolchain)
* Bluetooth enabled; pair the BSC200 once in
  **Settings → Bluetooth & devices**

```bash
# Linux
cargo build --release
cargo run --release

# Windows (PowerShell / cmd)
cargo build --release
.\target\release\bsc200-tool.exe
```

On Linux, some distributions require the user to be in the `bluetooth` group
or to have the usual desktop polkit permissions to scan/pair. On Windows, the
first connection may show the system pairing prompt; all later connections use
the existing bond.

## GUI

* **Device** — scan, connect, pair (Linux: numeric-comparison/passkey prompt;
  Windows: OS pairing), read firmware version and battery, read routes.
  A manual address field also connects directly, which is useful on Windows
  when a paired BSC200 is not currently advertising.
* **Routes** — choose a GPX, preview the generated CNX, save the CNX, upload to
  the BSC200, optionally activate it.
* **Activities** — list activities, select files or download all, save FIT,
  optionally convert to GPX, open the output folder. GPX export includes
  elevation, time and the FIT track-point extensions listed below.
* **Log** — live protocol/status log, copy to clipboard.
* **Settings** — output folder, GPX conversion, route activation. The output
  folder and toggles are persisted in the user config directory.

## CLI helpers (no device required)

```bash
# GPX -> native CNX
cargo run -- --make-cnx route.gpx route.cnx

# Inspect a native CNX
cargo run -- --dump-cnx route.cnx

# FIT activity -> GPX
cargo run -- --fit-to-gpx activity.fit activity.gpx

# Repair a BSC200 FIT file for strict viewers
cargo run -- --repair-fit activity.fit activity-fixed.fit
```

## Hardware test CLI

These commands exercise the real device without starting the GUI:

```bash
# Scan / read version and battery
cargo run -- --hw-scan
cargo run -- --hw-info AA:BB:CC:DD:EE:FF

# Lists
cargo run -- --hw-activities AA:BB:CC:DD:EE:FF
cargo run -- --hw-routes AA:BB:CC:DD:EE:FF

# Download one activity (or --all) and export GPX
cargo run -- --hw-download AA:BB:CC:DD:EE:FF ./out --id 1234567890 --gpx

# Upload and activate a route
cargo run -- --hw-upload AA:BB:CC:DD:EE:FF route.gpx --activate
```

## FIT normalization

BSC200 firmware writes two things that strict FIT validators reject:

1. The `sport` message (global message 12) is appended **after** the
   `activity` message (34), although the FIT specification requires
   `activity` to be the last message.
2. `activity.num_sessions` (field 1) is written as `0`, even though the file
   contains one `session` message (global message 18).

The app repairs downloaded FIT files automatically: it moves trailing
messages before the activity message, re-emits the activity definition, sets
`num_sessions` to the actual session count, and recomputes the header and file
CRCs. The repair is data-preserving. Existing files can be fixed offline with:

```bash
cargo run -- --repair-fit original.fit fixed.fit
```

The repair was verified byte-for-byte against the fixed file returned by an
online FIT viewer for a real BSC200 activity.

## GPX track-point extensions

GPX 1.1 has no standard fields for cycling metrics, so the converter writes the
common Garmin extension namespaces plus one custom field:

| Metric | Element | Namespace |
|---|---|---|
| Heart rate | `gpxtpx:hr` | `TrackPointExtension/v2` |
| Cadence | `gpxtpx:cad` | `TrackPointExtension/v2` |
| Speed (m/s) | `gpxtpx:speed` | `TrackPointExtension/v2` |
| Temperature (°C) | `gpxtpx:atemp` | `TrackPointExtension/v2` |
| Power (W) | `gpxx:Watts` | `ActivityExtension/v2` |
| Cumulative distance (m) | `bsc200:distance` | `urn:bsc200-tool:extensions:v1` |

Garmin Connect, Strava and most analysis tools read the `gpxtpx` fields;
`gpxx:Watts` is the Garmin activity-extension spelling for power. The distance
element is a private extension because GPX has no standard equivalent.

## CNX encoding

The encoder follows the native iGPSPORT format:

* `<Tracks>` first point: `lat,lon,elevation_mm`
* subsequent lat/lon: first-order then second-order deltas on a `1e-7°` grid
* subsequent elevation: first-order delta in cm
* `<Encode>2</Encode>`, `<Lang>0</Lang>`
* `<Distance>` in metres (2 decimals), `<Ascent>`/`<Descent>` rounded metres
  (descent negative)
* 8-digit content-derived `<Id>`
* optional `<Points>` POIs

There is a round-trip self-test in `src/cnx.rs`.

## Protocol notes

* 20-byte header, CRC-8/MAXIM, big-endian payload length.
* Gen-4 framing: `header + payload` in one write on the data channel.
* Gen-2/3 framing: payload then header on the control channel.
* The app auto-probes both at connect time.
* Route upload uses `FILE_OPERATION`/`ADD` with header byte 3 = `0xAA` on
  channel 4. Some firmware sends no completion ACK; the app then verifies the
  route by re-reading the device route list.
* Activity download handles both the gen-4
  `u32 BE pb length + FileDownload + FIT` format and the gen-3 chunked
  `cycling_data_msg.file_content` format with per-block ACKs.
* Gen-4 activity downloads use header byte 3 = `0x55` on channel 3 and only
  consume notifications from channel 3. The BSC200 interleaves unsolicited
  `BACK` frames on channel 2; appending those to the file corrupts the FIT
  stream.
* Activity/route list timestamps are seconds since the FIT epoch
  (`631065600`) and encode the device's local wall-clock time (they include
  the device UTC offset). Filenames are derived by formatting
  `timestamp + 631065600` as UTC, which matches the time shown on the BSC200.
* Battery is read from the standard GATT Battery service (`0x180F/0x2A19`).
  The tested BSC200 does not expose that service, so battery is reported as
  unknown.
* On connect the app tries gen-4 framing, then gen-3, and retries the version
  read with the other framing if needed. The tested BSC200 answers gen-4
  directly.

## Known limitations

* Battery level is unavailable on BSC200 units without the standard Battery
  GATT service.
* The BSC200 frequently answers route-list requests on channel 1 even when
  the request was sent on channel 4 (and vice versa). The app retries the
  alternate channel, which adds a few seconds to a refresh.
* The Windows backend reports the bond state as unknown (WinRT does not expose
  it through `btleplug`), so the scan list never shows a `[paired]` marker on
  Windows. Pairing itself is handled by Windows.
* Passkey *entry* (keyboard) is not implemented on Linux; numeric-comparison
  and authorization prompts are. Most BSC200 pairing is expected to use Just
  Works or numeric comparison. Windows uses the OS pairing UI.
* No device settings editor, sensors, workouts, maps, firmware update, live
  tracking, or cloud sync.
