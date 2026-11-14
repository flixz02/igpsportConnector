# RouteBook

_Generated from `com.igpsport.blelib.bean.RouteBook`._

## Enums

### `ROUTE_BOOK_FILE_STATUS`

| Name | Value |
|---|---:|
| `enum_ROUTE_BOOK_FILE_INVALID_STATUS` | 0 |
| `enum_ROUTE_BOOK_FILE_USED_STATUS` | 1 |
| `enum_ROUTE_BOOK_FILE_UNUSED_STATUS` | 2 |

### `ROUTE_BOOK_SUB_OP_TYPE`

| Name | Value |
|---|---:|
| `enum_ROUTE_BOOK_GET_SUB_OP_TYPE_NONE` | 0 |
| `enum_ROUTE_BOOK_GET_SUB_OP_TYPE_LIST_NUM_GET` | 1 |
| `enum_ROUTE_BOOK_GET_SUB_OP_TYPE_LIST_GET` | 2 |
| `enum_ROUTE_BOOK_SET_SUB_OP_TYPE_USE` | 17 |
| `enum_ROUTE_BOOK_SET_SUB_OP_TYPE_RENAME` | 18 |

## Messages

### `route_book_data_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `operate_type` | `Common.SERVICE_OPERATE_TYPE` |
| 3 | `sub_operate_type` | `ROUTE_BOOK_SUB_OP_TYPE` |
| 4 | `route_book_infor_msg` | `repeated route_book_infor_message` |
| 10 | `route_book_list_get_msg` | `Common.file_list_get_message` |

### `route_book_infor_message`

| # | Field | Type |
|---:|---|---|
| 1 | `id` | `int32` |
| 3 | `name` | `string` |
| 7 | `status` | `ROUTE_BOOK_FILE_STATUS` |
