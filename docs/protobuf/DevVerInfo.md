# DevVerInfo

_Generated from `com.igpsport.blelib.bean.DevVerInfo`._

## Enums

### `OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_OPERATE_TYPE_NONE` | 0 |
| `enum_OPERATE_TYPE_SET` | 1 |
| `enum_OPERATE_TYPE_GET` | 2 |
| `enum_OPERATE_TYPE_SEND` | 3 |
| `enum_OPERATE_TYPE_ADD` | 4 |
| `enum_OPERATE_TYPE_DEL` | 5 |

## Messages

### `dev_ver_info_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `operate_type` | `OPERATE_TYPE` |
| 3 | `version_message` | `version_msg` |

### `version_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `main_boot_ver` | `int32` |
| 2 | `main_app_ver` | `int32` |
| 3 | `ble_boot_ver` | `int32` |
| 4 | `ble_app_ver` | `int32` |
| 5 | `hardware_ver` | `int32` |
| 6 | `protocol_ver` | `int32` |
| 7 | `compile_time` | `string` |
