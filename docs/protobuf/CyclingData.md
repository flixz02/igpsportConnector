# CyclingData

_Generated from `com.igpsport.blelib.bean.CyclingData`._

## Enums

### `CYCLING_DATA_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_CYCLING_DATA_OPERATE_TYPE_NONE` | 0 |
| `enum_CYCLING_DATA_OPERATE_TYPE_LIST_GET` | 1 |
| `enum_CYCLING_DATA_OPERATE_TYPE_LIST_SEND` | 2 |
| `enum_CYCLING_DATA_OPERATE_TYPE_FILE_GET` | 3 |
| `enum_CYCLING_DATA_OPERATE_TYPE_FILE_SEND` | 4 |
| `enum_CYCLING_DATA_OPERATE_TYPE_FILE_DEL` | 5 |
| `enum_CYCLING_DATA_OPERATE_TYPE_ALL_DEL` | 6 |
| `enum_CYCLING_DATA_OPERATE_TYPE_AUTO_UPLOAD` | 7 |
| `enum_CYCLING_DATA_OPERATE_TYPE_LIST_NUM_GET` | 8 |

## Messages

### `cycling_data_auto_upload_message`

| # | Field | Type |
|---:|---|---|
| 1 | `status` | `int32` |
| 2 | `cycling_data_url` | `string` |
| 3 | `cycling_data_check_url` | `string` |

### `cycling_data_file_flag_message`

| # | Field | Type |
|---:|---|---|
| 1 | `timestamp` | `int32` |
| 2 | `file_size` | `int32` |
| 3 | `user_id` | `string` |
| 4 | `device_id` | `string` |

### `cycling_data_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `cycling_data_operate_type` | `CYCLING_DATA_OPERATE_TYPE` |
| 3 | `cycling_data_file_flag_msg` | `repeated cycling_data_file_flag_message` |
| 4 | `file_content` | `bytes` |
| 5 | `cycling_data_auto_upload_msg` | `cycling_data_auto_upload_message` |
| 6 | `list_msg` | `Common.file_list_get_message` |
