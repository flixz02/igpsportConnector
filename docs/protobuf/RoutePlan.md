# RoutePlan

_Generated from `com.igpsport.blelib.bean.RoutePlan`._

## Enums

### `ROUTE_PLAN_FILE_STATUS`

| Name | Value |
|---|---:|
| `enum_INVALID_STATUS` | 0 |
| `enum_USED_STATUS` | 1 |
| `enum_UNUSED_STATUS` | 2 |

### `ROUTE_PLAN_FILE_TYPE`

| Name | Value |
|---|---:|
| `enum_ROUTE_PLAN_FILE_TYPE_INVALID` | 0 |
| `enum_ROUTE_PLAN_FILE_TYPE_CNX` | 1 |
| `enum_ROUTE_PLAN_FILE_TYPE_GPX` | 2 |
| `enum_ROUTE_PLAN_FILE_TYPE_FIT` | 3 |
| `enum_ROUTE_PLAN_FILE_TYPE_TCX` | 4 |
| `enum_ROUTE_PLAN_FILE_TYPE_XML` | 5 |

### `ROUTE_PLAN_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_ROUTE_PLAN_OPERATE_TYPE_NONE` | 0 |
| `enum_ROUTE_PLAN_OPERATE_TYPE_LIST_GET` | 1 |
| `enum_ROUTE_PLAN_OPERATE_TYPE_LIST_SEND` | 2 |
| `enum_ROUTE_PLAN_OPERATE_TYPE_FILE_DEL` | 3 |
| `enum_ROUTE_PLAN_OPERATE_TYPE_FILE_SEND` | 4 |
| `enum_ROUTE_PLAN_OPERATE_TYPE_FILE_USE` | 5 |
| `enum_ROUTE_PLAN_OPERATE_TYPE_FILES_DEL` | 6 |
| `enum_ROUTE_PLAN_OPERATE_TYPE_LIST_NUM_GET` | 7 |
| `enum_ROUTE_PLAN_OPERATE_TYPE_RENAME` | 8 |
| `enum_ROUTE_PLAN_OPERATE_TYPE_TURN_PROMPT` | 9 |
| `enum_ROUTE_PLAN_OPERATE_TYPE_NOTIFY_TRAJECTORY_DEVIATION` | 10 |
| `enum_ROUTE_PLAN_OPERATE_TYPE_ROUTE_POINT_GET` | 11 |
| `enum_ROUTE_PLAN_OPERATE_TYPE_ROUTE_REROUTE_STATUS` | 12 |
| `enum_ROUTE_PLAN_OPERATE_TYPE_REQUEST_WHETHER_SUPPORT_FUNCTION` | 13 |
| `enum_ROUTE_PLAN_OPERATE_TYPE_SUPPORT_FUNCTION_ENABLE_STATUS` | 14 |

### `ROUTE_TURN_PROMPT`

| Name | Value |
|---|---:|
| `enumNAVI_TRUN_LEFT_ROUND` | 0 |
| `enumNAVI_TURN_LEFT_SHARPLY` | 1 |
| `enumNAVI_TURN_LEFT_NORMALLY` | 2 |
| `enumNAVI_TURN_LEFT_FRONT` | 3 |
| `enumNAVI_TURN_RIGHT_FRONT` | 4 |
| `enumNAVI_TURN_RIGHT_NORMALLY` | 5 |
| `enumNAVI_TURN_RIGHT_SHARPLY` | 6 |
| `enumNAVI_TRUN_RIGHT_ROUND` | 7 |
| `enumNAVI_TRUN_LEFT_ROUND_AT_ONCE` | 8 |
| `enumNAVI_TURN_LEFT_AT_ONCE` | 9 |
| `enumNAVI_TURN_LEFT_NORMALLY_AT_ONCE` | 10 |
| `enumNAVI_TURN_LEFT_FRONT_AT_ONCE` | 11 |
| `enumNAVI_TURN_RIGHT_FRONT_AT_ONCE` | 12 |
| `enumNAVI_TURN_RIGHT_NORMALLY_AT_ONCE` | 13 |
| `enumNAVI_TURN_RIGHT_AT_ONCE` | 14 |
| `enumNAVI_TRUN_RIGHT_ROUND_AT_ONCE` | 15 |
| `enumNAVI_TRUN_WINDING` | 16 |
| `enumNAVI_START_NAVIGATION` | 17 |
| `enumNAVI_YAW` | 18 |
| `enumNAVI_YAW_ANT_RENAVI` | 19 |
| `enumNAVI_YAW_RECOVERY` | 20 |
| `enumNAVI_NAVIGATION_END` | 21 |

### `route_reroute_status`

| Name | Value |
|---|---:|
| `enum_ROUTE_REROUTE_INVALID_STATUS` | 0 |
| `enum_ROUTE_REROUTE_ERROR_PLANNING_FAILED` | 1 |

## Messages

### `app_support_function_status`

| # | Field | Type |
|---:|---|---|
| 1 | `is_support_reroute` | `bool` |

### `reroute_point_info_message`

| # | Field | Type |
|---:|---|---|
| 1 | `reroute_start_point` | `route_point_msg` |
| 2 | `reroute_end_point` | `route_point_msg` |
| 3 | `reroute_pass_point` | `repeated route_point_msg` |

### `route_plan_data_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `route_plan_operate_type` | `ROUTE_PLAN_OPERATE_TYPE` |
| 3 | `line_id` | `repeated string` |
| 4 | `file_content` | `bytes` |
| 5 | `route_plan_info_msg` | `repeated route_plan_info_message` |
| 6 | `reroute_point_msg` | `reroute_point_info_message` |
| 7 | `reroute_status` | `route_reroute_status` |
| 8 | `support_function_status` | `app_support_function_status` |
| 12 | `route_list_get_msg` | `Common.file_list_get_message` |

### `route_plan_info_message`

| # | Field | Type |
|---:|---|---|
| 1 | `id` | `int32` |
| 2 | `file_type` | `ROUTE_PLAN_FILE_TYPE` |
| 3 | `name` | `string` |
| 4 | `total_distance` | `int32` |
| 5 | `longitude_start` | `double` |
| 6 | `latitude_start` | `double` |
| 7 | `status` | `ROUTE_PLAN_FILE_STATUS` |

### `route_point_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `longitude` | `double` |
| 2 | `latitude` | `double` |
