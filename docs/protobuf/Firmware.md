# Firmware

_Generated from `com.igpsport.blelib.bean.Firmware`._

## Enums

### `FIRMWARE_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_FIRMWARE_OPERATE_TYPE_NONE` | 0 |
| `enum_FIRMWARE_OPERATE_TYPE_GET_VERSION` | 1 |
| `enum_FIRMWARE_OPERATE_TYPE_SEND_VERSION` | 2 |
| `enum_FIRMWARE_OPERATE_TYPE_MCU_UPDATE` | 3 |
| `enum_FIRMWARE_OPERATE_TYPE_PROGRESS` | 4 |
| `enum_FIRMWARE_OPERATE_TYPE_BLE_UPDATE` | 5 |

## Messages

### `firmware_data_message`

| # | Field | Type |
|---:|---|---|
| 1 | `mcu_firmware_ver` | `int32` |
| 2 | `ble_firmware_ver` | `int32` |
| 3 | `firmware_size` | `int32` |
| 4 | `url` | `string` |
| 5 | `process` | `int32` |
| 6 | `ble_boot_firmware_ver` | `int32` |

### `firmware_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `firmware_operate_type` | `FIRMWARE_OPERATE_TYPE` |
| 3 | `firmware_data_msg` | `firmware_data_message` |
