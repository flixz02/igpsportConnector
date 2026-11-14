# Theme

_Generated from `com.igpsport.blelib.bean.Theme`._

## Enums

### `enum_bg_type`

| Name | Value |
|---|---:|
| `BG_TYPE_INVALID` | 0 |
| `BG_TYPE_POWERON` | 1 |
| `BG_TYPE_THEME1` | 2 |
| `BG_TYPE_THEME2` | 3 |
| `BG_TYPE_THEME3` | 4 |

### `enum_operate`

| Name | Value |
|---|---:|
| `THEME_OP_INVALID` | 0 |
| `THEME_OP_LIST_GET` | 1 |
| `THEME_OP_SET` | 2 |
| `THEME_OP_USE` | 3 |
| `THEME_OP_DEFAULT_BG` | 4 |
| `THEME_OP_GET_BG` | 5 |

### `enum_theme_dark_mode`

| Name | Value |
|---|---:|
| `THEME_DARK_MODE_DAY` | 0 |
| `THEME_DARK_MODE_NIGHT` | 1 |
| `THEME_DARK_MODE_AUTO` | 2 |

### `enum_theme_func_key`

| Name | Value |
|---|---:|
| `THEME_FUNC_KEY_INVALID` | 0 |
| `THEME_FUNC_KEY_RIDE` | 1 |
| `THEME_FUNC_KEY_WORKOUT` | 2 |
| `THEME_FUNC_KEY_NAVI` | 3 |
| `THEME_FUNC_KEY_THEME` | 4 |
| `THEME_FUNC_KEY_HISTORY` | 5 |
| `THEME_FUNC_KEY_SET` | 6 |
| `THEME_FUNC_KEY_ROADBOOK` | 7 |
| `THEME_FUNC_KEY_INS` | 8 |
| `THEME_FUNC_KEY_SAFETY_TRACK` | 9 |
| `THEME_FUNC_KEY_PM` | 10 |
| `THEME_FUNC_KEY_SENSOR` | 11 |
| `THEME_FUNC_KEY_SYSTEM` | 12 |
| `THEME_FUNC_KEY_WEATHER` | 13 |
| `THEME_FUNC_KEY_STAGE` | 14 |
| `THEME_FUNC_KEY_RECORD` | 15 |
| `THEME_FUNC_KEY_FITNESS_LEVEL` | 16 |
| `THEME_FUNC_KEY_TRAINNING_STATUS` | 17 |
| `THEME_FUNC_KEY_RECOVERY_STATUS` | 18 |
| `THEME_FUNC_KEY_GROUP_RIDE` | 19 |
| `THEME_FUNC_KEY_OFFLINE_PLAN` | 20 |
| `THEME_FUNC_KEY_TRACK_NAVI` | 21 |

### `enum_theme_height`

| Name | Value |
|---|---:|
| `THEME_KEY_HEIGHT_INVALID` | 0 |
| `THEME_KEY_HEIGHT_1` | 1 |
| `THEME_KEY_HEIGHT_2` | 2 |
| `THEME_KEY_HEIGHT_3` | 3 |
| `THEME_KEY_HEIGHT_4` | 4 |

### `enum_theme_status`

| Name | Value |
|---|---:|
| `THEME_STATUS_UNUSED` | 0 |
| `THEME_STATUS_USED` | 1 |

### `enum_theme_uitype`

| Name | Value |
|---|---:|
| `THEME_UITYPE_SIX_GRID` | 0 |
| `THEME_UITYPE_ROLLER` | 1 |
| `THEME_UITYPE_SPIN_ROLLER` | 2 |

### `enum_theme_width`

| Name | Value |
|---|---:|
| `THEME_KEY_WIDTH_INVALID` | 0 |
| `THEME_KEY_WIDTH_1` | 1 |
| `THEME_KEY_WIDTH_2` | 2 |
| `THEME_KEY_WIDTH_3` | 3 |
| `THEME_KEY_WIDTH_4` | 4 |

## Messages

### `theme_func_info`

| # | Field | Type |
|---:|---|---|
| 1 | `func_key` | `enum_theme_func_key` |
| 2 | `open` | `bool` |
| 3 | `theme_ui_width` | `enum_theme_width` |
| 4 | `theme_ui_height` | `enum_theme_height` |
| 5 | `is_removable` | `bool` |
| 6 | `is_closable` | `bool` |
| 7 | `ui` | `repeated ui_dimension` |

### `theme_infor_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `theme_id` | `int32` |
| 2 | `theme_index` | `int32` |
| 3 | `theme_status` | `enum_theme_status` |
| 4 | `func_key` | `repeated enum_theme_func_key` |
| 5 | `theme_uitype` | `enum_theme_uitype` |
| 6 | `theme_color` | `int32` |
| 7 | `func_info` | `repeated theme_func_info` |

### `theme_message`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `op_code` | `enum_operate` |
| 3 | `theme_info` | `repeated theme_infor_msg` |
| 4 | `dark_mode` | `enum_theme_dark_mode` |
| 5 | `bg_type` | `enum_bg_type` |
| 6 | `is_default_bg` | `int32` |

### `ui_dimension`

| # | Field | Type |
|---:|---|---|
| 1 | `theme_ui_width` | `enum_theme_width` |
| 2 | `theme_ui_height` | `enum_theme_height` |
