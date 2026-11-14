# WatchHealthData

_Generated from `com.igpsport.blelib.bean.WatchHealthData`._

## Enums

### `WATCH_DAILY_ACTIVITY_TYPE`

| Name | Value |
|---|---:|
| `WATCH_DAILY_ACTIVITY_TYPE_NONE` | 0 |
| `WATCH_DAILY_ACTIVITY_TYPE_STEP` | 1 |
| `WATCH_DAILY_ACTIVITY_TYPE_CALORIES` | 2 |
| `WATCH_DAILY_ACTIVITY_TYPE_INTENSE_TIME` | 3 |
| `WATCH_DAILY_ACTIVITY_TYPE_ACTIVE_HOUR` | 4 |

### `WATCH_DAILY_ALARM_TYPE`

| Name | Value |
|---|---:|
| `WATCH_DAILY_ALARM_TYPE_NONE` | 0 |
| `WATCH_DAILY_ALARM_TYPE_STAND` | 1 |
| `WATCH_DAILY_ALARM_TYPE_DRINK` | 2 |

### `WATCH_HEALTH_DATA_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_WATCH_HEALTH_DATA_OPERATE_TYPE_NONE` | 0 |
| `enum_WATCH_HEALTH_DATA_OPERATE_TYPE_REAL_TIME_GET` | 1 |
| `enum_WATCH_HEALTH_DATA_OPERATE_TYPE_HISTORY_GET` | 3 |
| `enum_WATCH_HEALTH_DATA_OPERATE_TYPE_DLY_ACT_TARGET_GET` | 5 |
| `enum_WATCH_HEALTH_DATA_OPERATE_TYPE_DLY_ACT_TARGET_SET` | 7 |
| `enum_WATCH_HEALTH_DATA_OPERATE_TYPE_DLY_ACT_ALARM_GET` | 8 |
| `enum_WATCH_HEALTH_DATA_OPERATE_TYPE_DLY_ACT_ALARM_SET` | 10 |
| `enum_WATCH_HEALTH_DATA_OPERATE_TYPE_DLY_ACT_DATA_SYNC` | 11 |
| `enum_WATCH_HEALTH_DATA_OPERATE_TYPE_HISTORY_FILE_PACKAGED` | 12 |
| `enum_WATCH_HEALTH_DATA_OPERATE_TYPE_HISTORY_FILE_GET` | 13 |
| `enum_WATCH_HEALTH_DATA_OPERATE_TYPE_HISTORY_LIST_GET` | 15 |

### `WATCH_HEALTH_DATA_TYPE`

| Name | Value |
|---|---:|
| `WATCH_HEALTH_DATA_DEFAULT` | 0 |
| `WATCH_HEALTH_DATA_SLEEP` | 1 |
| `WATCH_HEALTH_DATA_STRESS` | 2 |
| `WATCH_HEALTH_DATA_HEART_RATE` | 3 |
| `WATCH_HEALTH_DATA_SPO2` | 4 |
| `WATCH_HEALTH_DATA_HEART_RATE_VARIABILITY` | 5 |
| `WATCH_HEALTH_DATA_DAILY_ACTIVITY` | 6 |
| `WATCH_HEALTH_DATA_RESTING_HR` | 7 |

## Messages

### `watch_daily_alarm_message`

| # | Field | Type |
|---:|---|---|
| 1 | `daily_alarm_type` | `WATCH_DAILY_ALARM_TYPE` |
| 2 | `alarm_switch` | `int32` |
| 3 | `alarm_time_start` | `int32` |
| 4 | `alarm_time_end` | `int32` |
| 5 | `alarm_interval` | `int32` |
| 6 | `dnd_start` | `int32` |
| 7 | `dnd_end` | `int32` |
| 8 | `dnd_switch` | `int32` |

### `watch_daily_target_message`

| # | Field | Type |
|---:|---|---|
| 1 | `daily_activity_type` | `WATCH_DAILY_ACTIVITY_TYPE` |
| 2 | `goal_value` | `int32` |
| 3 | `status` | `int32` |

### `watch_health_data_file_list_message`

| # | Field | Type |
|---:|---|---|
| 1 | `file_name` | `string` |
| 2 | `health_data_type` | `WATCH_HEALTH_DATA_TYPE` |

### `watch_health_data_message`

| # | Field | Type |
|---:|---|---|
| 1 | `health_data_type` | `WATCH_HEALTH_DATA_TYPE` |
| 2 | `timestamp` | `int32` |
| 3 | `timezone` | `int32` |
| 4 | `health_data_value` | `int32` |
| 5 | `sleep_data_msg` | `watch_sleep_data_message` |
| 6 | `hrv_period_msg` | `watch_hrv_period_message` |
| 7 | `step` | `int32` |
| 8 | `calories` | `float` |
| 9 | `intense_time` | `int32` |

### `watch_health_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `health_data_operate_type` | `WATCH_HEALTH_DATA_OPERATE_TYPE` |
| 3 | `version` | `int32` |
| 4 | `health_data_msg` | `repeated watch_health_data_message` |
| 5 | `daily_target_msg` | `repeated watch_daily_target_message` |
| 6 | `daily_alarm_msg` | `repeated watch_daily_alarm_message` |
| 7 | `file_list_num` | `Common.file_list_get_message` |
| 8 | `watch_daily_all_alarm_status` | `int32` |
| 9 | `file_list_msg` | `repeated watch_health_data_file_list_message` |

### `watch_hrv_period_message`

| # | Field | Type |
|---:|---|---|
| 1 | `timestamp` | `int32` |
| 2 | `timezone` | `int32` |
| 3 | `sum` | `int32` |
| 4 | `data_num` | `int32` |
| 5 | `hrv_data` | `repeated int32` |
| 6 | `min` | `int32` |
| 7 | `max` | `int32` |
| 8 | `avg` | `int32` |
| 9 | `reserved` | `int32` |

### `watch_short_sleep_data_message`

| # | Field | Type |
|---:|---|---|
| 1 | `start_time` | `int32` |
| 2 | `end_time` | `int32` |
| 3 | `total_time` | `int32` |

### `watch_short_sleep_period_message`

| # | Field | Type |
|---:|---|---|
| 1 | `data_num` | `int32` |
| 2 | `short_sleep_data_msg` | `repeated watch_short_sleep_data_message` |

### `watch_sleep_data_message`

| # | Field | Type |
|---:|---|---|
| 1 | `timestamp` | `int32` |
| 2 | `short_sleep_period_msg` | `watch_short_sleep_period_message` |
| 3 | `sleep_period_summary_msg` | `watch_sleep_period_summary_message` |
| 4 | `sleep_period_stage_msg` | `watch_sleep_period_stage_message` |

### `watch_sleep_period_stage_message`

| # | Field | Type |
|---:|---|---|
| 1 | `num` | `int32` |
| 2 | `sleep_stage_msg` | `repeated watch_sleep_stage_message` |

### `watch_sleep_period_summary_message`

| # | Field | Type |
|---:|---|---|
| 1 | `start_time` | `int32` |
| 2 | `end_time` | `int32` |
| 3 | `total_time` | `int32` |
| 4 | `stage_num` | `int32` |
| 5 | `wake_minutes` | `int32` |
| 6 | `rem_minutes` | `int32` |
| 7 | `light_minutes` | `int32` |
| 8 | `deep_minutes` | `int32` |
| 9 | `sleep_score` | `int32` |
| 10 | `sleep_type` | `int32` |

### `watch_sleep_stage_message`

| # | Field | Type |
|---:|---|---|
| 1 | `stage` | `int32` |
| 2 | `total_time` | `int32` |
