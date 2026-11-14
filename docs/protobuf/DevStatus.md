# DevStatus

_Generated from `com.igpsport.blelib.bean.DevStatus`._

## Enums

### `DEV_CYCLING_STATUS`

| Name | Value |
|---|---:|
| `DEV_CYCLING_STATUS_FREE` | 0 |
| `DEV_CYCLING_STATUS_DOING` | 1 |
| `DEV_CYCLING_STATUS_PAUSE` | 2 |

### `DEV_NAVI_STATUS`

| Name | Value |
|---|---:|
| `DEV_NAVI_STATUS_OFF` | 0 |
| `DEV_NAVI_STATUS_ON` | 1 |

### `DEV_STATUS_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_DEV_STATUS_OPERATE_TYPE_NONE` | 0 |
| `enum_DEV_STATUS_OPERATE_TYPE_GET` | 1 |
| `enum_DEV_STATUS_OPERATE_TYPE_SEND` | 2 |
| `enum_DEV_STATUS_OPERATE_TYPE_ADD_POSITION_POINT` | 3 |
| `enum_DEV_STATUS_OPERATE_TYPE_DELETE_POSITION_POINT` | 4 |
| `enum_DEV_STATUS_OPERATE_TYPE_MODIFY_POSITION_POINT` | 5 |

### `DEV_WIFI_STATUS`

| Name | Value |
|---|---:|
| `DEV_WIFI_STATUS_IDLE` | 1 |
| `DEV_WIFI_STATUS_MAP` | 2 |
| `DEV_WIFI_STATUS_FIRMWARE` | 3 |
| `DEV_WIFI_STATUS_UPLOAD` | 4 |
| `DEV_WIFI_STATUS_SET` | 5 |

## Messages

### `dev_cycling_status_massage`

| # | Field | Type |
|---:|---|---|
| 1 | `dev_cycling_status` | `DEV_CYCLING_STATUS` |
| 2 | `cycling_start_time` | `int32` |

### `dev_gps_massage`

| # | Field | Type |
|---:|---|---|
| 1 | `latitude` | `double` |
| 2 | `longitude` | `double` |

### `dev_position_point`

| # | Field | Type |
|---:|---|---|
| 1 | `latitude` | `int32` |
| 2 | `longitude` | `int32` |
| 3 | `pointname` | `string` |
| 4 | `point_index` | `int32` |

### `dev_position_point_group`

| # | Field | Type |
|---:|---|---|
| 1 | `position_point_num` | `int32` |
| 2 | `position_info` | `repeated dev_position_point` |

### `dev_status_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `op_type` | `DEV_STATUS_OPERATE_TYPE` |
| 3 | `dev_cycling_status_msg` | `dev_cycling_status_massage` |
| 4 | `dev_gps_msg` | `dev_gps_massage` |
| 5 | `rt_data_msg` | `rt_data_message` |
| 6 | `wifi_status` | `DEV_WIFI_STATUS` |
| 7 | `navi_status` | `DEV_NAVI_STATUS` |
| 8 | `position_point_manager` | `dev_position_point_group` |

### `rt_data_message`

| # | Field | Type |
|---:|---|---|
| 1 | `real_time_speed` | `int32` |
| 2 | `avg_speed` | `int32` |
| 3 | `riding_time` | `int32` |
| 4 | `riding_distance` | `int32` |
| 5 | `real_time_cad` | `int32` |
| 6 | `real_time_hrm` | `int32` |
| 7 | `avg_hrm` | `int32` |
| 8 | `real_time_power` | `int32` |
| 9 | `total_height` | `int32` |
| 10 | `avg_rise` | `int32` |
| 11 | `cur_height` | `int32` |
| 12 | `cur_slope` | `int32` |
| 13 | `max_speed` | `int32` |
| 14 | `max_hrm` | `int32` |
| 15 | `avg_cad` | `int32` |
| 16 | `max_cad` | `int32` |
| 17 | `avg_power` | `int32` |
| 18 | `max_power` | `int32` |
| 19 | `course` | `int32` |
