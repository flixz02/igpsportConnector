# PeripheralInfo

_Generated from `com.igpsport.blelib.bean.PeripheralInfo`._

## Enums

### `PERIPHERAL_INFO_MODEL_TYPE`

| Name | Value |
|---|---:|
| `PIM_UNSPECIFIED` | 0 |
| `PIM_HR70` | 1 |
| `UNRECOGNIZED` | -1 |

### `PERIPHERAL_INFO_OPERATE`

| Name | Value |
|---|---:|
| `PIO_UNSPECIFIED` | 0 |
| `UNRECOGNIZED` | -1 |

### `PERIPHERAL_INFO_SERVICE`

| Name | Value |
|---|---:|
| `PIS_UNSPECIFIED` | 0 |
| `PIS_MODEL` | 1 |
| `PIS_NAME` | 2 |
| `PIS_VERSION` | 3 |
| `PIS_ID` | 11 |
| `PIS_POWER` | 16 |
| `PIS_TEMPERATURE` | 17 |
| `PIS_MEMORY` | 18 |
| `PIS_ALL_INFO` | 31 |
| `UNRECOGNIZED` | -1 |

## Messages

### `peripheral_info_data_id`

| # | Field | Type |
|---:|---|---|
| 1 | `device_id` | `int32` |
| 2 | `manufacturer_id` | `int32` |

### `peripheral_info_data_memory`

| # | Field | Type |
|---:|---|---|
| 1 | `remain_internal_flash` | `int32` |
| 2 | `total_internal_flash` | `int32` |
| 3 | `remain_iexternal_flash` | `int32` |
| 4 | `total_external_flash` | `int32` |
| 5 | `remain_internal_ram` | `int32` |
| 6 | `total_internal_ram` | `int32` |
| 7 | `remain_external_ram` | `int32` |
| 8 | `total_external_ram` | `int32` |

### `peripheral_info_data_name`

| # | Field | Type |
|---:|---|---|
| 1 | `device_name` | `string` |
| 2 | `ble_name` | `string` |
| 3 | `manufacturer_name` | `string` |

### `peripheral_info_data_power`

| # | Field | Type |
|---:|---|---|
| 1 | `vol_cur` | `int32` |
| 2 | `percent` | `int32` |

### `peripheral_info_data_temperature`

| # | Field | Type |
|---:|---|---|
| 1 | `temp_cur` | `int32` |
| 2 | `temp_max` | `int32` |

### `peripheral_info_data_version`

| # | Field | Type |
|---:|---|---|
| 1 | `mcu_version` | `int32` |
| 2 | `app_version` | `int32` |
| 3 | `boot_version` | `int32` |
| 4 | `hard_version` | `int32` |
| 5 | `check_version` | `int32` |

### `peripheral_info_format`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `PeripheralCommon.PERIPHERAL_SERVICE_TYPE` |
| 2 | `operate_type` | `PeripheralCommon.PERIPHERAL_OPERATE_TYPE` |
| 3 | `sub_service_type` | `PERIPHERAL_INFO_SERVICE` |
| 4 | `sub_operate_type` | `PERIPHERAL_INFO_OPERATE` |
| 5 | `message` | `peripheral_info_message` |

### `peripheral_info_message`

| # | Field | Type |
|---:|---|---|
| 1 | `model` | `PERIPHERAL_INFO_MODEL_TYPE` |
| 2 | `name` | `peripheral_info_data_name` |
| 3 | `version` | `peripheral_info_data_version` |
| 11 | `id` | `peripheral_info_data_id` |
| 16 | `power` | `peripheral_info_data_power` |
| 17 | `temperature` | `peripheral_info_data_temperature` |
| 18 | `memory` | `peripheral_info_data_memory` |
