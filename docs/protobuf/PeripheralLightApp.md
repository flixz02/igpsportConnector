# PeripheralLightApp

_Generated from `com.igpsport.blelib.bean.PeripheralLightApp`._

## Enums

### `BLE_LIGHT_CONFIG_SUP`

| Name | Value |
|---|---:|
| `BLCS_ROUTE` | 0 |
| `BLCS_RIDE_SYNC` | 1 |
| `BLCS_BRAKE_LIGHT` | 2 |
| `BLCS_AUTO_LIGHT` | 3 |
| `BLCS_AUTO_SLEEP` | 4 |
| `BLCS_SYNC_OFF` | 5 |
| `BLCS_TEAM_RIDE` | 6 |
| `BLCS_RADAR` | 7 |
| `BLCS_AUTO_START` | 8 |
| `BLCS_LUMEN_VARY` | 9 |
| `BLCS_ANGEL_VARY` | 10 |
| `BLCS_HL_BEAM` | 11 |
| `BLCS_AUTO_TURN` | 12 |
| `BLCS_AUTO_LOW` | 13 |
| `BLCS_SLP_VARY` | 14 |
| `BLCS_AUTO_LOWBAT` | 15 |
| `BLCS_INTELL_SAVING` | 16 |
| `BLCS_KNOCK_TRANS` | 17 |
| `BLCS_AUTO_LOOP` | 18 |
| `BLCS_STOP_FLASH` | 19 |
| `BLCS_AD_WARN` | 20 |
| `BLCS_RADAR_WARN` | 21 |
| `UNRECOGNIZED` | -1 |

### `BLE_LIGHT_CUSTOM_SUBTYPE`

| Name | Value |
|---|---:|
| `BLCS_CONSTANT` | 0 |
| `BLCM_FLICKER` | 1 |
| `BLCS_BREATH` | 2 |
| `UNRECOGNIZED` | -1 |

### `BLE_LIGHT_MODE`

| Name | Value |
|---|---:|
| `BLM_LIGHT_OFF` | 0 |
| `BLM_HIGH_ALWAYS` | 1 |
| `BLM_MID_ALWAYS` | 2 |
| `BLM_LOW_ALWAYS` | 3 |
| `BLM_HIGH_BLINK` | 4 |
| `BLM_LOW_BLINK` | 5 |
| `BLM_GRADIENT` | 6 |
| `BLM_HBEAM_HSTEADY` | 7 |
| `BLM_HBEAM_MSTEADY` | 8 |
| `BLM_HBEAM_LSTEADY` | 9 |
| `BLM_LBEAM_HSTEADY` | 10 |
| `BLM_LBEAM_MSTEADY` | 11 |
| `BLM_LBEAM_LSTEADY` | 12 |
| `BLM_ROTATION` | 13 |
| `BLM_LEFT_TURN` | 14 |
| `BLM_RIGHT_TURN` | 15 |
| `BLM_SUPERHIGH` | 16 |
| `BLM_SOS_WARNING` | 17 |
| `BLM_COMET_FLASH` | 18 |
| `BLM_WATERFALL_FLASH` | 19 |
| `BLM_PINWHEEL` | 20 |
| `BLM_SPECIAL_1` | 32 |
| `BLM_SPECIAL_2` | 33 |
| `BLM_SPECIAL_3` | 34 |
| `BLM_SPECIAL_4` | 35 |
| `BLM_SPECIAL_5` | 36 |
| `BLM_SPECIAL_6` | 37 |
| `BLM_SPECIAL_7` | 38 |
| `BLM_SPECIAL_8` | 39 |
| `BLM_SPECIAL_9` | 40 |
| `BLM_SPECIAL_10` | 41 |
| `BLM_CUSTOMIZE_1` | 64 |
| `BLM_CUSTOMIZE_2` | 65 |
| `BLM_CUSTOMIZE_3` | 66 |
| `BLM_CUSTOMIZE_4` | 67 |
| `BLM_CUSTOMIZE_5` | 68 |
| `BLM_CUSTOMIZE_6` | 69 |
| `BLM_CUSTOMIZE_7` | 70 |
| `BLM_CUSTOMIZE_8` | 71 |
| `BLM_CUSTOMIZE_9` | 72 |
| `BLM_CUSTOMIZE_10` | 73 |
| `BLM_CUSTOMIZE_11` | 74 |
| `BLM_CUSTOMIZE_12` | 75 |
| `UNRECOGNIZED` | -1 |

### `BLE_LIGHT_MODE_TYPE`

| Name | Value |
|---|---:|
| `BLMT_TYPE_NORMAL` | 0 |
| `BLMT_TYPE_CUSTOM` | 1 |
| `UNRECOGNIZED` | -1 |

### `BLE_LIGHT_OPERATE`

| Name | Value |
|---|---:|
| `BLO_OPERATE_SET` | 0 |
| `BLO_OPERATE_GET` | 1 |
| `BLO_OPERATE_NTC` | 2 |
| `BLO_OPERATE_ENABLE` | 3 |
| `BLO_OPERATE_DISABLE` | 4 |
| `UNRECOGNIZED` | -1 |

### `BLE_LIGHT_SERVICE`

| Name | Value |
|---|---:|
| `BLS_LIGHT_CFG` | 0 |
| `BLS_MODE_SUP` | 1 |
| `BLS_MODE_CUR` | 2 |
| `BLS_CUSTOME_MODE` | 3 |
| `BLS_SMT_CONFIG` | 4 |
| `BLS_LEFT_TIME` | 5 |
| `BLS_BAT_PCT` | 6 |
| `BLS_MODE_ENABLE` | 7 |
| `BLS_CONFIG_RESPECTIVE` | 8 |
| `BLS_RIDE_CFG` | 9 |
| `UNRECOGNIZED` | -1 |

### `BLE_LIGHT_TYPE`

| Name | Value |
|---|---:|
| `BLT_TAIL_LIGHT` | 0 |
| `BLT_FRONT_LIGHT` | 1 |
| `BLT_HEAD_LIGHT` | 2 |
| `BLT_LEFT_LIGHT` | 3 |
| `BLT_RIGHT_LIGHT` | 4 |
| `BLT_TIRE_LIGHT` | 5 |
| `BLT_PACK_LIGHT` | 6 |
| `BLT_PEDAL_LIGHT` | 7 |
| `BLT_FRAME_LIGHT` | 8 |
| `BLT_SPOKE_LIGHT` | 9 |
| `BLT_RADAR_LIGHT` | 10 |
| `BLT_TURN_LIGHT` | 11 |
| `BLT_FLASH_LIGHT` | 12 |
| `UNRECOGNIZED` | -1 |

### `BLT_SMT_CFG_STATUS`

| Name | Value |
|---|---:|
| `BSCS_CFG_OFF` | 0 |
| `BSCS_CFG_ON` | 1 |
| `BSCS_CFG_FOL` | 2 |
| `UNRECOGNIZED` | -1 |

### `BRAKE_LIGHT_INTENSITY`

| Name | Value |
|---|---:|
| `LOW` | 0 |
| `MEDIUM` | 1 |
| `HIGH` | 2 |
| `UNRECOGNIZED` | -1 |

## Messages

### `blt_bat_pct`

| # | Field | Type |
|---:|---|---|
| 13 | `bat_pct` | `int32` |

### `blt_cus_mode_arg`

| # | Field | Type |
|---:|---|---|
| 1 | `mode` | `BLE_LIGHT_MODE` |
| 2 | `selected` | `BLE_LIGHT_CUSTOM_SUBTYPE` |
| 3 | `cus_mode` | `repeated blt_cus_mode_cfg` |

### `blt_cus_mode_cfg`

| # | Field | Type |
|---:|---|---|
| 1 | `subtype` | `BLE_LIGHT_CUSTOM_SUBTYPE` |
| 2 | `lightness_map` | `repeated blt_spe_lightness` |
| 3 | `spe_cycle` | `spe_flicker_cycle` |
| 4 | `spe_ratio` | `spe_lighten_ratio` |

### `blt_cus_mode_get`

| # | Field | Type |
|---:|---|---|
| 1 | `mode` | `BLE_LIGHT_MODE` |

### `blt_cus_mode_modify`

| # | Field | Type |
|---:|---|---|
| 1 | `mode` | `BLE_LIGHT_MODE` |
| 2 | `subtype` | `BLE_LIGHT_CUSTOM_SUBTYPE` |
| 3 | `value` | `blt_light_value` |
| 4 | `cycle` | `int32` |
| 5 | `ratio` | `int32` |

### `blt_left_time`

| # | Field | Type |
|---:|---|---|
| 1 | `time` | `int64` |

### `blt_light_mode_cur`

| # | Field | Type |
|---:|---|---|
| 1 | `cur_mode` | `BLE_LIGHT_MODE` |

### `blt_light_self`

| # | Field | Type |
|---:|---|---|
| 1 | `light_type` | `BLE_LIGHT_TYPE` |
| 2 | `light_cnt` | `int32` |

### `blt_message_format`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `PeripheralCommon.PERIPHERAL_SERVICE_TYPE` |
| 2 | `operate_type` | `PeripheralCommon.PERIPHERAL_OPERATE_TYPE` |
| 3 | `blt_service_type` | `BLE_LIGHT_SERVICE` |
| 4 | `blt_operate_type` | `BLE_LIGHT_OPERATE` |
| 5 | `light_self` | `blt_light_self` |
| 6 | `mode_sup` | `repeated blt_mode_sup` |
| 7 | `cus_mode_get` | `blt_cus_mode_get` |
| 8 | `mode_arg` | `blt_cus_mode_arg` |
| 9 | `cfg_sup` | `repeated blt_smt_cfg` |
| 10 | `cfg_set` | `blt_smt_cfg` |
| 11 | `mode_set` | `blt_mode_enable` |
| 12 | `custom_cfg` | `blt_cus_mode_modify` |
| 13 | `cur_mode` | `blt_light_mode_cur` |
| 14 | `left_time` | `blt_left_time` |
| 15 | `bat_pct` | `blt_bat_pct` |
| 16 | `ride_cfg_all` | `ride_cfg_msg_all` |
| 17 | `ride_cfg_set` | `ride_cfg_msg` |

### `blt_mode_enable`

| # | Field | Type |
|---:|---|---|
| 1 | `mode` | `BLE_LIGHT_MODE` |
| 2 | `enable` | `bool` |

### `blt_mode_sup`

| # | Field | Type |
|---:|---|---|
| 1 | `mode` | `BLE_LIGHT_MODE` |
| 2 | `mode_type` | `BLE_LIGHT_MODE_TYPE` |
| 3 | `enable` | `bool` |
| 4 | `editable` | `blt_mode_editable` |

### `blt_smt_cfg`

| # | Field | Type |
|---:|---|---|
| 1 | `config` | `BLE_LIGHT_CONFIG_SUP` |
| 2 | `status` | `BLT_SMT_CFG_STATUS` |
| 3 | `user_data` | `cfg_user_data` |

### `blt_spe_lightness`

| # | Field | Type |
|---:|---|---|
| 1 | `light_num` | `int32` |
| 2 | `pct` | `int32` |

### `ride_cfg_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `config` | `BLE_LIGHT_CONFIG_SUP` |
| 2 | `status` | `BLT_SMT_CFG_STATUS` |
| 3 | `brake_cfg` | `brake_cfg_msg` |

### `ride_cfg_msg_all`

| # | Field | Type |
|---:|---|---|
| 1 | `ride_cfg` | `repeated ride_cfg_msg` |
