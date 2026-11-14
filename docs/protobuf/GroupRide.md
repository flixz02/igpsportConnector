# GroupRide

_Generated from `com.igpsport.blelib.bean.GroupRide`._

## Enums

### `exit_reason`

| Name | Value |
|---|---:|
| `enum_INVALID_REASON` | 0 |
| `enum_VOLUNTARY` | 1 |
| `enum_TIMEOUT` | 2 |
| `enum_DISSOLVE` | 3 |
| `enum_KICKED_OUT` | 4 |

### `group_ride_msg_type`

| Name | Value |
|---|---:|
| `enum_INVALID_TYPE` | 0 |
| `enum_ONLINE_STATUS_COMFIRM_TYPE` | 1 |
| `enum_MEMBER_PERSONAL_INFOR` | 2 |
| `enum_MEMBER_GROUP_RIDE_INFOR` | 3 |
| `enum_MEMBER_DANGROUS_INFOR` | 4 |
| `enum_GROUP_RIDE_CTRL_ENTER` | 5 |
| `enum_GROUP_RIDE_CTRL_EXIT` | 6 |
| `enum_GROUP_RIDE_CTRL_START` | 7 |
| `enum_GROUP_RIDE_CTRL_FILE_START` | 8 |
| `enum_GROUP_RIDE_CTRL_FILE_END` | 9 |
| `enum_NOTICE_MSG` | 10 |

### `group_ride_notice_cmd`

| Name | Value |
|---|---:|
| `enum_INVALID_CMD` | 0 |
| `enum_ONLINE_STATUS_COMFIRM` | 1 |
| `enum_EMERGENCY_CALLING` | 2 |
| `enum_DANGEROUS_POINT_MARK` | 3 |
| `enum_GROUP_RIDE_ENTER` | 4 |
| `enum_GROUP_RIDE_EXIT` | 5 |
| `enum_GROUP_RIDE_REJECT` | 6 |
| `enum_ROUTE_UPDTAE_ACCEPT` | 7 |

### `member_identity_infor`

| Name | Value |
|---|---:|
| `enum_INVALID_IDENTITY_INFOR` | 0 |
| `enum_GROUP_RIDE_LEADER` | 1 |
| `enum_GROUP_RIDE_VICE_LEADER` | 2 |
| `enum_GROUP_RIDE_MEMBER` | 3 |

### `member_status`

| Name | Value |
|---|---:|
| `enum_GROUP_RIDE_INVALID_STATUS` | 0 |
| `enum_EXIT` | 1 |
| `enum_ONLINE` | 2 |
| `enum_OFFLINE` | 3 |

## Messages

### `group_ride_ctrl_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `group_ride_id` | `int32` |
| 2 | `status` | `int32` |
| 3 | `reason` | `exit_reason` |
| 4 | `group_ride_code` | `string` |

### `group_ride_infor_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `member_id` | `int32` |
| 2 | `status` | `member_status` |
| 3 | `latitude` | `double` |
| 4 | `longitude` | `double` |
| 5 | `stright_line_distance` | `int32` |
| 6 | `sos` | `int32` |

### `group_ride_message`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `operate_type` | `Common.SERVICE_OPERATE_TYPE` |
| 3 | `msg_type` | `group_ride_msg_type` |
| 4 | `online_status` | `online_status_msg` |
| 5 | `personal_infor` | `repeated personal_infor_msg` |
| 6 | `ride_infor` | `repeated group_ride_infor_msg` |
| 7 | `ctrl` | `group_ride_ctrl_msg` |

### `online_status_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `status` | `int32` |
| 2 | `group_ride_id` | `int32` |

### `personal_infor_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `member_id` | `int32` |
| 2 | `member_identity` | `member_identity_infor` |
| 3 | `nikename` | `string` |
| 4 | `is_owner` | `int32` |
