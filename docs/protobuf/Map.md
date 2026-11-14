# Map

_Generated from `com.igpsport.blelib.bean.Map`._

## Enums

### `MAP_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_MAP_OPERATE_TYPE_NONE` | 0 |
| `enum_MAP_OPERATE_TYPE_LIST_GET` | 1 |
| `enum_MAP_OPERATE_TYPE_LIST_SEND` | 2 |
| `enum_MAP_OPERATE_TYPE_ASSIGN_UPDATE` | 3 |
| `enum_MAP_OPERATE_TYPE_ALL_UPDATE` | 4 |
| `enum_MAP_OPERATE_TYPE_DOWNLOAD` | 5 |
| `enum_MAP_OPERATE_TYPE_SUCCESS` | 6 |
| `enum_MAP_OPERATE_YPTE_CANCEL_DOWNLOAD` | 7 |
| `enum_MAP_OPERATE_TYPE_PROGRESS_GET` | 8 |
| `enum_MAP_OPERATE_TYPE_PROGRESS_UPLOAD` | 9 |
| `enum_MAP_OPERATE_TYPE_ASSIGN_DEL` | 10 |
| `enum_MAP_OPERATE_TYPE_ALL_DEL` | 11 |
| `enum_MAP_CONFIG_CMD` | 12 |

## Messages

### `map_data_message`

| # | Field | Type |
|---:|---|---|
| 1 | `map_id` | `int32` |
| 2 | `area_id` | `string` |
| 3 | `version` | `int32` |
| 4 | `url` | `string` |
| 5 | `size` | `int32` |
| 6 | `area_type` | `int32` |
| 7 | `progress` | `int32` |
| 8 | `config` | `int32` |

### `map_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `map_operate_type` | `MAP_OPERATE_TYPE` |
| 3 | `map_data_msg` | `repeated map_data_message` |
