# WatchDial

_Generated from `com.igpsport.blelib.bean.WatchDial`._

## Enums

### `WATCH_DIAL_ADD_STATE_TYPE`

| Name | Value |
|---|---:|
| `enum_WATCH_DIAL_ADD_ALLOW` | 0 |
| `enum_WATCH_DIAL_ADD_DISALLOW_UNZIPPING` | 1 |
| `enum_WATCH_DIAL_ADD_DISALLOW_MAX_LIMIT` | 2 |
| `enum_WATCH_DIAL_ADD_DISALLOW_EXERCISING` | 3 |
| `enum_WATCH_DIAL_ADD_DISALLOW_EXIST` | 4 |

### `WATCH_DIAL_COLOR_TYPE_E`

| Name | Value |
|---|---:|
| `enum_WATCH_DIAL_COLOR_TYPE_INVALID` | 0 |
| `enum_WATCH_DIAL_COLOR_TYPE_MAGENTA` | 1 |
| `enum_WATCH_DIAL_COLOR_TYPE_RED1` | 2 |
| `enum_WATCH_DIAL_COLOR_TYPE_GREEN1` | 3 |
| `enum_WATCH_DIAL_COLOR_TYPE_BLUE1` | 4 |
| `enum_WATCH_DIAL_COLOR_TYPE_YELLOW1` | 5 |
| `enum_WATCH_DIAL_COLOR_TYPE_CYAN1` | 6 |
| `enum_WATCH_DIAL_COLOR_TYPE_LIGHT_STEEL_BLUE_SIMI` | 7 |

### `WATCH_DIAL_DATA_TYPE_E`

| Name | Value |
|---|---:|
| `enum_WATCH_DIAL_DATA_TYPE_INVALID` | 0 |
| `enum_WATCH_DIAL_DATA_TYPE_HR` | 1 |
| `enum_WATCH_DIAL_DATA_TYPE_STRESS` | 2 |
| `enum_WATCH_DIAL_DATA_TYPE_STEP` | 3 |
| `enum_WATCH_DIAL_DATA_TYPE_CAL` | 4 |
| `enum_WATCH_DIAL_DATA_TYPE_INTENSE` | 5 |
| `enum_WATCH_DIAL_DATA_TYPE_RUN_VO2MAX` | 6 |
| `enum_WATCH_DIAL_DATA_TYPE_RIDE_VO2MAX` | 7 |
| `enum_WATCH_DIAL_DATA_TYPE_BATTERY` | 8 |
| `enum_WATCH_DIAL_DATA_TYPE_ALTITUDE` | 9 |
| `enum_WATCH_DIAL_DATA_TYPE_BAROMETRIC` | 10 |
| `enum_WATCH_DIAL_DATA_TYPE_WEATHER` | 11 |
| `enum_WATCH_DIAL_DATA_TYPE_ALIPAY` | 12 |
| `enum_WATCH_DIAL_DATA_TYPE_COMPASS` | 13 |
| `enum_WATCH_DIAL_DATA_TYPE_ALARM` | 14 |
| `enum_WATCH_DIAL_DATA_TYPE_STOPWATCH` | 15 |
| `enum_WATCH_DIAL_DATA_TYPE_TIMER` | 16 |
| `enum_WATCH_DIAL_DATA_TYPE_HR_PUSH` | 17 |

### `WATCH_DIAL_OPERATION_TYPE`

| Name | Value |
|---|---:|
| `enum_WATCH_DIAL_OPERATION_TYPE_NONE` | 0 |
| `enum_WATCH_DIAL_OPERATION_TYPE_WATCH_DIAL_CURRENT_GET` | 1 |
| `enum_WATCH_DIAL_OPERATION_TYPE_WATCH_DIAL_ADD` | 2 |
| `enum_WATCH_DIAL_OPERATION_TYPE_WATCH_DIAL_DELETE` | 3 |
| `enum_WATCH_DIAL_OPERATION_TYPE_WATCH_DIAL_LIST_GET` | 4 |
| `enum_WATCH_DIAL_OPERATION_TYPE_WATCH_DIAL_LIST_NUM_GET` | 5 |
| `enum_WATCH_DIAL_OPERATION_TYPE_WATCH_DIAL_CFG_GET` | 6 |
| `enum_WATCH_DIAL_OPERATION_TYPE_WATCH_DIAL_CFG_SET` | 7 |
| `enum_WATCH_DIAL_OPERATION_TYPE_WATCH_DIAL_USE` | 8 |
| `enum_WATCH_DIAL_OPERATION_TYPE_WATCH_DIAL_INFO_GET` | 9 |
| `enum_WATCH_DIAL_OPERATION_TYPE_WATCH_DIAL_ADD_STATE_GET` | 10 |

### `WATCH_DIAL_TIME_POSITION_TYPE_E`

| Name | Value |
|---|---:|
| `enum_WATCH_DIAL_TIME_POSITION_TYPE_INVALID` | 0 |
| `enum_WATCH_DIAL_TIME_POSITION_TYPE_TOP` | 1 |
| `enum_WATCH_DIAL_TIME_POSITION_TYPE_CENTER` | 2 |
| `enum_WATCH_DIAL_TIME_POSITION_TYPE_BOTTOM` | 3 |

### `WATCH_DIAL_TYPE`

| Name | Value |
|---|---:|
| `enum_WATCH_DIAL_TYPE_INVALID` | 0 |
| `enum_WATCH_DIAL_TYPE_SYSTEM` | 1 |
| `enum_WATCH_DIAL_TYPE_PHOTO` | 2 |
| `enum_WATCH_DIAL_TYPE_STORE` | 3 |

## Messages

### `dial_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `operation_type` | `WATCH_DIAL_OPERATION_TYPE` |
| 3 | `file_list_get` | `Common.file_list_get_message` |
| 4 | `file_list` | `repeated watch_dial_file_flag_message` |
| 5 | `cfg_message` | `watch_dial_cfg_message` |
| 6 | `dial_info_message` | `watch_dial_base_info_message` |
| 7 | `state` | `WATCH_DIAL_ADD_STATE_TYPE` |

### `watch_dial_base_info_message`

| # | Field | Type |
|---:|---|---|
| 1 | `resolution_width` | `int32` |
| 2 | `resolution_height` | `int32` |
| 3 | `master_dial_info` | `watch_master_dial_message` |
| 4 | `max_dial_num` | `int32` |

### `watch_dial_cfg_message`

| # | Field | Type |
|---:|---|---|
| 1 | `goodsid` | `int32` |
| 2 | `data_support_type` | `repeated WATCH_DIAL_DATA_TYPE_E` |
| 3 | `data_cur_type` | `repeated WATCH_DIAL_DATA_TYPE_E` |
| 4 | `color_support_type` | `repeated WATCH_DIAL_COLOR_TYPE_E` |
| 5 | `color_cur_type` | `WATCH_DIAL_COLOR_TYPE_E` |
| 6 | `dial_type` | `WATCH_DIAL_TYPE` |

### `watch_dial_file_flag_message`

| # | Field | Type |
|---:|---|---|
| 1 | `goodsid` | `int32` |
| 2 | `file_size` | `int32` |
| 3 | `in_use` | `int32` |
| 4 | `dial_type` | `WATCH_DIAL_TYPE` |

### `watch_master_dial_message`

| # | Field | Type |
|---:|---|---|
| 1 | `allow_time_position_type` | `repeated WATCH_DIAL_TIME_POSITION_TYPE_E` |
