# SportAbility

_Generated from `com.igpsport.blelib.bean.SportAbility`._

## Enums

### `SPORT_ABILITY_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_SPORT_ABILITY_OPERATE_TYPE_NONE` | 0 |
| `enum_SPORT_ABILITY_OPERATE_TYPE_DATA_SYNC` | 1 |
| `enum_SPORT_ABILITY_OPERATE_TYPE_FILE_PACKAGED_REQUEST` | 2 |
| `enum_SPORT_ABILITY_OPERATE_TYPE_FILE_PACKAGED_COMPLETE` | 3 |
| `enum_SPORT_ABILITY_OPERATE_TYPE_FILE_GET` | 4 |

## Messages

### `sport_ability_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `sport_ability_operate_type` | `SPORT_ABILITY_OPERATE_TYPE` |
| 3 | `sport_ability_sync_data_msg` | `sport_ability_sync_data_message` |

### `sport_ability_sync_data_message`

| # | Field | Type |
|---:|---|---|
| 1 | `start_timestamp` | `int32` |
| 2 | `end_timestamp` | `int32` |
