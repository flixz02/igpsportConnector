# PeripheralFirmware

_Generated from `com.igpsport.blelib.bean.PeripheralFirmware`._

## Enums

### `PERIPHERAL_FIRMWARE_OPERATE`

| Name | Value |
|---|---:|
| `PFO_UNSPECIFIED` | 0 |
| `PFO_MCU_UPDATE_START` | 1 |
| `PFO_MCU_UPDATE_PROCESS` | 2 |
| `PFO_MCU_UPDATE_END` | 3 |
| `PFO_MCU_UPDATE_ERROR` | 4 |
| `UNRECOGNIZED` | -1 |

### `PERIPHERAL_FIRMWARE_SERVICE`

| Name | Value |
|---|---:|
| `PFS_UNSPECIFIED` | 0 |
| `PFS_ENTER_DFU` | 1 |
| `PFS_MCU_UPDATE` | 2 |
| `PFS_FW_STATUS` | 3 |
| `UNRECOGNIZED` | -1 |

## Messages

### `peripheral_firmware_format`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `PeripheralCommon.PERIPHERAL_SERVICE_TYPE` |
| 2 | `operate_type` | `PeripheralCommon.PERIPHERAL_OPERATE_TYPE` |
| 3 | `sub_service_type` | `PERIPHERAL_FIRMWARE_SERVICE` |
| 4 | `sub_operate_type` | `PERIPHERAL_FIRMWARE_OPERATE` |
| 5 | `message` | `peripheral_firmware_message` |

### `peripheral_firmware_message`

| # | Field | Type |
|---:|---|---|
| 6 | `bin_packet_size` | `int32` |
| 7 | `bin_packet_data` | `bytes` |
| 8 | `bin_total_size` | `int32` |
