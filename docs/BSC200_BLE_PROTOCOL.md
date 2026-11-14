# iGPSPORT BSC200 — Bluetooth LE protocol

Reconstructed from the decompiled Android app **iGPSPORT Ride 8.06.28**
(`com.qiwu.worldwide.ride`, BLE SDK in `com.igpsport.blelib`) in the parent
`igpsDecomp` workspace. Paths below refer to that workspace.

Everything here is derived from the app's own code. The main sources are:

| Area | Source file(s) |
|---|---|
| GATT services / characteristics | `app/src/main/java/com/igpsport/blelib/manager/*.java` |
| 20-byte frame header | `pbfactory/BaseHead20Bytes.java`, `pbfactory/BaseFactory.java`, `pbfactory/ConfirmCommand.java`, `pbfactory/TransmitCompleteCommand.java` |
| CRC-8 | `utils/CRC8.java` |
| CRC-16 (firmware only) | `utils/CRC16Utils.java` |
| Request/response engine | `BaseCommand.java`, `IGPDeviceManager.java` |
| Command service abstraction | `pbfactory/*ServiceFactory.java` |
| Payload schemas | `bean/*.java` (generated protobuf-lite) |
| File transfer | `devicemanager/SyncActivityDelegate.java`, `OtherDelegate.java`, `RoutePlanningDelegate.java`, `FirmwareUpdateDelegate.java` |
| BSC200 feature checks | `config/DeviceConstant.java`, `globalapp/devicemodule/pagesetting/**` |

A dependency-free reference implementation of the framing layer is in
[`reference_codec.py`](reference_codec.py). Full extracted protobuf schemas
are in [`protobuf/`](protobuf/). A complete cross-platform Rust/egui
implementation (route import + activity export) is in the parent directory;
see [`../README.md`](../README.md).

> **Confidence note.** The transport framing, CRCs, service IDs, operation IDs
> and payload field numbers are directly readable in the code and are high
> confidence. The byte order of multi-byte payload fields is standard protobuf
> (little-endian varints/fixed32/fixed64). Units of a few live-data fields are
> not used by the app and are therefore marked *raw*.

---

## 1. Transport

### 1.1 BLE GATT layout

The device exposes the Nordic UART Service (NUS) pattern on up to four channel
pairs. All UUIDs below are full 128-bit UUIDs; channels 3 and 4 are optional
depending on the model/firmware.

| Channel | Service UUID | RX (write, app→device) | TX (notify, device→app) | Purpose |
|---|---|---|---|---|
| 1 | `6e400001-b5a3-f393-e0a9-e50e24dcca9e` | `6e400002-…-e50e24dcca9e` | `6e400003-…-e50e24dcca9e` | Main data channel |
| 2 | `6e400001-b5a3-f393-e0a9-e50e24dcca8e` | `6e400002-…-e50e24dcca8e` | `6e400003-…-e50e24dcca8e` | Control channel (headers/ACKs on gen 2–3) |
| 3 | `6e400001-b5a3-f393-e0a9-e50e24dcca7e` | `6e400002-…-e50e24dcca7e` | `6e400003-…-e50e24dcca7e` | Log / auxiliary channel |
| 4 | `6e400001-b5a3-f393-e0a9-e50e24dcca6e` | `6e400002-…-e50e24dcca6e` | `6e400003-…-e50e24dcca6e` | New-generation config/file channel |

Standard services used in addition:

| Service | Characteristic | Use |
|---|---|---|
| `00001800-0000-1000-8000-00805f9b34fb` (Generic Access) | `00002a00` Device Name | Read/write device name |
| `0000180a-0000-1000-8000-00805f9b34fb` (Device Information) | `2a29` manufacturer, `2a28` software, `2a27` hardware, `2a26` firmware | Identification |
| `0000180f-0000-1000-8000-00805f9b34fb` (Battery) | `00002a19` Battery Level | Battery %, notifications |
| `00001816-0000-1000-8000-00805f9b34fb` (CSC) | `00002a5b` CSC Measurement | Speed/cadence sensor passthrough |
| `0000180d-0000-1000-8000-00805f9b34fb` (Heart Rate) | `00002a37` HR Measurement | HR sensor passthrough |
| Nordic Secure DFU service | (`no.nordicsemi.android.dfu`) | Firmware update after `EnterDFU` |

The app treats channels 1–4 as independent command queues (`firstQueue` …
`fourthQueue`). Received notifications on any TX characteristic are fed to the
currently active command on that channel.

### 1.2 MTU and chunking

* The app asks for **MTU 247** on devices listed in
  `DeviceConstant.MTU247_SUPPORTED_DEVICES`; **BSC200 is in that list**
  (`DeviceConstant.java:57`). ATT payload is therefore up to **244 bytes**.
* Writes use `writeCharacteristic(..., data, 1)` and are split by the Nordic
  BLE library into MTU-sized chunks. A logical frame is simply the
  concatenation of those chunks; there is no per-chunk header.
* `IGPDevice.getSendFileMtuSize()` defaults to **4096**, used as the logical
  block size for some file transfers.
* BSC200 is in `BLE_ONLY_DEVICE_LIST` (`DeviceConstant.java:37`), i.e. it has
  no Wi-Fi module. Wi-Fi-related services are therefore not applicable even
  though the SDK implements them.

### 1.3 Connection / pairing modes

`DeviceInformation.DeviceInfo.connectionMode` (from the server-side device
record) selects the pairing flow (`ConnectMode.java`):

| Value | Mode | Meaning |
|---:|---|---|
| 1 | `NotBound` | BLE-only connection, no bonding |
| 2 | `BindMode` | BLE bonding + account binding |
| 3 | `VerificationCode` | Passkey/verification-code flow |

`DeviceInfo.pairMode` / `PairMode` enum: `NONE=0`, `JUST_WORKS=1`,
`PASSKEY_ENTRY=2`, `NUMERIC_COMPARISON=3`.

The binding state itself is exchanged over the **BLE service (10)** — see
§5.1.

---

## 2. Frame format

All new-SDK (generation 2+) protocol traffic is a **20-byte header followed
by an optional payload**. The payload is a protobuf message (proto2 syntax,
protobuf-lite wire format).

### 2.1 Header layout

| Offset | Size | Name | Meaning |
|---:|---:|---|---|
| 0 | 1 | `command_type` | `0x01` request (app→device), `0x02` confirm/ACK, `0x03` notification / streamed data |
| 1 | 1 | `main_service` | Service ID (`Common.service_type_index`, §3) |
| 2 | 1 | `second_service` | Sub-service ID, or `0xFF` |
| 3 | 1 | `byte3` | Usually `0xFF`; `0xAA` marks a generic file upload (`PktConstants.HEAD_CODE`) |
| 4 | 1 | `main_operation` | Primary operation ID (service-specific enum, §4) |
| 5 | 1 | `second_operation` | Secondary operation / sub-operation, or `0xFF` |
| 6 | 1 | reserved | Always `0xFF` |
| 7 | 1 | `data_size_hi` / `status` | For type 1/3: payload length, high byte. For type 2: status code (§2.4) |
| 8 | 1 | `data_size_lo` | Payload length, low byte (type 1/3 only) |
| 9 | 1 | `payload_crc` | CRC-8 of the payload bytes only |
| 10 | 1 | `end_type` | `1` = complete protobuf message, `3` = last block of a file stream |
| 11–18 | 8 | reserved | Always `0xFF` |
| 19 | 1 | `header_crc` | CRC-8 of header bytes 0–18 |

Notes:

* The payload length is **big-endian** (`BaseFactory.unsignedShortToByte2`).
* `end_type` is not a generic "more data" flag: the app only distinguishes
  `1` (protobuf) and `3` (last file block). Continuation blocks are identified
  by the absence of a new valid header.
* A type-2 confirm frame has **no payload and no length**; byte 7 carries the
  status and bytes 8–18 are `0xFF`.
* `0xFF` is the reserved filler everywhere.

### 2.2 CRC-8

`com.igpsport.blelib.utils.CRC8` is **CRC-8/MAXIM** (a.k.a. CRC-8/DALLAS):

```
poly = 0x31 reflected (0x8C), init = 0x00, refin = refout = true, xorout = 0x00
check("123456789") = 0xA1
```

The same algorithm is used twice:

* `header[9] = CRC8(payload)`
* `header[19] = CRC8(header[0..18])`

### 2.3 Worked example — read firmware versions

Service `17` (`DEV_VER_INFO`), operation `2` (`GET`). The payload is
`dev_ver_info_msg { service_type = 17, operate_type = 2 }`, which encodes as
protobuf bytes `08 11 10 02`.

```
01 11 ff ff 02 ff ff 00 04 ad 01 ff ff ff ff ff ff ff ff ae  08 11 10 02
│  │  │  │  │  │  │  └─┴─ length = 4            │  │            └─ protobuf payload
│  │  │  │  │  │  │        crc8(payload)=0xAD ──┘  │
│  │  │  │  │  │  │        end_type = 1            │
│  │  │  │  │  │  └─ second op = 0xFF              └─ crc8(header[0..18]) = 0xAE
│  │  │  │  │  └─ main op = 2 (GET)
│  │  │  │  └─ byte3 = 0xFF
│  │  │  └─ second service = 0xFF
│  │  └─ main service = 17
│  └─ reserved
└─ request
```

The device answers with a type-1 or type-3 frame carrying service/operation
17/2 and a `version_msg` payload. A successful confirmation of that command is:

```
02 11 ff ff 02 ff ff 00 ff ff ff ff ff ff ff ff ff ff ff 44
```

(`status = 0`, CRC-8 of the first 19 bytes = `0x44`).

### 2.4 ACK / status codes

A type-2 frame confirms a request. `status` (byte 7) is
`DeviceReturnStatus`:

| Value | Name | Meaning |
|---:|---|---|
| 0 | `Success` | OK |
| 1 | `DataError` | Malformed data / CRC error |
| 2 | `MemoryError` | Device storage error |
| 3 | `LowBattery` | Battery too low |
| 4 | `QuantityIsFull` | e.g. route/workout list full |
| 5 | `IsBeingUsed` | Resource busy |
| 6 | `UnsupportedCommand` | Command not supported by this firmware |
| 16 | `WifiConnectionSucceeded` | |
| 17 | `WifiWrongPassword` | |
| 18 | `WifiConnectionTimedOut` | |
| 19 | `WifiNotConnected` | |
| 20 | `WifiPleaseEnterPassword` | |
| 21 | `WifiMapDownload` | Wi-Fi busy downloading maps |
| 22 | `WifiFirmwareDownload` | Wi-Fi busy downloading firmware |
| 23 | `WifiCyclingActivityIsUploading` | Wi-Fi busy uploading activities |
| 65 | `NavigationRouteDeletionFailed` | |
| 66 | `NavigationRouteDoesNotExist` | |
| 80 | `PacketAckSuccess` | Per-packet ACK (file transfer) |
| 81 | `PacketAckError` | Per-packet NAK |
| 82 | `PacketAckTimeout` | |
| 100 | `McuFwCorrupted` | |
| 112 | `SportCannotOperateDial` | watch dial |
| 113 | `DeleteDialDoesNotExist` | watch dial |
| 114 | `LastDialCannotBeDeleted` | watch dial |
| 115 | `UnzipDialCannotOperateDial` | watch dial |

The app treats status `0` as success. The generic command engine
(`BaseCommand.checkIsFinish`) special-cases only a few nonzero values in a
type-2 frame:

* `1` (`DataError`) — retry the whole command;
* `2` (`MemoryError`) — abort and report;
* `6` (`UnsupportedCommand`) — abort and report;
* `23` (`WifiCyclingActivityIsUploading`) — abort and report "device busy".

Other nonzero statuses are passed to the per-command translation callback,
which may or may not inspect them. When implementing a client, treat any
nonzero status as failure unless the specific operation documents otherwise.
Some services return extra status mapping, e.g. route plans
(`ErrorCode.linePlanMap`): 0 success, 1 data error, 4 list full, 5 route in
use, 65 deletion failed, 66 route missing.

### 2.5 Reliability / command engine

`BaseCommand.java`:

* Every logical command is enqueued on one of four `ConcurrentLinkedQueue`s
  (one per GATT channel). Only the head command runs at a time per queue.
* **Send timeout:** 6000 ms (`RetryPolicy.DEFAULT_RETRY_TIMEOUT_IN_MILLIS`).
* **Receive timeout:** `IGPDevice.getDataReceiveTimeOut()` — 1500 ms base,
  6000 ms for `IGSDevice` (all protobuf bike computers).
* Retries: 3 by default (`BaseCommand.retryTime = 3`), resetting the command
  from scratch. `setRetryTime(0)` is used for non-idempotent file transfers and
  bonding operations.
* A response is matched to a command by comparing the response header's
  `main_service` and `main_operation` (and often `second_operation`) to the
  request. The payload alone is not used for matching.
* `stopSendingData()` aborts chunked writes; `onPacketSent` is called for every
  MTU chunk (used for progress and pacing).
* On GATT error `-1`, `8`, or `133` the manager disconnects and reconnects once
  after 1 s.

The command `WriteCommand` enum (e.g. `ReadDeviceStatus`, `SendRoadPlanFile`)
is only an internal tag used for logging/queue control; it is not sent over
the air.

---

## 3. Service IDs

`Common.service_type_index` — this is header byte 1.

| ID | Name | Meaning | Payload schema |
|---:|---|---|---|
| 0 | `NONE` | Reserved | — |
| 1 | `INS` | Phone notifications / calls / find-my-phone | `Ins` |
| 2 | `MAP` | Legacy map management | `Map` |
| 3 | `BACK` | Server→device data: weather, time, AGPS, location | `Back` |
| 4 | `FIRMWARE` | Firmware version / update / DFU | `Firmware` |
| 5 | `WIFI` | Wi-Fi provisioning and upload (not on BSC200) | `Wifi` |
| 6 | `CYCLING_DATA` | Activity files | `CyclingData` |
| 7 | `ROUTE_PLAN` | Routes / navigation | `RoutePlan` |
| 8 | `REAL_TIME_TRACE` | Live-tracking FIT stream | `RealTimeTrace` |
| 9 | `USER_CONFIG` | Rider profile | `UserConfig` |
| 10 | `BLE` | Bonding / pairing / account binding | `Ble` |
| 11 | `FACTORY` | Factory test / manufacturing | `Factory` |
| 12 | `CONFIG` | All device settings (pages, units, bike, alarms…) | `Config` |
| 13 | `DEV_STATUS` | Live status, position points, ride state | `DevStatus` |
| 14 | `SENSOR` | Sensor list / pairing / calibration | `Sensor` |
| 15 | `TRAINING` | Workouts / segments | `Training` |
| 16 | `TEAM_INFO` | Team members | `TeamInfo` |
| 17 | `DEV_VER_INFO` | Firmware version info | `DevVerInfo` |
| 18 | `LANGUAGE` | Language packs | `LanguagePack` |
| 19 | `LOG` | Device logs | `Log` |
| 20 | `THEME` | Themes / backgrounds | `Theme` |
| 21 | `FILE_OPERATION` | Generic file upload (routes, workouts, firmware, maps, AGPS, themes) | `GeneralFileOperation` |
| 22 | `MAP_NEW` | New map management (navigation maps) | `defpackage/MapNew` |
| 23 | `ROUTE_BOOK` | Electronic road books | `RouteBook` |
| 24 | `STAGE` | Segments/stages | `Stage` |
| 25 | `TRAINING_PLAN` | Training schedule | `TrainPlan` |
| 26 | `ALG_GOMORE` | GoMore training metrics | `AlgGmData` |
| 27 | `GROUP_RIDE` | Group riding | `GroupRide` |
| 28 | `CLOUD_SWITCH_DEV` | Cloud device switch | `CloudSwitchDev` |
| 29 | `MUSIC_CTL` | Music control (head unit ↔ phone) | `Media` |
| 30 | `WATCH_CONTROL` | Watch control | `AppWatch` |
| 81 | `WATCH_CONFIG` | Watch settings | `WatchConfig` |
| 83 | `WATCH_HEALTH` | Watch health data | `WatchHealthData` |
| 84 | `WATCH_ALG_ACQUISITION` | Watch algorithm data | — |
| 85 | `WATCH_DIAL` | Watch faces | `WatchDial` |
| 86 | `SPORT_PAGE_FILED` | Watch sport page fields | `SportPageField` |
| 87 | `SPORT_ABILITY` | Sport ability metrics | `SportAbility` |

BSC200 is a **bike computer** (`DEVICE_CYCLE_COMPUTER`); services 1–27 are the
relevant set. Support for an individual feature is advertised by the device
record's `DeviceInformation.FunctionType` list (see §5.2), not hard-coded by
model name.

---

## 4. Operations per service

Header byte 4 is the primary operation. For services with a sub-operation,
byte 5 sometimes carries it; for a few services the sub-operation exists only
inside the protobuf payload (the factories leave byte 5 at `0xFF`). Response
matching often uses `second_operation`, so the device echoes the sub-operation
in the response header.

### 4.1 High-level table

| Service | Request operation | Second service / sub-op | Response operation |
|---|---|---|---|
| INS (1) | `Ins.INS_OPERATE_TYPE` | second_service = `Ins.INS_SERVICE_TYPE` (call/note/app) | same |
| MAP (2) | `Map.MAP_OPERATE_TYPE` | — | same |
| BACK (3) | `Back.BACK_OPERATE_TYPE` | second_service = `Back.BACK_SERVICE_TYPE` | same |
| FIRMWARE (4) | `Firmware.FIRMWARE_OPERATE_TYPE` | — | same |
| WIFI (5) | `Wifi.WIFI_OPERATE_TYPE` | — | same |
| CYCLING_DATA (6) | `CyclingData.CYCLING_DATA_OPERATE_TYPE` | — | same |
| ROUTE_PLAN (7) | `RoutePlan.ROUTE_PLAN_OPERATE_TYPE` | — | same |
| REAL_TIME_TRACE (8) | `RealTimeTrace.REAL_TIME_TRACE_OPERATE_TYPE` | — | same |
| USER_CONFIG (9) | `UserConfig.USER_CONFIG_OPERATE_TYPE` | — | same |
| BLE (10) | `Ble.BLE_OPERATE_TYPE` | — | same |
| FACTORY (11) | `Factory.FACTORY_OPERATE_TYPE` | — | same |
| CONFIG (12) | `Config.CONFIG_OPERATE_TYPE` | second_service = `Config.CONFIG_SERVICE_TYPE` | same |
| DEV_STATUS (13) | `DevStatus.DEV_STATUS_OPERATE_TYPE` | — | same |
| SENSOR (14) | `Sensor.SENSOR_OPERATE_TYPE` | — | same |
| TRAINING (15) | `Common.SERVICE_OPERATE_TYPE` | sub-op = `Training.TRAINING_SUB_OPERATE_TYPE` | main + sub-op |
| TEAM_INFO (16) | `TeamInfo.TEAM_INFO_OPERATE_TYPE` | — | same |
| DEV_VER_INFO (17) | `DevVerInfo.OPERATE_TYPE` | — | same |
| LANGUAGE (18) | `Common.SERVICE_OPERATE_TYPE` | sub-op = `LanguagePack.LANGUAGE_PACK_SUB_OPERATE_TYPE` | main + sub-op |
| LOG (19) | `Common.SERVICE_OPERATE_TYPE` | sub-op = `Log.LOG_SUB_OPERATE_TYPE` | main + sub-op |
| THEME (20) | `Theme.enum_operate` | — | same |
| FILE_OPERATION (21) | `Common.SERVICE_OPERATE_TYPE` | — | same |
| MAP_NEW (22) | `MapNew.map_op_type` | — | same |
| ROUTE_BOOK (23) | `Common.SERVICE_OPERATE_TYPE` | sub-op = `RouteBook.ROUTE_BOOK_SUB_OP_TYPE` | main + sub-op |
| STAGE (24) | `Stage.STAGE_DATA_OPERATE_TYPE` | — | same |
| TRAINING_PLAN (25) | `TrainPlan.TRAINING_PLAN_OPERATE_TYPE` | — | same |
| ALG_GOMORE (26) | `AlgGmData.LIB_GM_OPERATE_TYPE` | — | same |
| GROUP_RIDE (27) | `GroupRide.group_ride_msg_type` | — | same |
| MUSIC_CTL (29) | `Media.REMOTE_CMD` | — | same |

### 4.2 Core operation values (BSC200-relevant)

**DEV_VER_INFO (17)** — `DevVerInfo.OPERATE_TYPE`

| Value | Name | Direction |
|---:|---|---|
| 0 | `NONE` | — |
| 1 | `SET` | app→device |
| 2 | `GET` | app→device |
| 3 | `SEND` | device→app |

Payload: `dev_ver_info_msg { service_type, operate_type, version_msg }`.
`version_msg` fields: `main_boot_ver=1`, `main_app_ver=2`,
`ble_boot_ver=3`, `ble_app_ver=4`, `hardware_ver=5`, `protocol_ver=6`,
`compile_time=7`.

**DEV_STATUS (13)** — `DevStatus.DEV_STATUS_OPERATE_TYPE`

| Value | Name | Use |
|---:|---|---|
| 0 | `NONE` | — |
| 1 | `GET` | Poll current status |
| 2 | `SEND` | Device reports status |
| 3 | `ADD_POSITION_POINT` | Add navigation waypoint |
| 4 | `DELETE_POSITION_POINT` | Delete waypoint |
| 5 | `MODIFY_POSITION_POINT` | Modify waypoint |

Payload: `dev_status_msg { service_type=1, op_type=2, dev_cycling_status_msg=3, dev_gps_msg=4, rt_data_msg=5, wifi_status=6, navi_status=7, position_point_manager=8 }`.

`rt_data_message` fields (all int32, raw units):

| # | Field | Meaning |
|---:|---|---|
| 1 | `real_time_speed` | current speed |
| 2 | `avg_speed` | average speed |
| 3 | `riding_time` | ride time |
| 4 | `riding_distance` | distance |
| 5 | `real_time_cad` | cadence |
| 6 | `real_time_hrm` | heart rate |
| 7 | `avg_hrm` | average HR |
| 8 | `real_time_power` | power |
| 9 | `total_height` | total ascent |
| 10 | `avg_rise` | average grade |
| 11 | `cur_height` | current elevation |
| 12 | `cur_slope` | current slope |
| 13 | `max_speed` | max speed |
| 14 | `max_hrm` | max HR |
| 15 | `avg_cad` | average cadence |
| 16 | `max_cad` | max cadence |
| 17 | `avg_power` | average power |
| 18 | `max_power` | max power |
| 19 | `course` | heading |

Enums: `DEV_CYCLING_STATUS` 0=free, 1=doing, 2=pause;
`DEV_WIFI_STATUS` 1=idle, 2=map, 3=firmware, 4=upload, 5=set;
`DEV_NAVI_STATUS` 0=off, 1=on.
Waypoints: `dev_position_point_group { position_point_num=1,
position_info[]=2 }`, and `dev_position_point { latitude=1, longitude=2,
pointname=3, point_index=4 }` (integer lat/lon in the group form).

**USER_CONFIG (9)** — `USER_CONFIG_OPERATE_TYPE` 0=NONE, 1=SET, 2=GET.

`user_config_msg { service_type=1, operate_type=2, user_config_data_message=3 }`
and `user_config_data_msg { sex=1, weight=2, age=3, height=4, wheel_dia=5,
bike_weight=6, time_zone=7, member_id=8 }`.

**CONFIG (12)** — second service selects the settings block:

| Value | `CONFIG_SERVICE_TYPE` | Payload block |
|---:|---|---|
| 0 | `NONE` | — |
| 1 | `USER` | `user_data_msg` |
| 2 | `PAGE` | `page_msg[]` + `page_status_message` |
| 3 | `BIKE` | `bike_msg[]` |
| 4 | `UNIT` | `unit_msg[]` |
| 5 | `LANG` | `language_msg` |
| 6 | `KEY` | `key_set_msg[]` |
| 7 | `WHEEL` | `bike_msg` wheel data |
| 16 | `GPS` | `config_msg` |
| 17 | `SOUND` | `sound_set_msg[]` |
| 18 | `POWER` | `config_msg` |
| 19 | `DIS_COLOR` | `config_msg` |
| 20 | `BK` | `backlight_msg` |
| 48 | `ALARM` | `alarm_msg[]` |
| 49 | `LAP` | `lap_msg[]` |
| 50 | `AUTO` | `auto_set_msg[]` |
| 51 | `MODE` | `mode_msg[]` |
| 52 | `ALTITUDE` | `config_msg.altitude` |
| 53 | `DATA` | `data_msg[]` |
| 54 | `MODE_TEMPLATE` | `mode_msg[]` |

`CONFIG_OPERATE_TYPE`: 0=NONE, 1=SET, 2=GET, 3=SEND, 4=ADD, 5=DEL,
6=GET_MODULE_INFO, 7=CTRL, 8=SET_DEFAULT.

`page_msg` (pixel-LCD page; BSC200 uses this): `page_index=1`, `status=2`,
`data=3` (repeated int32 field IDs), `data_site=4`, `page_name=5`,
`page_type=6` (`PAGE_TYPE`), `page_mode=7`, `main_page_status=8`,
`line_width=9`, `graphic_display=10`, `forbidden_close=11`.
`cur_page_status_msg`: `page_line_num_max=1`, `page_line_data_max=2`,
`data_page_num_max=3`, `unsupported_data=4`, `lcd_type=5`,
`support_page_layout=6`, `main_page_open=11`.
`PAGE_TYPE`: 0 invalid, 1 data, 2 elevation, 3 map, 4 area, 5 training course,
6 training FEC, 7 compass, 8 lap, 9 roadbook, 10 climb, 11 AI partner,
12 segment, 13 fitness, 14 climb map, 15 group ride, 16 music, 17 camera.
`LCD_TYPE`: 1 pixel LCD, 2 segment LCD.

**SENSOR (14)** — `SENSOR_OPERATE_TYPE`: 0=NONE, 1=GET, 2=SET, 3=DEL,
4=CONNECT, 5=SEND, 6=EXIT, 7=POW_CALIB, 8=SYNC, 9=QUERY.

`sensor_message { service_type=1, sensor_operate_type=2, sensor_data_msg[]=3 }`;
`sensor_data_message` fields: `sensor_type=1`, `sensor_radio_type=2`,
`sensor_status_type=3`, `sensor_key=4`, `sensor_ble_name=5`, `sensor_rssi=6`,
`sensor_pwr=7`, `wheel_size=8`, `crank_length=9`, `sensor_forbidden=10`,
`auto_wheel_size=11`, `radar_sensor_set_msg=12`, `di2_sensor_set_msg[]=13`.
`SENSOR_TYPE`: 0 invalid, 1 HRM, 2 combined speed/cadence, 3 pedal power,
4 other power, 5 cadence, 6 speed, 7 shifter, 8 Di2, 9 FEC, 10 LEV, 11 RD,
12 radar, 13 light, 14 core temp, 15 camera.
`SENSOR_RADIO_TYPE`: 1 BLE, 2 ANT. `SENSOR_STATUS_TYPE`: 0 saved,
1 connected, 2 not saved.

**CYCLING_DATA (6)** — `CYCLING_DATA_OPERATE_TYPE`: 0=NONE, 1=LIST_GET,
2=LIST_SEND, 3=FILE_GET, 4=FILE_SEND, 5=FILE_DEL, 6=ALL_DEL,
7=AUTO_UPLOAD, 8=LIST_NUM_GET.

`cycling_data_msg { service_type=1, operate_type=2, file_flag_msg[]=3,
file_content=4 (bytes), auto_upload_msg=5, list_msg=6 }`.
`cycling_data_file_flag_message`: `timestamp=1`, `file_size=2`,
`user_id=3`, `device_id=4`.

**ROUTE_PLAN (7)** — `ROUTE_PLAN_OPERATE_TYPE`: 0=NONE, 1=LIST_GET,
2=LIST_SEND, 3=FILE_DEL, 4=FILE_SEND, 5=FILE_USE, 6=FILES_DEL,
7=LIST_NUM_GET, 8=RENAME, 9=TURN_PROMPT, 10=NOTIFY_TRAJECTORY_DEVIATION,
11=ROUTE_POINT_GET, 12=ROUTE_REROUTE_STATUS,
13=REQUEST_WHETHER_SUPPORT_FUNCTION, 14=SUPPORT_FUNCTION_ENABLE_STATUS.

`route_plan_data_msg { service_type=1, operate_type=2, line_id[]=3,
file_content=4, route_plan_info_msg[]=5, reroute_point_msg=6,
reroute_status=7, support_function_status=8, route_list_get_msg=12 }`.
`route_plan_info_message`: `id=1`, `file_type=2`, `name=3`,
`total_distance=4`, `longitude_start=5`, `latitude_start=6`, `status=7`.
`FILE_TYPE`: 0 invalid, 1 CNX, 2 GPX, 3 FIT, 4 TCX, 5 XML.
`FILE_STATUS`: 0 invalid, 1 used, 2 unused.

**TRAINING (15)** and **FILE_OPERATION (21)** — workouts are uploaded as a
generic file (`FILE_OPERATION`, op `ADD`, header byte3 `0xAA`) with
`general_file_operation` metadata:

`general_file_operation` fields: `service_type=1`, `operate_type=2`,
`file_type=3` (`FILE_TYPE_*`), `file_size=4`, `file_id=5`, `file_name=6`,
`file_extension=7`, `file_md5=8`, `file_verify_val=9`, `is_temp=10`,
`extra_info=11`, `watch_photo_dial_extra_info=12`, `method_information=15`.
`file_extra_Info`: `distance=1`, `total_ascent=2`, `avg_slope=3`.
`file_transfer_method_information`: `resume_supported=1`,
`packet_ack_supported=2`, `resume_offset=3`.
`file_operation_type`: 0 general, 1 training, 2 route plan, 3 map, 4 theme,
5 firmware, 6 language, 7 AGPS, 8 route book, 9 stage, 10 power-on image,
11 schedule, 12 theme background, 13 group-ride avatar, 14 cloud-switch device,
15 group-ride route book, 16 watch dial, 17 yaw re-navigation book,
18 watch photo dial.

The TRAINING service carries workout metadata (`training_message`), with
`TRAINING_SUB_OPERATE_TYPE`: 0 invalid, 1 SET_USE, 2 SET_NAME, 3 SET_REMIND,
17 GET_MODULE_INFO, 18 GET_LIST, 19 GET_LIST_NUM, 33 ADD_FILE, 49 DEL_FILE.

**BACK (3)** — weather/time/AGPS/location; second service
`BACK_SERVICE_TYPE`: 1 MAIN, 2 WEATHER, 3 AIR_PRESSURE, 4 ELEVATION,
5 EPHEMERIS, 6 TIME, 7 LOCATE_INFO. Operation: 1 GET, 2 SEND.
Payload `back_msg` fields: `three_days_msg[]=4`, `cur_msg=5`,
`three_hours_msg[]=6`, `air_pressure_msg=7`, `ephemeris_data_msg=8`,
`time_msg=9`, `locate_info_msg=10`.

**FIRMWARE (4)** — `FIRMWARE_OPERATE_TYPE`: 0 NONE, 1 GET_VERSION,
2 SEND_VERSION, 3 MCU_UPDATE, 4 PROGRESS, 5 BLE_UPDATE.
Payload `firmware_msg { service_type=1, operate_type=2, firmware_data_msg=3 }`
with `firmware_data_message`: `mcu_firmware_ver=1`, `ble_firmware_ver=2`,
`firmware_size=3`, `url=4`, `process=5`, `ble_boot_firmware_ver=6`.

**REAL_TIME_TRACE (8)** — 0 NONE, 1 GET, 2 FIT, 3 FINISH, 4 END.
`real_time_trace_msg { service_type=1, operate_type=2, fit_msg=3 }`.

**BLE (10)** — see §5.1.

**INS (1)** — notification forwarding. `INS_OPERATE_TYPE`: 0 NONE, 1 CTRL,
2 INCOMING_CALL, 3 ANSWER_CALL, 4 REJECT_CALL, 5 CHECK_CALL,
6 INCOMING_NOTE, 7 CHECK_NOTE, 8 GET_SWITCH_STATUS, 9 SET_APP_LIST,
10 DEVICE_REJECT_CALL, 11 DEVICE_MUTE_CALL, 12 FIND_START, 13 FIND_STOP.
`INS_SERVICE_TYPE`: 0 NONE, 1 MAIN, 2 CALL, 3 NOTE, 4 FIND_PHONE,
5 FIND_WATCH.
`ins_msg { service_type=1, ins_service_type=2, ins_operate_type=3,
ins_data_msg=4, config=5, ancs_filter_msg=6 }`.
`ins_data_message`: `tel_num=1` (UTF-8 bytes), `name=2`, `content=3`,
`time=4`, `uid=5`, `count=6`, `app_name=7`, `is_app=8`, `pairing_code=9`,
`apk_name=10`.

**FILE_OPERATION (21)** and **MAP_NEW (22)** — see §6/§7.

**LOG (19)** — main op `Common.SERVICE_OPERATE_TYPE` (GET/SET/ADD/DEL),
sub-op `LOG_SUB_OPERATE_TYPE`: 0 invalid, 16 NUMBER_GET, 17 GET (file),
18 DEBUG_LOG_PACK_REQUEST, 19 DEBUG_LOG_PACK_COMPLETE.
`log_msg` carries `service=1`, `operate=2`, `sub_operate=3`, `log_type`,
`log_name`, `content` (see `protobuf/Log.md`).

**LANGUAGE (18)** — sub-op `LANGUAGE_PACK_SUB_OPERATE_TYPE`: 0 invalid,
1 SET_DOWNLOAD_URL, 16 GET_MODULE_INFO, 17 GET_LIST, 32 ADD_PACK, 48 DEL_PACK.
Payload `language_pack_msg` (see `protobuf/LanguagePack.md`).

**THEME (20)** — `Theme.enum_operate`: 0 invalid, 1 LIST_GET, 2 SET, 3 USE,
4 DEFAULT_BG, 5 GET_BG.

**SYNC / server-initiated** — the device can also initiate a request (header
type 1 or 3) for weather, AGPS, time sync, or location, handled by callbacks
such as `onNeedWeather`, `onDeviceNeedEphemeris`, `onNeedLocation`,
`onTimeCalibration` in `IGPDeviceManager`. The app answers with `BACK` (3)
`SEND` frames. Expect these without a preceding app request.

---

## 5. Connection, binding and discovery

### 5.1 Binding (BLE service 10)

`BLE_OPERATE_TYPE`: 0 NONE, 1 BOND_INFO, 2 BOND_REQ, 3 CONNECT_STATUS,
4 UNBOND, 5 WATCH_BOND_USER_GET, 7 WATCH_BOND_USER_SET,
8 WATCH_BOND_STATE_UPDATE, 9 BOND_STATUS_GET.

`ble_msg` fields: `service_type=1`, `ble_operate_type=2`,
`ble_data_msg[]=3` (`status=1`, `member_id=2`),
`ble_bond_user_msg=4` (`user_id=1`, `state=2`, `bond_allow=3`),
`ble_bond_status_msg=5` (`bond_status=1`).

Typical flow (from `PairBindingDelegate`):

1. **Read bond info** — op `BOND_INFO (1)`; response `ble_data_msg[]` lists
   bound member IDs and their status.
2. **Check bound** — op `BOND_STATUS_GET (9)`; `bond_status == 1` means bound.
3. **Bind** — for Android 8+ the app calls the OS `createBond()`; for older
   Android it sends op `BOND_REQ (2)`. A confirmation status 0 means accepted.
4. **Report app state / user** — op `CONNECT_STATUS (3)` with
   `ble_data_message { status, member_id }`, or for watches
   `WATCH_BOND_USER_SET (7)` with `watch_ble_bond_user_message { user_id }`.
5. **Unbind** — op `UNBOND (4)`.

`DeviceInfo.connectionMode` decides whether step 3 is used at all (§1.3).
`getAllowPairing()` (op 5) returns `bond_allow` (1 = allowed).

### 5.2 Device feature discovery

The app does **not** assume BSC200 capabilities from the name. The server
returns a base64 protobuf `DeviceInformation.DeviceInfo` (`pbData`) in the
scan/device record, and the app reads:

* `generation` (1, 2, 3, 4) — framing generation
* `send_file_mtu_size`
* `data_receive_timeout`
* `function_type_list` — capability bitmask/list
* `connection_mode`, `pair_mode`, `is_support_auth`
* `device_category_type`

`FunctionType` values relevant to BSC200 include:
`DATA_MANAGEMENT_*` (0–5), `DEVICE_SETTINGS_MODULE (6)`, `NOTIFICATION_*`
(31–37), `ROUTES_MODULE (161)`, `SENSORS_MODULE (201)`,
`PAGE_SETTING_MODULE (281)`, `TRAINING_MODULE (311)`,
`BIKE_SETTING_MODULE (341)`, `DISPLAY_SETTING_MODULE (521)`,
`ALARM_*`, `LAP_SET_SUB_MODULE (107)`, `SOUND_MODULE (108)`,
`AUTO_*`, `READ_SN_FUNCTION (549)`, `DEVICE_LOG (3008)`, and so on.
The full list is in [`protobuf/DeviceInformation.md`](protobuf/DeviceInformation.md).
If a capability is absent, the corresponding service/operation may still be
implemented but is not exposed by the app; the device can answer with
`UnsupportedCommand (6)`.

`DeviceConstant` also notes BSC200-specific handling:

* BSC200 is **BLE-only** (no Wi-Fi).
* BSC200 supports **MTU 247**.
* BSC200 uses the **pixel-LCD page settings** family together with BSC200C,
  BSC300, BSC300T, BSC200S (`PageConfigUtil.PIXEL_LCD_NAME_LIST`), with
  4 rows per page (`DEVICE_TOTAL_ROWS_NUM_MAP`).
* BSC200/BSC200C support **gear-of-brightness** only (not percentage
  brightness).

### 5.3 Post-connect handshake

After GATT service discovery, `IGPDeviceManager.initCharacteristic()`:

1. Enables notifications on every available TX characteristic (channels 1–4,
   CSC, battery, HR).
2. Starts the device-ready strategy (`CycleComputerReadyStrategy`):
   * pair/bind if `connectionMode` requires it;
   * read firmware versions (`DEV_VER_INFO`, op `GET`) and update the feature
     list;
   * read serial number if `READ_SN_FUNCTION` is present;
   * read device name / manufacturer via standard GATT.
3. The app then reads settings on demand (pages, units, bike, alarms, etc.)
   and starts the live status poll when a ride/tracking screen is active.

A robust third-party client should at minimum:

```
connect
request MTU 247
enable notifications on all TX characteristics
send DEV_VER_INFO GET (service 17, op 2)
send BLE BOND_STATUS_GET (service 10, op 9) if binding is relevant
```

---

## 6. File transfer

There are two different file layouts: a generic upload path and a download
path. The download layout depends on the device generation
(`IGPDevice.getGeneration()`).

### 6.1 Generic upload (routes, workouts, firmware, maps, language packs)

Used for app→device files on new-SDK devices (generation 2+, including
BSC200). The app sends one logical blob, split across BLE writes by the MTU.
Format (`RoutePlanningDelegate.sendRoutePlanFileSingleChannel`,
`FirmwareUpdateDelegate.sendFirmwareFileToDevice`):

```
┌──────────────────────────────────────────────┐
│ 20-byte header (type 1)                       │
│   main_service = 21 (FILE_OPERATION)          │
│   main_operation = 3 (ADD)                    │
│   byte3 = 0xAA  (PktConstants.HEAD_CODE)      │
│   length = 0 (the 4-byte length below is used)│
│   byte 19 = CRC-8 of header[0..18]            │
├──────────────────────────────────────────────┤
│ u32 big-endian: length of the protobuf        │
├──────────────────────────────────────────────┤
│ general_file_operation protobuf               │
│   file_type, file_size, file_name,            │
│   file_extension, file_md5, is_temp, ...      │
├──────────────────────────────────────────────┤
│ raw file bytes                                │
└──────────────────────────────────────────────┘
```

Important details:

* `byte3 = 0xAA` (`HEAD_CODE`) distinguishes a file upload from a normal
  protobuf command. The header is built with a placeholder CRC and then
  recomputed after setting `byte3` (`BaseFactory.updateCRC`).
* The 4-byte protobuf length is **big-endian** (`writeInt32BE`).
* Because the header is built with an empty protobuf, its `data_size` and
  `payload_crc` are 0. File integrity is carried in the metadata
  (`file_md5`, `file_verify_val`; generation-4 downloads also carry
  `file_crc32`).
* This is the modern (generation 3/4) path. Older generation 1/2 devices use
  the legacy chunked route-upload path in `RoutePlanningDelegate`; BSC200 is
  handled by the new-SDK code paths in this app (pixel-LCD page settings,
  MTU 247), so try the layout above first.
* For firmware, an optional per-block sub-package header is inserted before
  each raw block when `packet_ack_supported` is negotiated:

  ```
  5A A5 C3 3C | CRC16(block) LE16 | block_length LE16 | block_index LE32 | block bytes
  ```

  `CRC16` is the SDK's nibble implementation (§8). The first block also has
  the 20-byte header + 4-byte metadata length + `general_file_operation`
  metadata; later blocks are only the sub-package header + bytes.
* Successful completion is acknowledged by a type-2 frame on the control
  channel with `main_service = 21`, `main_operation = 3`, status 0. Route
  uploads may instead return `main_service = 7`, `main_operation = 4`
  (`FILE_SEND`) with status 0.
* Real BSC200 firmware may send **no** completion ACK. In that case verify
  the upload by re-reading the route/activity list and checking the file ID
  (see §11.5).
* The app never retries a file upload (`retryTime = 0`).

### 6.2 Activity download (device→app)

**Generation 2/3** (`SyncActivityDelegate.checkActivityFitFile`):

* The device sends a sequence of notifications, each carrying a 20-byte header
  + a `cycling_data_msg` with `file_content`.
* The app verifies `header.data_size` and `header.payload_crc` against the
  accumulated payload.
* For each block it writes a type-2 confirm on the **control channel**:

  ```
  02 06 FF FF 04 FF FF <status> FF*11 <crc8>
  ```

  * `status = 0`: ACK / continue (block accepted)
  * `status = 1`: resend this block (CRC/size mismatch)
* `end_type == 3` marks the last block. On a successful last block the app
  returns success without writing another confirm. If the final size or file
  ID does not match, it writes a status-0 confirm and restarts the transfer.

**Generation 4** (`BaseCommand.getFileInfoAndFilePair`):

* The device streams one logical file. The first notification carries a
  20-byte header (service 6, operation 4 `FILE_SEND`; the app does not require
  a particular `end_type` here, but a complete file normally uses `3`)
  followed by:

  ```
  u32 big-endian: protobuf length
  FileDownload.file_download {
      file_size = 1, file_type = 2, file_id = 3,
      file_name = 4, file_verify_val = 7, file_crc32 = 8
  }
  raw file bytes
  ```

* `FileDownload.file_download` is the metadata; `file_crc32` and
  `file_verify_val` allow integrity checking. The app accumulates all
  notifications until `isReceiveFinish` sees the header, then parses the whole
  payload. `BaseCommand.getDownloadFileSize()` computes the expected total as
  `pb_size + 24 + file_size` (20-byte header + 4-byte length + protobuf +
  file).
* On gen-4 hardware the request must be sent on **channel 3** with header
  **byte 3 = `0x55`**; otherwise the device only ACKs and sends no data.
  Only append notifications received on channel 3. The period `BACK` frame
  on channel 2 would otherwise be spliced into the FIT file (§11.4).

### 6.3 Live tracking

`REAL_TIME_TRACE` (8) start/stop is initiated with operation `GET (1)`; the
device then sends `FIT (2)` frames carrying FIT file fragments and `FINISH (3)`
/ `END (4)` at the end. The app exposes the fragments as
`onRealTimeTrackingDataReceived` and `onRealTimeTrackEnd`.

### 6.4 Firmware update

Two paths exist:

1. **Proprietary file update** — `FIRMWARE` service, op `MCU_UPDATE (3)` or
   `BLE_UPDATE (5)`, plus `FILE_OPERATION` upload of the firmware image with
   `file_type = 5`, optional resume and sub-package ACKs.
2. **Nordic Secure DFU** — send `EnterDFU` (`FIRMWARE`, op `BLE_UPDATE`,
   no data), then connect to the DFU service and run the Nordic DFU procedure
   (`DfuService extends DfuBaseService`).

Progress/status is reported through `FIRMWARE` op `PROGRESS (4)`.

### 6.5 Map download

Maps use `MAP_NEW` (22) for navigation maps on newer devices and legacy
`MAP` (2) for older ones. `MAP_NEW.map_op_type`: 0 invalid, 1 LIST_NUM_GET,
2 LIST_GET, 3 DOWNLOAD, 4 UPDATE, 5 DELETE, 6 SET. The map list is a
`map_new_msg` with `map_list_msg[]` (`map_id=2`, `map_country_name=1`,
`map_coordinates=3`) and `map_list_get_msg=5`. Map files themselves are
downloaded by the device from a URL (Wi-Fi) or uploaded via file transfer.

### 6.6 Native CNX route file (GPX import)

BSC200 route files are XML. The encoding below is from native iGPSPORT files and
two independent reverse-engineered converters; `bsc200-tool` implements it in
`src/cnx.rs` with a round-trip self-test.

```xml
<?xml version='1.0' encoding='UTF-8' standalone='yes' ?>
<Route>
  <Id>12345678</Id>                  <!-- 8-digit numeric, content-derived -->
  <Distance>50062.17</Distance>      <!-- metres, 2 decimals -->
  <Duration></Duration>
  <Ascent>882</Ascent>               <!-- metres, rounded -->
  <Descent>-876</Descent>            <!-- metres, negative -->
  <Encode>2</Encode>                 <!-- 2nd-order delta for lat/lon -->
  <Lang>0</Lang>
  <TracksCount>1390</TracksCount>
  <Tracks>lat,lon,ele_mm;Δ;Δ;…</Tracks>
  <Navs/>                            <!-- optional turn-by-turn; may be empty -->
  <Points>…</Points>                 <!-- optional POIs -->
  <PointsCount>0</PointsCount>
</Route>
```

`<Tracks>` codec:

* First element: `lat,lon,elevation_mm` — lat first, elevation in millimetres.
* Later triples:
  * lat/lon: first difference for the first point, then second-order delta,
    unit `1e-7°`.
  * elevation: first-order delta in centimetres.
  * Quantise positions onto the `1e-7°` grid before differencing to avoid drift.
* Trailing semicolon is present.

`<Points>` entries are:

```xml
<Point><Lat>45.4460729</Lat><Lng>10.9774572</Lng><Type>7</Type><Descr>Water</Descr></Point>
```

`Type` is the iGPSPORT POI enum (0 waypoint, 1 sprint, 2–6 climbs, 7 supply,
8 garbage, 9 restroom, 10 service, 11 medical, 12 equipment, 13 shop,
14 meeting, 15 viewing, 16 Instagram, 17 tunnel, 18 valley, 19 dangerous road,
20 sharp turn, 21 steep slope, 22 intersection).

The device transcodes `.cnx` into `.tcnx`/`.tnav`/`.tclm`/`.tsgn` caches on
first use; deleting those caches is safe.

For a BLE upload, send the CNX bytes as a generic file upload
(`FILE_OPERATION`/`ADD`, `byte3 = 0xAA`, `file_type = ROUTE_PLAN` (2),
`file_extension = "cnx"`), then optionally activate with
`ROUTE_PLAN`/`FILE_USE` (`line_id = "<id>/cnx"`).

---

## 7. Common command recipes

Hex bytes below are complete **headers**; append the protobuf payload.
`crc` values are computed for the exact payload shown.

### 7.1 Read firmware versions

```
01 11 ff ff 02 ff ff 00 04 ad 01 ff ff ff ff ff ff ff ff ae
payload: 08 11 10 02
```

### 7.2 Poll device status

```
01 0d ff ff 01 ff ff 00 04 be 01 ff ff ff ff ff ff ff ff ec
payload: 08 0d 10 01
```

### 7.3 Set user profile

Service 9, op 1 (`SET`), payload `user_config_msg`:

```
08 09 10 01 1a <len> 08 <sex> 10 <weight> 18 <age> 20 <height>
28 <wheel_dia> 30 <bike_weight> 38 <timezone> 42 <member_id...>
```

### 7.4 Read page settings

Service 12, second service 2 (`PAGE`), op 2 (`GET`):

```
header: 01 0c 02 ff 02 ff ff <len_hi> <len_lo> <crc> 01 ff*8 <hcrc>
payload: config_msg { service_type=12, config_service_type=2,
                      config_operate_type=2 }
```

### 7.5 Add a route

1. Upload the route file with `FILE_OPERATION`/`ADD`, byte3 `0xAA`, metadata
   `file_type=2` (`ROUTE_PLAN`), `file_id`, `file_extension="cnx"`,
   `file_size`, `file_name`, optional `extra_info.distance` (×100).
2. On completion, the device confirms service 21/op 3 (or service 7/op 4).
   If no confirmation arrives, re-read the route list and verify that the
   uploaded `file_id` is present before reporting failure (§11.5).
3. Optionally activate it with `ROUTE_PLAN`, op 5 (`FILE_USE`) containing
   `line_id[]` = `<id>/<fileType>` on channel 4.

### 7.6 Delete all activities

`CYCLING_DATA` (6), op 6 (`ALL_DEL`), empty protobuf message with
`service_type=6, operate_type=6`.

### 7.7 Send an incoming call notification

`INS` (1), second service = `INS_SERVICE_TYPE` (e.g. call), op 2
(`INCOMING_CALL`), payload `ins_data_message` with `tel_num`, `name`,
`content`, `time`, `uid`, `count`, `is_app`.

### 7.8 Start notifications

`INS` (1), second service = `INS_SERVICE_TYPE`, op 1 (`CTRL`),
`ins_msg.config = 1` to enable, `0` to disable.

### 7.9 Enter DFU mode

`FIRMWARE` (4), op 5 (`BLE_UPDATE`), payload
`firmware_msg { service_type = 4, operate_type = 5 }` = `08 04 10 05`:

```
01 04 ff ff 05 ff ff 00 04 51 01 ff ff ff ff ff ff ff ff 1a  08 04 10 05
```

The device reboots into the Nordic Secure DFU bootloader. Firmware can also
be updated without DFU via the `FILE_OPERATION` firmware upload in §6.1.

---

## 8. CRC-16 used by firmware sub-packages

`CRC16Utils` is not a standard named CRC. It uses a 16-entry table and a
two-step nibble update:

```java
int i = (((crc >> 4) & 0xFF) ^ table[crc & 0x0F]) ^ table[b & 0x0F];
crc     = (((i   >> 4) & 0xFF) ^ table[i   & 0x0F]) ^ table[(b >> 4) & 0x0F];
```

```
table = 0000 CC01 D801 1400 F001 3C00 2800 E401
        A001 6C00 7800 B401 5000 9C01 8801 4400
```

Init 0, no final XOR; result is little-endian in the sub-package header.
Check value: `crc16("123456789") = 0x1404`. This CRC is **only** used in the
firmware sub-package header, not in the 20-byte frame header.

---

## 9. Protobuf payloads

All payloads are standard protobuf wire format (proto2 semantics: optional
scalar fields have presence, repeated fields are packed where the generated
code says so, strings are UTF-8, bytes are `bytes`).

The extracted schema for every message and enum is in
[`protobuf/`](protobuf/README.md). Most relevant files:

| File | Contents |
|---|---|
| [`protobuf/Common.md`](protobuf/Common.md) | `service_type_index`, `SERVICE_OPERATE_TYPE`, `file_list_get_message` |
| [`protobuf/DevVerInfo.md`](protobuf/DevVerInfo.md) | firmware/hardware versions |
| [`protobuf/DevStatus.md`](protobuf/DevStatus.md) | live ride data, GPS, navi state, waypoints |
| [`protobuf/Config.md`](protobuf/Config.md) | pages, units, bike, alarms, lap, auto, sound, keys, user |
| [`protobuf/UserConfig.md`](protobuf/UserConfig.md) | rider profile |
| [`protobuf/Sensor.md`](protobuf/Sensor.md) | sensors, Di2, radar |
| [`protobuf/CyclingData.md`](protobuf/CyclingData.md) | activities |
| [`protobuf/RoutePlan.md`](protobuf/RoutePlan.md) | routes, turn prompts, reroute |
| [`protobuf/Training.md`](protobuf/Training.md) | workouts/segments |
| [`protobuf/GeneralFileOperation.md`](protobuf/GeneralFileOperation.md) | file upload metadata |
| [`protobuf/FileDownload.md`](protobuf/FileDownload.md) | generation-4 download metadata |
| [`protobuf/Firmware.md`](protobuf/Firmware.md) | version/update messages |
| [`protobuf/Ble.md`](protobuf/Ble.md) | bonding |
| [`protobuf/Ins.md`](protobuf/Ins.md) | call/message notifications |
| [`protobuf/Back.md`](protobuf/Back.md) | weather, time, AGPS, location |
| [`protobuf/DeviceInformation.md`](protobuf/DeviceInformation.md) | device record and feature list |

`schema_raw.json` contains the same data in machine-readable form
(package → message → `[field_number, FIELD_NAME, type]` and enums).

### 9.1 Decoding tips

* A payload may be a bare message (e.g. `dev_ver_info_msg`) or a message with
  a `service_type`/operation field that mirrors the header. The header is
  authoritative for dispatch; the payload copy is informational.
* Unknown fields are normal and should be skipped (standard protobuf rules).
* Repeated scalar fields in these generated classes are **not** packed
  (proto2 default), except where the generated code declares `[packed=true]`;
  a generic protobuf decoder handles both.
* Enum values not listed in the extracted tables should be treated as unknown
  and preserved.

---

## 10. What is BSC200-specific vs. shared

The decompiled app is one binary supporting many iGPSPORT devices. The protocol
above is the shared **"new SDK"** protocol used by BSC200 and its siblings.

BSC200-specific facts found in the code:

* Model string is exactly `BSC200` (`DeviceConstant.BSC200`).
* It is in `BLE_ONLY_DEVICE_LIST` → no Wi-Fi hardware; ignore service 5.
* It is in `MTU247_SUPPORTED_DEVICES` → request MTU 247.
* It belongs to the **pixel-LCD / new page-setting** family
  (`BSC200`, `BSC200C`, `BSC300`, `BSC300T`, `BSC200S`), 4 rows per page,
  and is in `NEW_PAGE_SETTING_DEVICE_LIST`.
* `ONLY_SUPPORT_GEAR_OF_BRIGHTNESS_DEVICE` contains `BSC200` and `BSC200C` —
  brightness is a discrete gear, not a percentage.
* `ALWAYS_DISPLAY_PAGE_BSC200` = training FEC, training course, map pages.
* The app reads `generation` from the device record; it does not hardcode it.
  If you need to choose the write order (§10.1), read `generation` from
  `DeviceInformation.DeviceInfo` or observe one exchange.

### 10.1 Generation-dependent write order

The same logical frame is written differently depending on
`IGPDevice.getGeneration()`:

| Generation | Write sequence |
|---|---|
| 2, 3 | 1. write **payload** on the command's data characteristic (channel 1 or 4)<br>2. write **20-byte header** on control characteristic (channel 2) |
| 4 | write **header + payload** as one blob on channel 1 (status/settings) or channel 4 (files/config on newer devices) |

In both cases the receiving side sees header-then-payload conceptually; only
the BLE writes differ. Incoming notifications are always header + payload on
the TX characteristic.

For generation 4, the app also resets per-command file state and may use the
`4-byte length + FileDownload + bytes` download format (§6.2). For generation
2/3, expect the chunked `cycling_data_msg` + per-block confirm format.

---

## 11. BSC200 hardware validation notes

The following was observed on a real BSC200 (address `AA:BB:CC:DD:EE:FF`,
reported versions `BLE 144`, `proto 101`, `hw 100`, compile time
`Oct 21 2024 15:29:05`) using this repository's Rust implementation.

### 11.1 Connection and binding

* Only **gen-4 combined framing** was needed: header + payload in one BLE
  write on the command's data characteristic. The version request
  (`DEV_VER_INFO`, service 17/op 2) answered immediately; no gen-3 fallback
  was required on this unit.
* The device is **app-bound**. `BLE BOND_INFO (service 10, op 1)` returned a
  member ID (`REDACTED` in this case) and
  `CONNECT_STATUS (service 10, op 3)` with `status = 1` and the same member ID
  returned confirmation status 0. File operations worked after this
  handshake.
* The device has **no standard Battery GATT service** (`0x180F` is absent), so
  battery level cannot be read over GATT on this unit.
* The device periodically emits an unsolicited `BACK` notification on
  **channel 2**:

  ```
  03 03 05 ff 01 ff ff 00 ff … ff b3
  ```

  This is not file data and must be ignored during file transfers (see
  §11.4).

### 11.2 Observed channel usage

| Operation | Request channel | Response channel |
|---|---|---|
| Binding (`BOND_INFO`, `CONNECT_STATUS`) | 1 | 1 (ACK on 1) |
| Firmware version (`DEV_VER_INFO`) | 1 | 1 |
| Activity list (`CYCLING_LIST_NUM_GET` / `CYCLING_LIST_GET`) | 1 | 1 or 3 |
| Activity file download (`CYCLING_FILE_GET`) | 3 | 3 (stream) |
| Route list (`ROUTE_LIST_NUM_GET` / `ROUTE_LIST_GET`) | 4 or 1 | 1 or 4 |
| Route upload (`FILE_OPERATION` / `ADD`) | 4 | ACK optional |
| Route activation (`ROUTE_FILE_USE`) | 4 | 4 (ACK) |
| Unsolicited `BACK` | — | 2 |

Channel selection is firmware-timing dependent: activity-list requests that
timed out on channel 3 succeeded on channel 1, and route-list requests that
timed out on channel 4 succeeded on channel 1. A robust client should retry
the alternate channel rather than hard-code one.

### 11.3 Activity list and download

* `CYCLING_LIST_NUM_GET` (service 6, op 8) returned `file_num = 41`; the
  subsequent `CYCLING_LIST_GET` (service 6, op 1) with range `0..file_num-1`
  returned all 41 entries.
* The list `timestamp` field is **seconds since the FIT epoch**
  (1989-12-31T00:00:00Z), not Unix time. Add `631065600` to obtain a Unix
  timestamp (the Android app does this in `FitUtil.getDateFromFit`).
* The list timestamp is also **device-local wall-clock time** encoded this
  way: it is the FIT `record` start timestamp plus the device's UTC offset
  (for example +2 h in summer). Formatting `timestamp + 631065600` as UTC
  therefore recovers the time shown on the BSC200; formatting it in the
  PC's local timezone double-applies the offset. The FIT `record` timestamps
  inside the downloaded file are true UTC instants and are written to GPX
  unchanged.
* A gen-4 activity download is requested with `CYCLING_FILE_GET`
  (service 6, op 3) on **channel 3**, and the request header's **byte 3 must
  be `0x55`** (the app sets `mByteArray[3] = 85` in
  `SyncActivityDelegate.readActivityFitFile` before recomputing the header
  CRC). With `byte3 = 0xFF` the device only returned an ACK and no data.
* The response header is service 6, op 4 (`FILE_SEND`), byte 3 `0x55`,
  followed by `u32 BE protobuf length`, a `FileDownload` message and the raw
  FIT bytes. The expected total is `24 + pb_len + file_size`.
* The downloaded FIT file is a standard FIT file: the 14-byte header and the
  trailing CRC both validate. It still contains two semantic errors that
  strict viewers flag; see §11.7 for the required normalization.

### 11.4 Interleaved notifications corrupt naive downloads

During a long download the device interleaves its periodic 20-byte `BACK`
frame (channel 2) into the notification stream. A client that appends every
notification payload to the file will produce a FIT file with a 20-byte
frame spliced into it. Both `fitparser` and `fit-rs` then fail with
"no definition found for local message number …" at the splice point.

**Filter notifications by channel** and only append channel 3 payloads when
downloading a file. Once the `BACK` frames are removed, the FIT file parses
end-to-end with a normal FIT parser.

The same applies to the raw stream: the expected 20-byte frame starts with
`01 06 ff 55 04 ff ff …` and should not be confused with the interleaved
`02/03` frames.

### 11.5 Route upload and activation

* The gen-4 upload blob is written on **channel 4** as
  `header + u32 BE metadata length + general_file_operation + CNX bytes`.
* Completion ACKs for `FILE_OPERATION`/`ADD` are inconsistent. In several
  attempts this BSC200 sent no ACK within 30 seconds, but the route was
  present in the route list immediately afterwards; in another attempt an
  ACK arrived normally. Treat a missing ACK as "unverified" rather than
  failure and re-read the route list: success is confirmed by finding the
  uploaded `file_id`.
* Route activation with `ROUTE_PLAN`/`FILE_USE` (service 7, op 5) on channel 4
  returned a type-2 confirmation with status 0.
* The device route count may be one larger than the number of decoded
  `route_plan_info_msg` entries (one internal/temp entry has no ID), so
  compare by `file_id` rather than by list length.

### 11.6 FIT field interpretation for BSC200 records

For `record` messages (global message 20) the device uses:

| Field | Number | Scale / offset |
|---|---|---|
| `position_lat` | 0 | semicircles → `v * 180 / 2^31` |
| `position_long` | 1 | semicircles → `v * 180 / 2^31` |
| `altitude` | 2 | `v / 5 - 500` metres |
| `distance` | 5 | `v / 100` metres |
| `speed` | 6 | `v / 1000` m/s |
| `timestamp` | 253 | `v + 631065600` Unix seconds |

Heart rate (3), cadence (4), power (7) and temperature (13) are also present.
Altitude is invalid (`0xFFFF`) for the first point(s) of a recording; skip
rather than emitting `13107 m`. Applying these scales gives plausible
elevations (around 190–215 m for the test test rides) instead of raw field
values.

When exporting to GPX, the values are written as track-point extensions:

* `gpxtpx:hr`, `gpxtpx:cad`, `gpxtpx:speed`, `gpxtpx:atemp` in
  `http://www.garmin.com/xmlschemas/TrackPointExtension/v2`
* `gpxx:Watts` in
  `http://www.garmin.com/xmlschemas/ActivityExtension/v2`
* `bsc200:distance` in `urn:bsc200-tool:extensions:v1` (cumulative metres;
  GPX 1.1 has no standard distance element)

### 11.7 FIT message-order and `num_sessions` quirks

BSC200 FIT files (verified on several activities) contain two semantic errors
even though the header and trailing CRCs are valid:

1. **`sport` after `activity`.** The file ends with the `activity` data
   message (global message 34), followed by the `sport` definition and data
   message (global message 12). The FIT specification requires `activity` to
   be the final message. Strict viewers report "activity message is not
   last" or silently repair the file.
2. **`activity.num_sessions = 0`.** Field 1 of the activity message is written
   as `0`, although the file contains one `session` message (global message
   18). It should be the number of session messages (`1`).

A correct repair is:

1. Move any messages after the last activity data message to immediately
   before it.
2. Re-emit the activity definition message (in case the moved block redefined
   that local message type).
3. Patch `activity.num_sessions` to the number of `session` data messages in
   the file.
4. Recompute the header CRC and the trailing file CRC.

This produces a file that is byte-for-byte identical to the repaired file
returned by at least one online FIT viewer. The Rust implementation performs
this automatically for downloaded activities; see `fit::normalize` and the
`--repair-fit` CLI command.

---

## 12. Open items / caveats

* A few JADX methods could not be decompiled (notably
  `IGPDeviceManager.initCharacteristic$lambda$59`, `DeviceInfoHelper.getDeviceInfo`,
  `ReceivedDeviceDataCommand.storeData`, `CycleComputerReadyStrategy.toBindDevice`).
  The wire format above is reconstructed from all the surrounding code and the
  packet builders, which are fully readable.
* The exact receive-reassembly rule (when a notification starts a new frame
  vs. continues a file stream) is implemented in the unavailable
  `initCharacteristic$lambda$59`; the observable behaviour is consistent with:
  a notification beginning with a valid 20-byte header starts/updates a frame,
  otherwise it is appended to the current payload.
* Units for `rt_data_message` fields are not applied by the app (it only reads
  `wifi_status`), so they are documented as raw integers. Likely scales are
  0.01 for speed/distance and 1 for power/cadence/HR, but verify against a
  capture.
* `DeviceInfoHelper` maps older model names to generations; BSC200's generation
  is not hard-coded there and comes from the server-provided `DeviceInfo`.
* Services 28–30 and 81–87 are watch/cloud/music features; they are included
  in the tables for completeness but are not expected on a BSC200.
* If the device does not answer a service-17 `GET`, it may be a legacy
  generation-1 unit. Those use FIT-encoded commands built in
  `com.igpsport.blelib.utils.Command` (activity read/delete, device settings)
  rather than the 20-byte protobuf frame. BSC200 is not expected to behave
  this way, but it is a useful discriminator when capturing traffic from an
  unknown iGPSPORT model.
