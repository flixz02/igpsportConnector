# Ins

_Generated from `com.igpsport.blelib.bean.Ins`._

## Enums

### `ANCS_CATAGORY_ID`

| Name | Value |
|---|---:|
| `OTHER` | 0 |
| `INCOMING_CALL` | 1 |
| `MISSED_CALL` | 2 |
| `VOICE_MAIL` | 3 |
| `SOCIAL` | 4 |
| `ID_SCHEDULE` | 5 |
| `ID_EMAIL` | 6 |
| `NEWS` | 7 |
| `HEALTH_AND_FITNESS` | 8 |
| `BUSINESS_AND_FINANCE` | 9 |
| `LOCATION` | 10 |
| `ENTERTAINMENT` | 11 |

### `INS_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_INS_OPERATE_TYPE_NONE` | 0 |
| `enum_INS_OPERATE_TYPE_CTRL` | 1 |
| `enum_INS_OPERATE_TYPE_INCOMING_CALL` | 2 |
| `enum_INS_OPERATE_TYPE_ANSWER_CALL` | 3 |
| `enum_INS_OPERATE_TYPE_REJECT_CALL` | 4 |
| `enum_INS_OPERATE_TYPE_CHECK_CALL` | 5 |
| `enum_INS_OPERATE_TYPE_INCOMING_NOTE` | 6 |
| `enum_INS_OPERATE_TYPE_CHECK_NOTE` | 7 |
| `enum_INS_OPERATE_TYPE_GET_SWITCH_STATUS` | 8 |
| `enum_INS_OPERATE_TYPE_SET_APP_LIST` | 9 |
| `enum_INS_OPERATE_TYPE_DEVICE_REJECT_CALL` | 10 |
| `enum_INS_OPERATE_TYPE_DEVICE_MUTE_CALL` | 11 |
| `enum_INS_OPERATE_TYPE_FIND_START` | 12 |
| `enum_INS_OPERATE_TYPE_FIND_STOP` | 13 |

### `INS_SERVICE_TYPE`

| Name | Value |
|---|---:|
| `enum_INS_SERVICE_TYPE_NONE` | 0 |
| `enum_INS_SERVICE_TYPE_MAIN` | 1 |
| `enum_INS_SERVICE_TYPE_CALL` | 2 |
| `enum_INS_SERVICE_TYPE_NOTE` | 3 |
| `enum_INS_SERVICE_TYPE_FIND_PHONE` | 4 |
| `enum_INS_SERVICE_TYPE_FIND_WATCH` | 5 |

## Messages

### `ancs_filter_message`

| # | Field | Type |
|---:|---|---|
| 1 | `catagory_id` | `ANCS_CATAGORY_ID` |
| 2 | `app_identifier` | `string` |
| 3 | `status` | `int32` |

### `ins_data_message`

| # | Field | Type |
|---:|---|---|
| 1 | `tel_num` | `bytes` |
| 2 | `name` | `string` |
| 3 | `content` | `string` |
| 4 | `time` | `string` |
| 5 | `uid` | `int32` |
| 6 | `count` | `int32` |
| 7 | `app_name` | `string` |
| 8 | `is_app` | `int32` |
| 9 | `pairing_code` | `bool` |
| 10 | `apk_name` | `string` |

### `ins_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `ins_service_type` | `INS_SERVICE_TYPE` |
| 3 | `ins_operate_type` | `INS_OPERATE_TYPE` |
| 4 | `ins_data_msg` | `ins_data_message` |
| 5 | `config` | `int32` |
| 6 | `ancs_filter_msg` | `repeated ancs_filter_message` |
