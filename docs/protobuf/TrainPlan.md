# TrainPlan

_Generated from `com.igpsport.blelib.bean.TrainPlan`._

## Enums

### `TRAINING_PLAN_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_TRAINING_PLAN_OPERATE_TYPE_NONE` | 0 |
| `enum_TRAINING_PLAN_OPERATE_TYPE_GET_SCHEDULE_NUM` | 1 |
| `enum_TRAINING_PLAN_OPERATE_TYPE_GET_SCHEDULE_FILE` | 2 |
| `enum_TRAINING_PLAN_OPERATE_TYPE_SYNC_TRAINING_FILE_START` | 3 |
| `enum_TRAINING_PLAN_OPERATE_TYPE_SYNC_TRAINING_FILE_END` | 4 |
| `enum_TRAINING_PLAN_OPERATE_TYPE_UPDATE_SCHEDULE_STATE_INFO` | 5 |

## Messages

### `schedule_data_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `schedule_date_operate_type` | `TRAINING_PLAN_OPERATE_TYPE` |
| 3 | `operate_type` | `Common.SERVICE_OPERATE_TYPE` |
| 4 | `file_number` | `int32` |
| 5 | `utc_time` | `int32` |
| 6 | `schedule_name` | `string` |
