# Common

_Generated from `com.igpsport.blelib.bean.Common`._

## Enums

### `SERVICE_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_SERVICE_OPERATE_TYPE_NONE` | 0 |
| `enum_SERVICE_OPERATE_TYPE_SET` | 1 |
| `enum_SERVICE_OPERATE_TYPE_GET` | 2 |
| `enum_SERVICE_OPERATE_TYPE_ADD` | 3 |
| `enum_SERVICE_OPERATE_TYPE_DEL` | 4 |

### `service_type_index`

| Name | Value |
|---|---:|
| `enum_SERVICE_TYPE_INDEX_NONE` | 0 |
| `enum_SERVICE_TYPE_INDEX_INS` | 1 |
| `enum_SERVICE_TYPE_INDEX_MAP` | 2 |
| `enum_SERVICE_TYPE_INDEX_BACK` | 3 |
| `enum_SERVICE_TYPE_INDEX_FIRMWARE` | 4 |
| `enum_SERVICE_TYPE_INDEX_WIFI` | 5 |
| `enum_SERVICE_TYPE_INDEX_CYCLING_DATA` | 6 |
| `enum_SERVICE_TYPE_INDEX_ROUTE_PLAN` | 7 |
| `enum_SERVICE_TYPE_INDEX_REAL_TIME_TRACE` | 8 |
| `enum_SERVICE_TYPE_INDEX_USER_CONFIG` | 9 |
| `enum_SERVICE_TYPE_INDEX_BLE` | 10 |
| `enum_SERVICE_TYPE_INDEX_FACTORY` | 11 |
| `enum_SERVICE_TYPE_INDEX_CONFIG` | 12 |
| `enum_SERVICE_TYPE_INDEX_DEV_STATUS` | 13 |
| `enum_SERVICE_TYPE_INDEX_SENSOR` | 14 |
| `enum_SERVICE_TYPE_INDEX_TRAINING` | 15 |
| `enum_SERVICE_TYPE_INDEX_TEAM_INFO` | 16 |
| `enum_SERVICE_TYPE_INDEX_DEV_VER_INFO` | 17 |
| `enum_SERVICE_TYPE_INDEX_LANGUAGE` | 18 |
| `enum_SERVICE_TYPE_INDEX_LOG` | 19 |
| `enum_SERVICE_TYPE_INDEX_THEME` | 20 |
| `enum_SERVICE_TYPE_INDEX_FILE_OPERATION` | 21 |
| `enum_SERVICE_TYPE_INDEX_MAP_NEW` | 22 |
| `enum_SERVICE_TYPE_INDEX_ROUTE_BOOK` | 23 |
| `enum_SERVICE_TYPE_INDEX_STAGE` | 24 |
| `enum_SERVICE_TYPE_INDEX_TRAINING_PLAN` | 25 |
| `enum_SERVICE_TYPE_INDEX_ALG_GOMORE` | 26 |
| `enum_SERVICE_TYPE_INDEX_GROUP_RIDE` | 27 |
| `enum_SERVICE_TYPE_INDEX_CLOUD_SWITCH_DEV` | 28 |
| `enum_SERVICE_TYPE_INDEX_MUSIC_CTL` | 29 |
| `enum_SERVICE_TYPE_INDEX_WATCH_CONTROL` | 30 |
| `enum_SERVICE_TYPE_INDEX_WATCH_CONFIG` | 81 |
| `enum_SERVICE_TYPE_INDEX_WATCH_HEALTH` | 83 |
| `enum_SERVICE_TYPE_INDEX_WATCH_ALG_ACQUISITION` | 84 |
| `enum_SERVICE_TYPE_INDEX_WATCH_DIAL` | 85 |
| `enum_SERVICE_TYPE_INDEX_SPORT_PAGE_FILED` | 86 |
| `enum_SERVICE_TYPE_INDEX_SPORT_ABILITY` | 87 |

## Messages

### `file_list_get_message`

| # | Field | Type |
|---:|---|---|
| 1 | `file_num` | `int32` |
| 2 | `file_list_support_num_max` | `int32` |
| 3 | `file_index_start` | `int32` |
| 4 | `file_index_end` | `int32` |
