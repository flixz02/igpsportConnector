# PeripheralConfig

_Generated from `com.igpsport.blelib.bean.PeripheralConfig`._

## Enums

### `PERIPHERAL_CONFIG_OPERATE`

| Name | Value |
|---|---:|
| `PCO_UNSPECIFIED` | 0 |
| `UNRECOGNIZED` | -1 |

### `PERIPHERAL_CONFIG_SERVICE`

| Name | Value |
|---|---:|
| `PCS_UNSPECIFIED` | 0 |
| `PCS_SHUTDOWN` | 4 |
| `PCS_SLEEP` | 5 |
| `PCS_LIGHT_MODE` | 6 |
| `PCS_DATA_BROADCAST` | 7 |
| `PCS_ALL_CONFIG` | 21 |
| `PCS_ALL_CONFIG_RESET` | 22 |
| `UNRECOGNIZED` | -1 |

### `enum_PERIPHERAL_CONFIG_LIGHT_MODE`

| Name | Value |
|---|---:|
| `PCLM_UNSPECIFIED` | 0 |
| `DEVICE_SCENE_OFF` | 4 |
| `DEVICE_SCENE_DEFAULT` | 5 |
| `DEVICE_SCENE_TEAM` | 6 |
| `DEVICE_SCENE_GRADIENT` | 7 |
| `DEVICE_SCENE_NIGHT` | 8 |
| `DEVICE_SCENE_DAYLIGHT` | 9 |
| `UNRECOGNIZED` | -1 |

## Messages

### `peripheral_config_data_broadcast`

| # | Field | Type |
|---:|---|---|
| 1 | `data_switch` | `bool` |

### `peripheral_config_data_shutdown`

| # | Field | Type |
|---:|---|---|
| 1 | `shutdown_switch` | `bool` |
| 2 | `countdown` | `int32` |

### `peripheral_config_data_sleep`

| # | Field | Type |
|---:|---|---|
| 1 | `sleep_switch` | `bool` |
| 2 | `countdown` | `int32` |

### `peripheral_config_format`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `PeripheralCommon.PERIPHERAL_SERVICE_TYPE` |
| 2 | `operate_type` | `PeripheralCommon.PERIPHERAL_OPERATE_TYPE` |
| 3 | `sub_service_type` | `PERIPHERAL_CONFIG_SERVICE` |
| 4 | `sub_operate_type` | `PERIPHERAL_CONFIG_OPERATE` |
| 5 | `message` | `peripheral_config_message` |

### `peripheral_config_light_mode`

| # | Field | Type |
|---:|---|---|
| 1 | `light_mode` | `enum_PERIPHERAL_CONFIG_LIGHT_MODE` |

### `peripheral_config_message`

| # | Field | Type |
|---:|---|---|
| 1 | `shutdown` | `peripheral_config_data_shutdown` |
| 2 | `sleep` | `peripheral_config_data_sleep` |
| 3 | `light` | `peripheral_config_light_mode` |
| 4 | `data_service` | `peripheral_config_data_broadcast` |
