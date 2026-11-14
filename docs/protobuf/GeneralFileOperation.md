# GeneralFileOperation

_Generated from `com.igpsport.blelib.bean.GeneralFileOperation`._

## Enums

### `file_operation_type`

| Name | Value |
|---|---:|
| `enum_FILE_TYPE_GENERAL` | 0 |
| `enum_FILE_TYPE_TRAINING` | 1 |
| `enum_FILE_TYPE_ROUTE_PLAN` | 2 |
| `enum_FILE_TYPE_MAP` | 3 |
| `enum_FILE_TYPE_THEME` | 4 |
| `enum_FILE_TYPE_FIRMWARE` | 5 |
| `enum_FILE_TYPE_LANGUAGE` | 6 |
| `enum_FILE_TYPE_AGPS` | 7 |
| `enum_FILE_TYPE_ROUTE_BOOK` | 8 |
| `enum_FILE_TYPE_STAGE` | 9 |
| `enum_FILE_TYPE_POWERON_IMG` | 10 |
| `enum_FILE_TYPE_SCHEDULE` | 11 |
| `enum_FILE_TYPE_THEME_BACKGOUND` | 12 |
| `enum_FILE_TYPE_GROUP_RIDE_USER_HEAD_PORTRAIT` | 13 |
| `enum_FILE_TYPE_CLOUD_SWITCH_DEV` | 14 |
| `enum_FILE_TYPE_GROUP_RIDE_ROUTE_BOOK` | 15 |
| `enum_FILE_TYPE_WATCH_DIAL` | 16 |
| `enum_FILE_TYPE_YAW_ANT_RENAVI_BOOK` | 17 |
| `enum_FILE_TYPE_WATCH_PHOTO_DIAL` | 18 |

## Messages

### `file_extra_Info`

| # | Field | Type |
|---:|---|---|
| 1 | `distance` | `int32` |
| 2 | `totalascent` | `int32` |
| 3 | `avgslope` | `int32` |

### `file_transfer_method_information`

| # | Field | Type |
|---:|---|---|
| 1 | `resume_supported` | `bool` |
| 2 | `packet_ack_supported` | `bool` |
| 3 | `resume_offset` | `int32` |

### `general_file_operation`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `operate_type` | `Common.SERVICE_OPERATE_TYPE` |
| 3 | `file_type` | `file_operation_type` |
| 4 | `file_size` | `int32` |
| 5 | `file_id` | `int32` |
| 6 | `file_name` | `string` |
| 7 | `file_extension` | `string` |
| 8 | `file_md5` | `string` |
| 9 | `file_verify_val` | `int32` |
| 10 | `is_temp` | `bool` |
| 11 | `extra_info` | `file_extra_Info` |
| 12 | `watch_photo_dial_extra_info` | `watch_photo_dial_extra_Info` |
| 15 | `method_information` | `file_transfer_method_information` |

### `watch_photo_dial_extra_Info`

| # | Field | Type |
|---:|---|---|
| 1 | `dial_id` | `int32` |
| 2 | `time_position_type` | `int32` |
| 3 | `color` | `int32` |
| 4 | `num` | `int32` |
| 5 | `dial_type` | `int32` |
