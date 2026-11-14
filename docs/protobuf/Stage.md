# Stage

_Generated from `com.igpsport.blelib.bean.Stage`._

## Enums

### `STAGE_DATA_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_STAGE_DATA_OPERATE_TYPE_NONE` | 0 |
| `enum_STAGE_DATA_OPERATE_TYPE_LIST_NUM_GET` | 1 |
| `enum_STAGE_DATA_OPERATE_TYPE_LIST_GET` | 2 |
| `enum_STAGE_DATA_OPERATE_TYPE_FILE_DEL` | 3 |
| `enum_STAGE_DATA_OPERATE_TYPE_FILE_SYNC_START` | 4 |
| `enum_STAGE_DATA_OPERATE_TYPE_FILE_SYNC_END` | 5 |

## Messages

### `stage_data_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `statge_date_operate_type` | `STAGE_DATA_OPERATE_TYPE` |
| 3 | `stage_file_msg` | `repeated stage_file_message` |
| 4 | `list_msg` | `Common.file_list_get_message` |

### `stage_file_message`

| # | Field | Type |
|---:|---|---|
| 1 | `file_name` | `string` |
| 2 | `file_size` | `int32` |
| 3 | `file_id` | `int32` |
