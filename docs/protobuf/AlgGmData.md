# AlgGmData

_Generated from `com.igpsport.blelib.bean.AlgGmData`._

## Enums

### `LIB_GM_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `LIB_GM_OPERATE_TYPE_INVALID` | 1 |
| `LIB_GM_OPERATE_TYPE_GET` | 2 |

## Messages

### `pb_gm_format_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `operate_type` | `LIB_GM_OPERATE_TYPE` |
| 3 | `persistent_data` | `pb_gm_persistent_data` |

### `pb_gm_historical_data`

| # | Field | Type |
|---:|---|---|
| 1 | `records` | `repeated pb_gm_historical_record` |

### `pb_gm_historical_record`

| # | Field | Type |
|---:|---|---|
| 1 | `timestamp` | `int32` |
| 2 | `vo2max` | `int32` |
| 3 | `ftp` | `int32` |
| 4 | `map` | `int32` |
| 5 | `trainingload` | `int32` |
| 6 | `trainingstatus` | `int32` |
| 7 | `tltrend` | `int32` |

### `pb_gm_persistent_data`

| # | Field | Type |
|---:|---|---|
| 1 | `statistical` | `pb_gm_statistical_outputs` |
| 2 | `historical` | `pb_gm_historical_data` |

### `pb_gm_statistical_outputs`

| # | Field | Type |
|---:|---|---|
| 1 | `start_time` | `int32` |
| 2 | `end_time` | `int32` |
| 3 | `recoverytime` | `int32` |
| 4 | `stamina_level` | `int32` |
| 5 | `power_ability` | `int32` |
| 6 | `anaerobic_ability` | `int32` |
| 7 | `aerobic_power_ability` | `int32` |
| 8 | `aerobic_abiltiy` | `int32` |
| 9 | `endurance_abiltiy` | `int32` |
| 10 | `ultra_endurance_ability` | `int32` |
| 11 | `vo2max` | `int32` |
| 12 | `ftp` | `int32` |
| 13 | `pwr` | `int32` |
| 14 | `map` | `int32` |
| 15 | `trainingload` | `int32` |
| 16 | `lthr` | `int32` |
| 17 | `phrz` | `repeated int32` |
| 18 | `trainingstatus` | `int32` |
| 19 | `tltrend` | `int32` |
| 20 | `stamina` | `int32` |
