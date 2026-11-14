# Config

_Generated from `com.igpsport.blelib.bean.Config`._

## Enums

### `ALARM_SELECT`

| Name | Value |
|---|---:|
| `ALARM_SELECT_INVALID` | 0 |
| `ALARM_SELECT_RIDE_TIME` | 1 |
| `ALARM_SELECT_DISTANCE` | 2 |
| `ALARM_SELECT_CALORIE` | 3 |

### `ALARM_TYPE`

| Name | Value |
|---|---:|
| `ALARM_TYPE_INVALID` | 0 |
| `ALARM_TYPE_RIDE_TIME` | 1 |
| `ALARM_TYPE_RIDE_DISTANCE` | 2 |
| `ALARM_TYPE_HRM` | 3 |
| `ALARM_TYPE_CAD` | 4 |
| `ALARM_TYPE_PWR` | 5 |
| `ALARM_TYPE_CALORIE` | 6 |
| `ALARM_TYPE_SPEED` | 7 |
| `ALARM_TYPE_WATER` | 8 |
| `ALARM_TYPE_FOOD` | 9 |
| `ALARM_TYPE_CORE_TEMP` | 10 |

### `AUTO_TYPE`

| Name | Value |
|---|---:|
| `enum_AUTO_TYPE_INVALID` | 0 |
| `enum_PAUSE` | 1 |
| `enum_RECORD_START` | 2 |
| `enum_POWER_OFF` | 3 |
| `enum_SLEEP` | 4 |
| `enum_HOME_PAGE_BACK` | 5 |
| `enum_SMART_SAVE` | 6 |
| `enum_POWER_SAVE` | 7 |
| `enum_PAGE_AUTO` | 8 |
| `enum_MOTION_CHECK` | 9 |

### `CONFIG_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_CONFIG_OPERATE_TYPE_NONE` | 0 |
| `enum_CONFIG_OPERATE_TYPE_SET` | 1 |
| `enum_CONFIG_OPERATE_TYPE_GET` | 2 |
| `enum_CONFIG_OPERATE_TYPE_SEND` | 3 |
| `enum_CONFIG_OPERATE_TYPE_ADD` | 4 |
| `enum_CONFIG_OPERATE_TYPE_DEL` | 5 |
| `enum_CONFIG_OPERATE_TYPE_GET_MODULE_INFO` | 6 |
| `enum_CONFIG_OPERATE_TYPE_CTRL` | 7 |
| `enum_CONFIG_OPERATE_TYPE_SET_DEFAULT` | 8 |

### `CONFIG_SERVICE_TYPE`

| Name | Value |
|---|---:|
| `enum_CONFIG_SERVICE_TYPE_NONE` | 0 |
| `enum_CONFIG_SERVICE_TYPE_USER` | 1 |
| `enum_CONFIG_SERVICE_TYPE_PAGE` | 2 |
| `enum_CONFIG_SERVICE_TYPE_BIKE` | 3 |
| `enum_CONFIG_SERVICE_TYPE_UNIT` | 4 |
| `enum_CONFIG_SERVICE_TYPE_LANG` | 5 |
| `enum_CONFIG_SERVICE_TYPE_KEY` | 6 |
| `enum_CONFIG_SERVICE_TYPE_WHEEL` | 7 |
| `enum_CONFIG_SERVICE_TYPE_GPS` | 16 |
| `enum_CONFIG_SERVICE_TYPE_SOUND` | 17 |
| `enum_CONFIG_SERVICE_TYPE_POWER` | 18 |
| `enum_CONFIG_SERVICE_TYPE_DIS_COLOR` | 19 |
| `enum_CONFIG_SERVICE_TYPE_BK` | 20 |
| `enum_CONFIG_SERVICE_TYPE_ALARM` | 48 |
| `enum_CONFIG_SERVICE_TYPE_LAP` | 49 |
| `enum_CONFIG_SERVICE_TYPE_AUTO` | 50 |
| `enum_CONFIG_SERVICE_TYPE_MODE` | 51 |
| `enum_CONFIG_SERVICE_TYPE_ALTITUDE` | 52 |
| `enum_CONFIG_SERVICE_TYPE_DATA` | 53 |
| `enum_CONFIG_SERVICE_TYPE_MODE_TEMPLATE` | 54 |

### `DATA_TYPE`

| Name | Value |
|---|---:|
| `enum_DATA_TYPE_INVALID` | 0 |
| `enum_DATA_TYPE_PWR_WITH_ZERO` | 1 |
| `enum_DATA_TYPE_CAD_WITH_ZERO` | 2 |

### `KEY_FUNCTION_TYPE`

| Name | Value |
|---|---:|
| `enum_KEY_FUNCTION_TYPE_INVALID` | 0 |
| `enum_RECORD_STATUS_MANUAL_PAUSE` | 1 |
| `enum_RECORD_STATUS_MANUAL_LAP` | 2 |
| `enum_RECORD_STATUS_FRONT_LIGHT` | 3 |
| `enum_RECORD_STATUS_TAIL_LIGHT` | 4 |

### `KEY_TYPE`

| Name | Value |
|---|---:|
| `enum_KEY_TYPE_INVALID` | 0 |
| `enum_KEY1` | 1 |
| `enum_KEY2` | 2 |
| `enum_KEY3` | 3 |
| `enum_KEY4` | 4 |
| `enum_KEY5` | 5 |
| `enum_KEY6` | 6 |

### `LANGUAGE_TYPE`

| Name | Value |
|---|---:|
| `enum_LANGUAGE_TYPE_INVALID` | 0 |
| `enum_LANGUAGE_TYPE_ENGLISH` | 1 |
| `enum_LANGUAGE_TYPE_SPANISH` | 2 |
| `enum_LANGUAGE_TYPE_FRENCH` | 3 |
| `enum_LANGUAGE_TYPE_GERMAN` | 4 |
| `enum_LANGUAGE_TYPE_JAPANESE` | 5 |
| `enum_LANGUAGE_TYPE_ITALIAN` | 6 |
| `enum_LANGUAGE_TYPE_PORTUGUESE` | 7 |
| `enum_LANGUAGE_TYPE_KOREAN` | 8 |
| `enum_LANGUAGE_TYPE_CHINESE` | 9 |
| `enum_LANGUAGE_TYPE_TAIWANESE` | 10 |
| `enum_LANGUAGE_TYPE_POLISH` | 11 |
| `enum_LANGUAGE_TYPE_CROATIAN` | 12 |
| `enum_LANGUAGE_TYPE_CZECH` | 13 |
| `enum_LANGUAGE_TYPE_DANISH` | 14 |
| `enum_LANGUAGE_TYPE_DUTCH` | 15 |
| `enum_LANGUAGE_TYPE_FINNISH` | 16 |
| `enum_LANGUAGE_TYPE_GREEK` | 17 |
| `enum_LANGUAGE_TYPE_HUNGARIAN` | 18 |
| `enum_LANGUAGE_TYPE_NORWEGIAN` | 19 |
| `enum_LANGUAGE_TYPE_SLOVAKIAN` | 20 |
| `enum_LANGUAGE_TYPE_SLOVENIAN` | 21 |
| `enum_LANGUAGE_TYPE_SWEDISH` | 22 |
| `enum_LANGUAGE_TYPE_RUSSIAN` | 23 |
| `enum_LANGUAGE_TYPE_TURKISH` | 24 |
| `enum_LANGUAGE_TYPE_LATVIAN` | 25 |
| `enum_LANGUAGE_TYPE_UKRAINIAN` | 26 |
| `enum_LANGUAGE_TYPE_ARABIC` | 27 |
| `enum_LANGUAGE_TYPE_FARSI` | 28 |
| `enum_LANGUAGE_TYPE_BULGARIAN` | 29 |
| `enum_LANGUAGE_TYPE_ROMANIAN` | 30 |
| `enum_LANGUAGE_TYPE_THAI` | 31 |
| `enum_LANGUAGE_TYPE_HEBREW` | 32 |
| `enum_LANGUAGE_TYPE_BRAZILIAN_PORTUGUESE` | 33 |
| `enum_LANGUAGE_TYPE_INDONESIAN` | 34 |
| `enum_LANGUAGE_TYPE_MALAYSIAN` | 35 |
| `enum_LANGUAGE_TYPE_VIETNAMESE` | 36 |
| `enum_LANGUAGE_TYPE_BURMESE` | 37 |
| `enum_LANGUAGE_TYPE_MONGOLIAN` | 38 |
| `enum_LANGUAGE_TYPE_ALL` | 100 |
| `enum_LANGUAGE_TYPE_CUSTOM` | 254 |

### `LAP_TYPE`

| Name | Value |
|---|---:|
| `LAP_TYPE_INVALID` | 0 |
| `LAP_TYPE_LOCATION` | 1 |
| `LAP_TYPE_TIME` | 2 |
| `LAP_TYPE_DISTANCE` | 3 |

### `LCD_TYPE`

| Name | Value |
|---|---:|
| `INVALID` | 0 |
| `PIXEL_LCD` | 1 |
| `SECTION_LCD` | 2 |

### `PAGE_AUTO_STATUS`

| Name | Value |
|---|---:|
| `PAGE_AUTO_STATUS_OFF` | 0 |
| `PAGE_AUTO_STATUS_ON` | 1 |
| `PAGE_AUTO_STATUS_FAST` | 2 |
| `PAGE_AUTO_STATUS_SLOW` | 3 |
| `PAGE_AUTO_STATUS_MAIN` | 4 |

### `PAGE_TYPE`

| Name | Value |
|---|---:|
| `enum_PAGE_TYPE_INVALID` | 0 |
| `enum_PAGE_TYPE_DATA` | 1 |
| `enum_PAGE_TYPE_ELEVATION` | 2 |
| `enum_PAGE_TYPE_MAP` | 3 |
| `enum_PAGE_TYPE_AREA` | 4 |
| `enum_PAGE_TYPE_TRAINING_COURSE` | 5 |
| `enum_PAGE_TYPE_TRAINING_FEC` | 6 |
| `enum_PAGE_TYPE_COMPASS` | 7 |
| `enum_PAGE_TYPE_LAP` | 8 |
| `enum_PAGE_TYPE_ROADBOOK` | 9 |
| `enum_PAGE_TYPE_CLIMB` | 10 |
| `enum_PAGE_TYPE_AIPARTNER` | 11 |
| `enum_PAGE_TYPE_SEGMENT` | 12 |
| `enum_PAGE_TYPE_FITNESS` | 13 |
| `enum_PAGE_TYPE_CLIMB_MAP` | 14 |
| `enum_PAGE_TYPE_GROUP_RIDE_INFOR` | 15 |
| `enum_PAGE_TYPE_MUSIC` | 16 |
| `enum_PAGE_TYPE_CAMERA` | 17 |

### `POWER_SAVE_STATUS`

| Name | Value |
|---|---:|
| `POWER_SAVE_STATUS_OFF` | 0 |
| `POWER_SAVE_STATUS_ON` | 1 |
| `POWER_SAVE_STATUS_SMART` | 2 |

### `SOUND_SCENE_TYPE`

| Name | Value |
|---|---:|
| `enum_SOUND_SCENE_TYPE_INVALID` | 0 |
| `enum_ALL_SCENE` | 1 |
| `enum_RECORDING_SCENE` | 2 |

### `SOUND_TYPE`

| Name | Value |
|---|---:|
| `enum_SOUND_TYPE_INVALID` | 0 |
| `enum_SOUND_RECORD_START` | 1 |
| `enum_SOUND_RECORD_PAUSE` | 2 |
| `enum_SOUND_RECORD_SAVE` | 4 |
| `enum_SOUND_LAP` | 8 |
| `enum_SOUND_ALART` | 16 |
| `enum_SOUND_KEY` | 32 |
| `enum_SOUND_USB` | 64 |
| `enum_SOUND_GPS` | 128 |
| `enum_SOUND_SENSOR` | 256 |
| `enum_SOUND_CALL` | 512 |
| `enum_SOUND_SOCIAL` | 1024 |
| `enum_SOUND_BURGLAR_ALARM` | 2048 |
| `enum_SOUND_DEV_FIND` | 4096 |

### `UNIT_ITEM`

| Name | Value |
|---|---:|
| `enum_UNIT_ITEM_INVALID` | 0 |
| `enum_UNIT_ITEM_DISTANCE` | 1 |
| `enum_UNIT_ITEM_ELEVATION` | 2 |
| `enum_UNIT_ITEM_WEIGHT` | 3 |
| `enum_UNIT_ITEM_TEMPERATURE` | 4 |

### `UNIT_TYPE`

| Name | Value |
|---|---:|
| `enum_UNIT_TYPE_INVALID` | 0 |
| `enum_UNIT_TYPE_METRIC` | 1 |
| `enum_UNIT_TYPE_INCH` | 2 |

## Messages

### `alarm_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `set_msg` | `repeated alarm_params_set_msg` |
| 2 | `alarm_interval_time` | `int32` |

### `alarm_params_set_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `alarm_type` | `ALARM_TYPE` |
| 2 | `value_max` | `int32` |
| 3 | `value_min` | `int32` |
| 4 | `status` | `int32` |
| 5 | `select_type` | `ALARM_SELECT` |
| 6 | `select_info` | `repeated alarm_select_info` |

### `alarm_select_info`

| # | Field | Type |
|---:|---|---|
| 1 | `select_type` | `ALARM_SELECT` |
| 2 | `value` | `int32` |

### `auto_set_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `auto_type` | `AUTO_TYPE` |
| 2 | `status` | `int32` |
| 3 | `param1` | `int32` |

### `backlight_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `backlight_night_on` | `int32` |
| 2 | `backling_time` | `int32` |
| 3 | `backlight_auto` | `int32` |
| 4 | `backlight_day_percent` | `int32` |
| 5 | `backlight_night_percent` | `int32` |

### `bike_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `bike_index` | `int32` |
| 2 | `bike_name` | `string` |
| 3 | `bike_weigth` | `int32` |
| 4 | `wheel_dia` | `int32` |
| 5 | `odometer` | `int64` |
| 6 | `auto_dia` | `int32` |
| 7 | `bike_status` | `int32` |

### `config_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `config_sevice_type` | `CONFIG_SERVICE_TYPE` |
| 3 | `config_operate_type` | `CONFIG_OPERATE_TYPE` |
| 4 | `user_data_message` | `user_data_msg` |
| 5 | `page_message` | `repeated page_msg` |
| 6 | `bike_message` | `repeated bike_msg` |
| 7 | `unit_message` | `repeated unit_msg` |
| 8 | `page_status_message` | `cur_page_status_msg` |
| 9 | `altitude` | `int32` |
| 12 | `language_message` | `language_msg` |
| 13 | `backlight_message` | `backlight_msg` |
| 19 | `cur_operate_mode` | `int32` |
| 20 | `mode_message` | `repeated mode_msg` |
| 21 | `alarm_message` | `repeated alarm_msg` |
| 22 | `lap_message` | `repeated lap_msg` |
| 23 | `auto_set_message` | `repeated auto_set_msg` |
| 24 | `key_set_message` | `repeated key_set_msg` |
| 25 | `sound_set_message` | `repeated sound_set_msg` |
| 26 | `data_message` | `repeated data_msg` |

### `cur_page_status_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `page_line_num_max` | `int32` |
| 2 | `page_line_data_max` | `int32` |
| 3 | `data_page_num_max` | `int32` |
| 4 | `unsupport_data` | `repeated int32` |
| 5 | `lcd_type` | `LCD_TYPE` |
| 6 | `support_page_layout` | `repeated int32` |
| 11 | `main_page_open` | `int32` |

### `data_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `data_type` | `DATA_TYPE` |
| 2 | `status` | `int32` |

### `key_set_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `key_type` | `KEY_TYPE` |
| 2 | `key_function_type` | `KEY_FUNCTION_TYPE` |
| 3 | `key_support` | `repeated KEY_FUNCTION_TYPE` |

### `language_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `cur_language` | `LANGUAGE_TYPE` |
| 2 | `supported_language` | `repeated LANGUAGE_TYPE` |

### `lap_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `lap_type` | `LAP_TYPE` |
| 2 | `value` | `int32` |
| 3 | `status` | `int32` |

### `mode_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `mode_index` | `int32` |
| 2 | `mode_color` | `int32` |
| 3 | `valid` | `int32` |
| 4 | `inuse` | `int32` |
| 5 | `template_index` | `int32` |
| 6 | `mode_name` | `string` |

### `page_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `page_index` | `int32` |
| 2 | `status` | `int32` |
| 3 | `data` | `repeated int32` |
| 4 | `data_site` | `int32` |
| 5 | `page_name` | `string` |
| 6 | `page_type` | `PAGE_TYPE` |
| 7 | `page_mode` | `int32` |
| 8 | `main_page_status` | `int32` |
| 9 | `line_width` | `int32` |
| 10 | `graphic_display` | `int32` |
| 11 | `forbidden_close` | `int32` |

### `section_data_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `hrm` | `repeated int32` |
| 2 | `cad` | `repeated int32` |
| 3 | `spd` | `repeated int32` |
| 10 | `ftp` | `int32` |
| 11 | `power` | `repeated int32` |

### `sound_set_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `sound_type` | `int32` |
| 2 | `sound_scene` | `SOUND_SCENE_TYPE` |
| 3 | `status` | `int32` |

### `unit_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `unit_item` | `UNIT_ITEM` |
| 2 | `unit_type` | `UNIT_TYPE` |

### `user_data_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `sex` | `int32` |
| 2 | `weight` | `int32` |
| 3 | `age` | `int32` |
| 4 | `height` | `int32` |
| 7 | `time_zone` | `int32` |
| 8 | `member_id` | `string` |
| 9 | `update_status` | `int32` |
| 11 | `ride_hr_max` | `int32` |
| 13 | `ride_hr_lactic_acid` | `int32` |
| 20 | `section_data` | `section_data_msg` |
| 21 | `personal_infor_change_time` | `int32` |
