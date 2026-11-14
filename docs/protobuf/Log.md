# Log

_Generated from `com.igpsport.blelib.bean.Log`._

## Enums

### `LOG_SUB_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_LOG_INVALID` | 0 |
| `enum_LOG_NUMBER_GET` | 16 |
| `enum_LOG_GET` | 17 |
| `enum_DEBUG_LOG_PACK_REQUEST` | 18 |
| `enum_DEBUG_LOG_PACK_COMPLETE` | 19 |

## Messages

### `log_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service` | `Common.service_type_index` |
| 2 | `operate` | `Common.SERVICE_OPERATE_TYPE` |
| 3 | `sub_operate` | `LOG_SUB_OPERATE_TYPE` |
| 4 | `log_name` | `string` |
| 5 | `content` | `bytes` |
| 6 | `log_num` | `int32` |
| 7 | `log_type` | `int32` |
