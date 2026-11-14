# MapNew (`com.igpsport.blelib` / obfuscated `defpackage`)

_Extracted from `app/src/main/java/defpackage/MapNew.java` (service 22,
`MAP_NEW`)._

## Enums

### `map_op_type`

| Name | Value |
|---|---:|
| `MAP_OP_INVALID` | 0 |
| `MAP_OP_LIST_NUM_GET` | 1 |
| `MAP_OP_LIST_GET` | 2 |
| `MAP_OP_DOWNLOAD` | 3 |
| `MAP_OP_UPDATE` | 4 |
| `MAP_OP_DELETE` | 5 |
| `MAP_OP_SET` | 6 |

### `map_op_state`

| Name | Value |
|---|---:|
| `MAP_OP_STATE_INVALID` | 0 |
| `MAP_OP_STATE_START` | 1 |
| `MAP_OP_STATE_END` | 2 |

## Messages

### `map_list_message`

| # | Field | Type |
|---:|---|---|
| 1 | `map_country_name` | `string` |
| 2 | `map_id` | `int32` |
| 3 | `map_coordinates` | `string` |

### `map_new_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `map_op_type` | `map_op_type` |
| 3 | `map_list_msg` | `repeated map_list_message` |
| 4 | `map_op_sta` | `map_op_state` |
| 5 | `map_list_get_msg` | `Common.file_list_get_message` |
