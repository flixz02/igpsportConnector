# TeamInfo

_Generated from `com.igpsport.blelib.bean.TeamInfo`._

## Enums

### `MEMBER_STATE_TYPE`

| Name | Value |
|---|---:|
| `enum_MEMBER_STATE_TYPE_INVALID` | 0 |
| `enum_MEMBER_STATE_TYPE_NORMAL` | 1 |
| `enum_MEMBER_STATE_TYPE_ABNORMAL` | 2 |
| `enum_MEMBER_STATE_TYPE_OFFLINE` | 3 |
| `enum_MEMBER_STATE_TYPE_JOIN` | 4 |
| `enum_MEMBER_STATE_TYPE_QUIT` | 5 |

### `TEAM_INFO_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_TEAM_INFO_OPERATE_TYPE_NONE` | 0 |
| `enum_TEAM_INFO_OPERATE_TYPE_SET` | 1 |

## Messages

### `team_info_data_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `latitude` | `double` |
| 2 | `longitude` | `double` |
| 3 | `status` | `int32` |
| 4 | `nick_name` | `string` |
| 5 | `course` | `int32` |

### `team_info_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `team_info_operate_type` | `TEAM_INFO_OPERATE_TYPE` |
| 3 | `member_num` | `int32` |
| 4 | `team_info_data_message` | `repeated team_info_data_msg` |
