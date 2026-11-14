# EmojiLight

_Generated from `com.igpsport.blelib.bean.EmojiLight`._

## Enums

### `EMOJI_ATTRIBUTE_TYPE`

| Name | Value |
|---|---:|
| `EAT_SPD` | 0 |
| `EAT_CAD` | 1 |
| `EAT_HR` | 2 |
| `EAT_PWR` | 3 |
| `EAT_DIS` | 4 |
| `EAT_TIME` | 5 |
| `UNRECOGNIZED` | -1 |

### `EMOJI_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `EOT_INVALID` | 0 |
| `EOT_DOWNLOAD_START` | 1 |
| `EOT_DOWNLOAD_ING` | 2 |
| `EOT_DOWNLOAD_SAVE` | 3 |
| `EOT_DOWNLOAD_PREVIEW` | 4 |
| `UNRECOGNIZED` | -1 |

### `EMOJI_SERVICE_TYPE`

| Name | Value |
|---|---:|
| `EST_INVALID` | 0 |
| `EST_BASE_MSG` | 1 |
| `EST_EMOJI_DOWNLOAD` | 2 |
| `EST_EMOJI_DELETE` | 3 |
| `EST_EMOJI_MODIFY` | 4 |
| `EST_EMOJI_DETAIL` | 5 |
| `EST_EMOJI_ABBR` | 6 |
| `EST_EMOJI_SWITCH` | 7 |
| `EST_EMOJI_CURRENT_USE` | 8 |
| `UNRECOGNIZED` | -1 |

### `SINGLE_EMOJI_MODE`

| Name | Value |
|---|---:|
| `SEM_INVALID` | 0 |
| `SEM_MODE_0` | 1 |
| `SEM_MODE_1` | 2 |
| `SEM_MODE_2` | 3 |
| `SEM_MODE_3` | 4 |
| `SEM_MODE_4` | 5 |
| `SEM_MODE_5` | 6 |
| `SEM_MODE_6` | 7 |
| `UNRECOGNIZED` | -1 |

## Messages

### `emoji_base_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `raw_size` | `int32` |
| 2 | `col_size` | `int32` |
| 3 | `bat_pct` | `int32` |
| 4 | `version` | `int32` |
| 5 | `cur_emoji` | `int32` |
| 6 | `emoji_cnt` | `int32` |
| 7 | `saved_msg` | `repeated emoji_detail_msg` |
| 8 | `max_emoji_cnt` | `int32` |

### `emoji_delete_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `uid` | `int32` |

### `emoji_detail_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `uid` | `int32` |
| 2 | `attr` | `EMOJI_ATTRIBUTE_TYPE` |
| 3 | `intensity` | `int32` |
| 4 | `mode_index` | `emoji_index_msg` |
| 5 | `emoji_name` | `string` |
| 6 | `is_global_intensity` | `int32` |

### `emoji_index_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `emoji_mode` | `SINGLE_EMOJI_MODE` |

### `emoji_msg_format`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `PeripheralCommon.PERIPHERAL_SERVICE_TYPE` |
| 2 | `operate_type` | `PeripheralCommon.PERIPHERAL_OPERATE_TYPE` |
| 3 | `emoji_service` | `EMOJI_SERVICE_TYPE` |
| 4 | `emoji_operate` | `EMOJI_OPERATE_TYPE` |
| 5 | `msg_all` | `emoji_msgs_all` |

### `emoji_msgs_all`

| # | Field | Type |
|---:|---|---|
| 1 | `base_msg` | `emoji_base_msg` |
| 2 | `detail` | `emoji_detail_msg` |
| 3 | `delete` | `emoji_delete_msg` |
| 4 | `single` | `single_emoji_pb_stream_msg` |
| 5 | `my_abbr_emoji` | `my_emoji_abbr_msg` |

### `my_emoji_abbr_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `saved_msg` | `repeated emoji_detail_msg` |
| 2 | `uid_in_using` | `int32` |

### `single_emoji_pb_stream_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `pb_data` | `bytes` |
| 2 | `size` | `int32` |
