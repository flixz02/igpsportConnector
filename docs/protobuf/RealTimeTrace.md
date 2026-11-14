# RealTimeTrace

_Generated from `com.igpsport.blelib.bean.RealTimeTrace`._

## Enums

### `REAL_TIME_TRACE_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_REAL_TIME_TRACE_OPERATE_TYPE_NONE` | 0 |
| `enum_REAL_TIME_TRACE_OPERATE_TYPE_GET` | 1 |
| `enum_REAL_TIME_TRACE_OPERATE_TYPE_FIT` | 2 |
| `enum_REAL_TIME_TRACE_OPERATE_TYPE_FINISH` | 3 |
| `enum_REAL_TIME_TRACE_OPERATE_TYPE_END` | 4 |

## Messages

### `real_time_trace_fit_message`

| # | Field | Type |
|---:|---|---|
| 1 | `timestamp` | `int32` |
| 2 | `fit_content` | `bytes` |

### `real_time_trace_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `real_time_trace_operate_type` | `REAL_TIME_TRACE_OPERATE_TYPE` |
| 3 | `real_time_trace_fit_msg` | `real_time_trace_fit_message` |
