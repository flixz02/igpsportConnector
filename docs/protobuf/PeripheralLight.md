# PeripheralLight

_Generated from `com.igpsport.blelib.bean.PeripheralLight`._

## Enums

### `AUTO_CONFIG_TYPE`

| Name | Value |
|---|---:|
| `enum_AUTO_CONFIG_INVALID` | 0 |
| `enum_AUTO_CONFIG_FOLLOW` | 1 |
| `enum_AUTO_CONFIG_SELF` | 2 |
| `enum_AUTO_CONFIG_OFF` | 3 |
| `UNRECOGNIZED` | -1 |

### `FOLLOW_MODE_FLAG`

| Name | Value |
|---|---:|
| `enum_MODE_FLAG_INVALID` | 0 |
| `enum_MODE_FLAG_TRUE` | 1 |
| `enum_MODE_FLAG_FALSE` | 2 |
| `UNRECOGNIZED` | -1 |

### `PERIPHERAL_LIGHT_CONFIG_SUP`

| Name | Value |
|---|---:|
| `PLCS_INVALID` | 0 |
| `PLCS_ROUTE` | 1 |
| `PLCS_RIDE_SYNC` | 2 |
| `PLCS_BRAKE_LIGHT` | 3 |
| `PLCS_SYNC_LIGHT` | 4 |
| `PLCS_SYNC_SLEEP` | 5 |
| `PLCS_SYNC_OFF` | 6 |
| `PLCS_TEAM_RIDE` | 7 |
| `PLCS_RADAR` | 8 |
| `PLCS_END` | 31 |
| `UNRECOGNIZED` | -1 |

### `PERIPHERAL_LIGHT_CUSTOME_MODE`

| Name | Value |
|---|---:|
| `PLCM_UNSPECIFIED` | 0 |
| `PLCM_GRADIENT` | 1 |
| `PLCM_REGULAR_BLINK` | 2 |
| `PLCM_ALWAYS` | 3 |
| `UNRECOGNIZED` | -1 |

### `PERIPHERAL_LIGHT_MODE`

| Name | Value |
|---|---:|
| `PLM_UNSPECIFIED` | 0 |
| `PLM_HIGH_ALWAYS` | 1 |
| `PLM_MID_ALWAYS` | 2 |
| `PLM_LOW_ALWAYS` | 3 |
| `PLM_HIGH_BLINK` | 4 |
| `PLM_LOW_BLINK` | 5 |
| `PLM_GRADIENT` | 6 |
| `PLM_CUSTOMIZE` | 7 |
| `PLM_LIGHT_OFF` | 20 |
| `UNRECOGNIZED` | -1 |

### `PERIPHERAL_LIGHT_OPERATE`

| Name | Value |
|---|---:|
| `PLO_NSPECIFIED` | 0 |
| `UNRECOGNIZED` | -1 |

### `PERIPHERAL_LIGHT_SERVICE`

| Name | Value |
|---|---:|
| `PLS_NSPECIFIED` | 0 |
| `PLS_LIGHT_TYPE` | 1 |
| `PLS_LIGHT_MODE_SUP` | 6 |
| `PLS_LIGHT_CURRENT` | 7 |
| `PLS_LIGHT_CUSTOME` | 8 |
| `PLS_CUSTOME_SET` | 9 |
| `PLS_LIGHT_AUTO_CONFIG` | 14 |
| `PLS_SLEEP_FOLLOW` | 15 |
| `PLS_AUTOLIGHT_FOLLOW` | 16 |
| `PLS_SYC_OFF` | 17 |
| `PLS_REMAINING_TIME` | 18 |
| `PLS_LIGHT_CFG_SUP` | 19 |
| `UNRECOGNIZED` | -1 |

### `PERIPHERAL_LIGHT_TYPE`

| Name | Value |
|---|---:|
| `PLT_INVALID` | 0 |
| `PLT_TAIL_LIGHT` | 1 |
| `PLT_FRONT_LIGHT` | 2 |
| `PLT_HEAD_LIGHT` | 3 |
| `PLT_LEFT_LIGHT` | 4 |
| `PLT_RIGHT_LIGHT` | 5 |
| `PLT_TIRE_LIGHT` | 6 |
| `PLT_PACK_LIGHT` | 7 |
| `PLT_PEDAL_LIGHT` | 8 |
| `PLT_FRAME_LIGHT` | 9 |
| `PLT_SPOKE_LIGHT` | 10 |
| `PLT_RADAR_LIGHT` | 11 |
| `UNRECOGNIZED` | -1 |

## Messages

### `auto_low_cfg`

| # | Field | Type |
|---:|---|---|
| 1 | `status` | `bool` |
| 2 | `time_out` | `int32` |

### `auto_pwr_cfg`

| # | Field | Type |
|---:|---|---|
| 1 | `status` | `bool` |

### `custome_mode_config`

| # | Field | Type |
|---:|---|---|
| 1 | `mode` | `PERIPHERAL_LIGHT_CUSTOME_MODE` |
| 2 | `m_lightness` | `int32` |
| 3 | `p_lightness` | `int32` |
| 4 | `cycle` | `int32` |
| 5 | `ratio` | `int32` |

### `peripheral_auto_config_message`

| # | Field | Type |
|---:|---|---|
| 1 | `auto_light` | `AUTO_CONFIG_TYPE` |
| 2 | `brake_swtich` | `bool` |
| 3 | `team_ride` | `bool` |
| 4 | `auto_sleep` | `AUTO_CONFIG_TYPE` |
| 5 | `sleep_cnt` | `int32` |
| 6 | `sync_off` | `bool` |
| 7 | `auto_pwr` | `auto_pwr_cfg` |
| 8 | `auto_low` | `auto_low_cfg` |

### `peripheral_light_format`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `PeripheralCommon.PERIPHERAL_SERVICE_TYPE` |
| 2 | `operate_type` | `PeripheralCommon.PERIPHERAL_OPERATE_TYPE` |
| 3 | `sub_service_type` | `PERIPHERAL_LIGHT_SERVICE` |
| 4 | `sub_operate_type` | `PERIPHERAL_LIGHT_OPERATE` |
| 5 | `message` | `peripheral_light_message` |

### `peripheral_light_message`

| # | Field | Type |
|---:|---|---|
| 1 | `light_modes_support` | `repeated PERIPHERAL_LIGHT_MODE` |
| 2 | `cur_mode` | `PERIPHERAL_LIGHT_MODE` |
| 3 | `cus_mode` | `PERIPHERAL_LIGHT_CUSTOME_MODE` |
| 4 | `cus_config` | `custome_mode_config` |
| 5 | `auto_config` | `peripheral_auto_config_message` |
| 6 | `sleep_flag` | `FOLLOW_MODE_FLAG` |
| 7 | `autolight_flag` | `FOLLOW_MODE_FLAG` |
| 8 | `remain_time` | `int32` |
| 9 | `off_flag` | `FOLLOW_MODE_FLAG` |
| 10 | `light_type` | `PERIPHERAL_LIGHT_TYPE` |
| 11 | `light_cfg_sup` | `repeated PERIPHERAL_LIGHT_CONFIG_SUP` |
