# UserConfig

_Generated from `com.igpsport.blelib.bean.UserConfig`._

## Enums

### `USER_CONFIG_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_USER_CONFIG_OPERATE_TYPE_NONE` | 0 |
| `enum_USER_CONFIG_OPERATE_TYPE_SET` | 1 |
| `enum_USER_CONFIG_OPERATE_TYPE_GET` | 2 |

## Messages

### `user_config_data_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `sex` | `int32` |
| 2 | `weight` | `int32` |
| 3 | `age` | `int32` |
| 4 | `height` | `int32` |
| 5 | `wheel_dia` | `int32` |
| 6 | `bike_weight` | `int32` |
| 7 | `time_zone` | `int32` |
| 8 | `member_id` | `string` |

### `user_config_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `user_config_operate_type` | `USER_CONFIG_OPERATE_TYPE` |
| 3 | `user_config_data_message` | `user_config_data_msg` |
