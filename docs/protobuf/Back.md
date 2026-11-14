# Back

_Generated from `com.igpsport.blelib.bean.Back`._

## Enums

### `AGPS_TYPE`

| Name | Value |
|---|---:|
| `enum_AGPS_FILE_TYPE_INVALID` | 0 |
| `enum_AGPS_FILE_TYPE_ONLINE` | 1 |
| `enum_AGPS_FILE_TYPE_ANO_OFFLINE` | 2 |
| `enum_AGPS_FILE_TYPE_ALM_OFFLINE` | 3 |

### `BACK_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_BACK_OPERATE_TYPE_NONE` | 0 |
| `enum_BACK_OPERATE_TYPE_GET` | 1 |
| `enum_BACK_OPERATE_TYPE_SEND` | 2 |

### `BACK_SERVICE_TYPE`

| Name | Value |
|---|---:|
| `enum_BACK_SERVICE_TYPE_NONE` | 0 |
| `enum_BACK_SERVICE_TYPE_MAIN` | 1 |
| `enum_BACK_SERVICE_TYPE_WEATHER` | 2 |
| `enum_BACK_SERVICE_TYPE_AIR_PRESSURE` | 3 |
| `enum_BACK_SERVICE_TYPE_ELEVATION` | 4 |
| `enum_BACK_SERVICE_TYPE_EPHEMERIS` | 5 |
| `enum_BACK_SERVICE_TYPE_TIME` | 6 |
| `enum_BACK_SERVICE_TYPE_LOCATE_INFO` | 7 |

### `BACK_SUB_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_BACK_SUB_OPERATE_TYPE_INVALID` | 0 |
| `enum_BACK_SUB_OPERATE_TYPE_EPHEMERIS_PART_6H` | 1 |
| `enum_BACK_SUB_OPERATE_TYPE_DEFAULT` | 255 |

### `GPS_TYPE`

| Name | Value |
|---|---:|
| `enum_GPS_TYPE_INVALID` | 0 |
| `enum_GPS_TYPE_GPS` | 1 |
| `enum_GPS_TYPE_BD` | 2 |
| `enum_GPS_TYPE_GLONASS` | 3 |
| `enum_GPS_TYPE_GALILEO` | 4 |

## Messages

### `air_pressure_data_message`

| # | Field | Type |
|---:|---|---|
| 1 | `air_pressure` | `int32` |

### `back_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `back_service_type` | `BACK_SERVICE_TYPE` |
| 3 | `back_operate_type` | `BACK_OPERATE_TYPE` |
| 4 | `three_days_msg` | `repeated weather_three_days_data_message` |
| 5 | `cur_msg` | `weather_current_data_message` |
| 6 | `three_hours_msg` | `repeated weather_three_hour_data_memsage` |
| 7 | `air_pressure_msg` | `air_pressure_data_message` |
| 8 | `ephemeris_data_msg` | `ephemeris_data_message` |
| 9 | `time_msg` | `time_data_message` |
| 10 | `locate_info_msg` | `locate_info_data_message` |

### `ephemeris_data_message`

| # | Field | Type |
|---:|---|---|
| 1 | `file_name` | `string` |
| 2 | `contents` | `bytes` |
| 3 | `gps_type` | `GPS_TYPE` |
| 4 | `agps_type` | `AGPS_TYPE` |
| 5 | `time` | `int32` |

### `locate_info_data_message`

| # | Field | Type |
|---:|---|---|
| 1 | `latitude` | `double` |
| 2 | `longitude` | `double` |

### `time_data_message`

| # | Field | Type |
|---:|---|---|
| 1 | `time` | `int32` |
| 2 | `time_zone` | `int32` |
| 3 | `utc_timestamp_ms` | `int64` |

### `weather_current_data_message`

| # | Field | Type |
|---:|---|---|
| 1 | `cur_temperature` | `int32` |
| 2 | `cur_weather` | `int32` |
| 3 | `cur_day_max_temp` | `int32` |
| 4 | `cur_day_min_temp` | `int32` |
| 5 | `time` | `string` |
| 6 | `wind_deg` | `string` |
| 7 | `wind_spd` | `string` |
| 9 | `humidity` | `int32` |
| 10 | `air_quality` | `int32` |
| 11 | `region` | `string` |

### `weather_three_days_data_message`

| # | Field | Type |
|---:|---|---|
| 1 | `weather_index` | `int32` |
| 2 | `max_temp` | `int32` |
| 3 | `min_temp` | `int32` |
| 4 | `rain_prob` | `int32` |
| 5 | `date` | `string` |
| 6 | `sun_rise_time` | `string` |
| 7 | `sun_set_time` | `string` |

### `weather_three_hour_data_memsage`

| # | Field | Type |
|---:|---|---|
| 1 | `wather_index` | `int32` |
| 2 | `temp` | `int32` |
| 3 | `rain_prob` | `int32` |
| 4 | `time` | `string` |
| 5 | `wind_deg` | `string` |
| 6 | `wind_spd` | `string` |
