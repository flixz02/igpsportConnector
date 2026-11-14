# Factory

_Generated from `com.igpsport.blelib.bean.Factory`._

## Enums

### `ANT_DEV_TYPE`

| Name | Value |
|---|---:|
| `enumInvalid_dev_type` | 0 |
| `enumHrm_dev_type` | 1 |
| `enumCbsc_dev_type` | 2 |
| `enumBpwr_dev_type` | 3 |
| `enumCad_dev_type` | 5 |
| `enumSpd_dev_type` | 6 |
| `enumShft_dev_type` | 7 |
| `enumDi2_dev_type` | 8 |
| `enumFe_dev_type` | 9 |
| `enumMax_dev_type` | 10 |

### `CONTROL_CMD_TYPE`

| Name | Value |
|---|---:|
| `enum_CMD_INVALID` | 0 |
| `enum_CMD_ENTER_CHECK` | 1 |
| `enum_CMD_LOCK` | 2 |
| `enum_CMD_FORMAT` | 16 |
| `enum_CMD_LFORMAT` | 17 |
| `enum_CMD_PARA_RESET` | 18 |
| `enum_CMD_FACTORY_RESET` | 19 |
| `enum_CMD_TEMPERATURE_LOG` | 32 |
| `enum_CMD_VOLTAGE_LOG` | 33 |
| `enum_CMD_ANT_LOG` | 34 |

### `FACTORY_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_FACTORY_OPERATE_TYPE_NONE` | 0 |
| `enum_FACTORY_OPERATE_TYPE_SN_GET` | 1 |
| `enum_FACTORY_OPERATE_TYPE_SN_SEND` | 2 |
| `enum_FACTORY_OPERATE_TYPE_SN_SET` | 3 |
| `enum_FACTORY_OPERATE_TYPE_SENSOR_GET` | 4 |
| `enum_FACTORY_OPERATE_TYPE_MEMORY_GET` | 5 |
| `enum_FACTORY_OPERATE_TYPE_BATTARY_GET` | 6 |
| `enum_FACTORY_OPERATE_TYPE_SIM_FIT_SET` | 7 |
| `enum_FACTORY_OPERATE_TYPE_GPS_COORDINATE_SET` | 8 |
| `enum_FACTORY_OPERATE_TYPE_SUN_TIME_SET` | 9 |
| `enum_FACTORY_OPERATE_TYPE_SUN_TIME_GET` | 10 |
| `enum_FACTORY_OPERATE_TYPE_POWER_SAVE_TIME_SET` | 11 |
| `enum_FACTORY_OPERATE_TYPE_RTC_SET` | 12 |
| `enum_FACTORY_OPERATE_TYPE_RTC_GET` | 13 |
| `enum_FACTORY_OPERATE_TYPE_FILTER_SET` | 14 |
| `enum_FACTORY_OPERATE_TYPE_FILTER_GET` | 15 |
| `enum_FACTORY_OPERATE_TYPE_GPS_CONTROL_CMD_SET` | 16 |
| `enum_FACTORY_OPERATE_TYPE_GPS_CONTROL_CMD_GET` | 17 |
| `enum_FACTORY_OPERATE_TYPE_GPS_GNSS_SET` | 18 |
| `enum_FACTORY_OPERATE_TYPE_GPS_GNSS_GET` | 19 |
| `enum_FACTORY_OPERATE_TYPE_GPS_DYNAMIC_SET` | 20 |
| `enum_FACTORY_OPERATE_TYPE_GPS_DYNAMIC_GET` | 21 |
| `enum_FACTORY_OPERATE_TYPE_GPS_CMD_SET` | 22 |
| `enum_FACTORY_OPERATE_TYPE_CONTROL_SET` | 23 |
| `enum_FACTORY_OPERATE_TYPE_CONTROL_DATA_SET` | 24 |
| `enum_FACTORY_OPERATE_TYPE_ALL_GET` | 254 |

### `FACTORY_SENSOR_TYPE`

| Name | Value |
|---|---:|
| `enum_SENSOR_INVALID` | 0 |
| `enum_GPS` | 1 |
| `enum_RTC` | 2 |
| `enum_AIR` | 3 |
| `enum_TEM` | 4 |
| `enum_ACC` | 5 |
| `enum_ANT` | 6 |
| `enum_KEY` | 7 |

### `FILTER_TYPE`

| Name | Value |
|---|---:|
| `enum_FILTER_INVALID` | 0 |
| `enum_FILTER_SMART_SAVE` | 1 |
| `enum_FILTER_SPD` | 2 |
| `enum_FILTER_CAD` | 3 |
| `enum_FILTER_HRM` | 4 |
| `enum_FILTER_GPS_ICON` | 5 |
| `enum_FLTER_ANT_ICON` | 6 |

### `GNSS_TYPE`

| Name | Value |
|---|---:|
| `enum_GNSS_INVALID` | 0 |
| `enum_GNSS_GPS` | 1 |
| `enum_GNSS_BD` | 2 |
| `enum_GNSS__GLONASS` | 4 |
| `enum_GNSS__QZSS` | 8 |
| `enum_GNSS_SBAS` | 16 |
| `enum_GNSS_GALILEO` | 32 |
| `enum_GNSS_IMES` | 64 |

### `GPS_CONTROL_CMD_TYPE`

| Name | Value |
|---|---:|
| `enum_GPS_CMD_INVALID` | 0 |
| `enum_GPS_CMD_POWER` | 1 |
| `enum_GPS_CMD_COLD_START` | 2 |
| `enum_GPS_CMD_POWER_SAVE` | 3 |
| `enum_GPS_CMD_GNSS` | 4 |

### `GPS_DYNAMIC_TYPE`

| Name | Value |
|---|---:|
| `enum_GPS_DYNAMIC_INVALID` | 0 |
| `euum_PORTABLE` | 1 |
| `enum_STATIONARY` | 2 |
| `enum_PEDESTRIAN` | 3 |
| `enum_AUTOMOTIVE` | 4 |
| `enum_SEA` | 5 |
| `enum_1G` | 6 |
| `enum_2G` | 7 |
| `enum_4G` | 8 |
| `enum_WRIST` | 9 |
| `enum_BIKE` | 10 |

## Messages

### `ant_message`

| # | Field | Type |
|---:|---|---|
| 1 | `dev_type` | `ANT_DEV_TYPE` |
| 2 | `dev_connect_status` | `int32` |
| 3 | `data` | `int32` |

### `battary_message`

| # | Field | Type |
|---:|---|---|
| 1 | `voltage` | `int32` |
| 2 | `power_percent` | `int32` |

### `control_message`

| # | Field | Type |
|---:|---|---|
| 1 | `cmd_type` | `CONTROL_CMD_TYPE` |
| 2 | `status` | `int32` |

### `data_set_message`

| # | Field | Type |
|---:|---|---|
| 1 | `data_type` | `int32` |
| 2 | `value` | `int32` |

### `factory_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `factory_operate_type` | `FACTORY_OPERATE_TYPE` |
| 3 | `factory_sn_msg` | `repeated factory_sn_message` |
| 4 | `factory_sensor_msg` | `repeated factory_sensor_message` |
| 5 | `memory_msg` | `memory_message` |
| 6 | `battary_msg` | `battary_message` |
| 7 | `sim_fit_msg` | `sim_fit_message` |
| 8 | `control_msg` | `control_message` |
| 9 | `gps_coordinate_msg` | `gps_coordinate_message` |
| 10 | `sun_time_msg` | `sun_time_message` |
| 11 | `power_save_msg` | `power_save_message` |
| 12 | `rtc_msg` | `rtc_message` |
| 13 | `filter_msg` | `filter_message` |
| 14 | `gps_control_cmd_msg` | `gps_control_cmd_message` |
| 15 | `gps_gnss_msg` | `gps_gnss_message` |
| 16 | `gps_dynamic_msg` | `gps_dynamic_message` |
| 17 | `gps_cmd_msg` | `gps_cmd_message` |
| 18 | `data_set_msg` | `repeated data_set_message` |
| 19 | `gps_snr_msg` | `repeated gps_snr_massage` |
| 20 | `ant_msg` | `repeated ant_message` |

### `factory_sensor_message`

| # | Field | Type |
|---:|---|---|
| 1 | `sensor_type` | `FACTORY_SENSOR_TYPE` |
| 2 | `data` | `int32` |
| 3 | `status` | `int32` |

### `factory_sn_message`

| # | Field | Type |
|---:|---|---|
| 1 | `sn` | `string` |
| 2 | `go_more_key` | `string` |

### `filter_message`

| # | Field | Type |
|---:|---|---|
| 1 | `filter_type` | `FILTER_TYPE` |
| 2 | `status` | `int32` |

### `gps_cmd_message`

| # | Field | Type |
|---:|---|---|
| 1 | `gps_cmd` | `repeated bytes` |

### `gps_control_cmd_message`

| # | Field | Type |
|---:|---|---|
| 1 | `gps_cmd_type` | `GPS_CONTROL_CMD_TYPE` |
| 2 | `status` | `int32` |

### `gps_coordinate_message`

| # | Field | Type |
|---:|---|---|
| 1 | `latitude` | `double` |
| 2 | `longitude` | `double` |

### `gps_dynamic_message`

| # | Field | Type |
|---:|---|---|
| 1 | `gps_dynamic_type` | `GPS_DYNAMIC_TYPE` |

### `gps_gnss_message`

| # | Field | Type |
|---:|---|---|
| 1 | `gnss_gnss_cmd` | `int32` |

### `gps_snr_massage`

| # | Field | Type |
|---:|---|---|
| 1 | `gnss_type` | `GNSS_TYPE` |
| 2 | `data` | `repeated int32` |

### `memory_message`

| # | Field | Type |
|---:|---|---|
| 1 | `total` | `int32` |
| 2 | `remain` | `int32` |

### `power_save_message`

| # | Field | Type |
|---:|---|---|
| 1 | `time` | `int32` |

### `rtc_message`

| # | Field | Type |
|---:|---|---|
| 1 | `time` | `int32` |

### `sim_fit_message`

| # | Field | Type |
|---:|---|---|
| 1 | `num` | `int32` |
| 2 | `size` | `int32` |

### `sun_time_message`

| # | Field | Type |
|---:|---|---|
| 1 | `sunrise_time` | `int32` |
| 2 | `sunset_time` | `int32` |
