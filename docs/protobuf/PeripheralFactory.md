# PeripheralFactory

_Generated from `com.igpsport.blelib.bean.PeripheralFactory`._

## Enums

### `PERIPHERAL_FACTORY_OPERATE`

| Name | Value |
|---|---:|
| `FAO_UNSPECIFIED` | 0 |
| `UNRECOGNIZED` | -1 |

### `PERIPHERAL_FACTORY_RADAR_TEST_OPERATE`

| Name | Value |
|---|---:|
| `FAS_NSPECIFIED_RADAR` | 0 |
| `FAS_RADAR_TEST_OPEN` | 1 |
| `FAS_RADAR_TEST_CLOSE` | 2 |
| `UNRECOGNIZED` | -1 |

### `PERIPHERAL_FACTORY_SERVICE`

| Name | Value |
|---|---:|
| `FAS_UNSPECIFIED` | 0 |
| `FAS_SN` | 1 |
| `FAS_DEVICE_INFO` | 2 |
| `FAS_DEVICE_OPERATE` | 3 |
| `FAS_DEVICE_CONFIG` | 4 |
| `FAS_BSP_MEMORY` | 21 |
| `FAS_BSP_RTC` | 22 |
| `FAS_BSP_BATTARY` | 23 |
| `FAS_BSP_TEMPERATURE` | 24 |
| `FAS_BSP_ACC` | 25 |
| `FAS_BSP_LSR` | 26 |
| `FAS_BSP_RGB` | 27 |
| `FAS_LED_TEST_PERI` | 28 |
| `FAS_THERMISTOR_VOL` | 29 |
| `FAS_ALL_FACTORY` | 41 |
| `FAS_LED_TEST_BUTTON` | 42 |
| `FAS_MOTOR_TEST_BUTTON` | 43 |
| `FAS_ENTER_FACTORY` | 44 |
| `FAS_RADAR_TEST` | 45 |
| `FAS_FACTORY_ARG` | 46 |
| `FAS_ANT_ID` | 47 |
| `FAS_CURRENT_TEST` | 48 |
| `FAS_VOID_WELDING_TEST` | 49 |
| `FAS_MAX_LUMEN_TEST` | 50 |
| `FAS_SLEEP_ENTER_TEST` | 51 |
| `FAS_RADAR_CALIB_FCAL` | 52 |
| `FAS_RADAR_CALIB_FDECT` | 53 |
| `UNRECOGNIZED` | -1 |

## Messages

### `peripheral_factory_data_ant_id`

| # | Field | Type |
|---:|---|---|
| 1 | `ant_id` | `int32` |

### `peripheral_factory_data_bsp_acc`

| # | Field | Type |
|---:|---|---|
| 1 | `x_axis_val` | `int32` |
| 2 | `y_axis_val` | `int32` |
| 3 | `z_axis_val` | `int32` |

### `peripheral_factory_data_bsp_battery`

| # | Field | Type |
|---:|---|---|
| 1 | `vol_cur` | `int32` |
| 2 | `percent` | `int32` |

### `peripheral_factory_data_bsp_lsr`

| # | Field | Type |
|---:|---|---|
| 1 | `lightness` | `int32` |

### `peripheral_factory_data_bsp_memory`

| # | Field | Type |
|---:|---|---|
| 1 | `remain_internal_flash` | `int32` |
| 2 | `total_internal_flash` | `int32` |
| 3 | `remain_iexternal_flash` | `int32` |
| 4 | `total_external_flash` | `int32` |

### `peripheral_factory_data_bsp_radar_raw`

| # | Field | Type |
|---:|---|---|
| 1 | `amplitude` | `int32` |
| 2 | `frequency` | `int32` |

### `peripheral_factory_data_bsp_radar_test`

| # | Field | Type |
|---:|---|---|
| 1 | `radar_test_cmd` | `PERIPHERAL_FACTORY_RADAR_TEST_OPERATE` |
| 2 | `radar_test_data` | `repeated peripheral_factory_data_bsp_radar_raw` |

### `peripheral_factory_data_bsp_rtc`

| # | Field | Type |
|---:|---|---|
| 1 | `zone` | `int32` |
| 2 | `rtc` | `int32` |
| 3 | `sunrise` | `int32` |
| 4 | `sunset` | `int32` |

### `peripheral_factory_data_bsp_temperature`

| # | Field | Type |
|---:|---|---|
| 1 | `temp_cur` | `int32` |
| 2 | `temp_max` | `int32` |

### `peripheral_factory_data_device_config`

| # | Field | Type |
|---:|---|---|
| 1 | `shutdown` | `PeripheralConfig.peripheral_config_data_shutdown` |
| 2 | `sleep` | `PeripheralConfig.peripheral_config_data_sleep` |

### `peripheral_factory_data_device_info`

| # | Field | Type |
|---:|---|---|
| 1 | `model` | `PeripheralInfo.PERIPHERAL_INFO_MODEL_TYPE` |
| 2 | `name` | `PeripheralInfo.peripheral_info_data_name` |
| 3 | `version` | `PeripheralInfo.peripheral_info_data_version` |
| 4 | `id` | `PeripheralInfo.peripheral_info_data_id` |

### `peripheral_factory_data_device_operate`

| # | Field | Type |
|---:|---|---|
| 1 | `hr_sport_mode` | `PeripheralHr.PERIPHERAL_HR_SPORT_MODE_TYPE` |

### `peripheral_factory_data_fcal_result`

| # | Field | Type |
|---:|---|---|
| 1 | `result` | `bool` |

### `peripheral_factory_data_radar_snr`

| # | Field | Type |
|---:|---|---|
| 1 | `snr` | `int32` |
| 2 | `signal` | `int32` |
| 3 | `noise` | `int32` |

### `peripheral_factory_data_sn`

| # | Field | Type |
|---:|---|---|
| 1 | `sn` | `string` |

### `peripheral_factory_data_therm_vol`

| # | Field | Type |
|---:|---|---|
| 1 | `vol` | `int32` |

### `peripheral_factory_data_void_welding`

| # | Field | Type |
|---:|---|---|
| 1 | `f7_reg_addr` | `int32` |
| 2 | `channel1` | `int32` |
| 3 | `channel2` | `int32` |
| 4 | `channel3` | `int32` |
| 5 | `channel4` | `int32` |
| 6 | `lock_state` | `int32` |
| 7 | `lock_val` | `int32` |
| 8 | `ana_val` | `int32` |

### `peripheral_factory_format`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `PeripheralCommon.PERIPHERAL_SERVICE_TYPE` |
| 2 | `operate_type` | `PeripheralCommon.PERIPHERAL_OPERATE_TYPE` |
| 3 | `sub_service_type` | `PERIPHERAL_FACTORY_SERVICE` |
| 4 | `sub_operate_type` | `PERIPHERAL_FACTORY_OPERATE` |
| 5 | `message` | `peripheral_factory_message` |

### `peripheral_factory_message`

| # | Field | Type |
|---:|---|---|
| 1 | `sn` | `peripheral_factory_data_sn` |
| 2 | `device_info` | `peripheral_factory_data_device_info` |
| 3 | `device_operate` | `peripheral_factory_data_device_operate` |
| 4 | `device_config` | `peripheral_factory_data_device_config` |
| 5 | `bsp_memory` | `peripheral_factory_data_bsp_memory` |
| 6 | `bsp_rtc` | `peripheral_factory_data_bsp_rtc` |
| 7 | `bsp_battery` | `peripheral_factory_data_bsp_battery` |
| 8 | `bsp_temperature` | `peripheral_factory_data_bsp_temperature` |
| 9 | `bsp_acc` | `peripheral_factory_data_bsp_acc` |
| 10 | `radar_test` | `peripheral_factory_data_bsp_radar_test` |
| 11 | `bsp_lsr` | `peripheral_factory_data_bsp_lsr` |
| 12 | `therm_vol` | `peripheral_factory_data_therm_vol` |
| 13 | `ant` | `peripheral_factory_data_ant_id` |
| 14 | `void_welding` | `peripheral_factory_data_void_welding` |
| 15 | `ota_data` | `peripheral_factory_data_radar_snr` |
| 16 | `fcal_result` | `peripheral_factory_data_fcal_result` |
| 17 | `self_test` | `peripheral_self_test_result` |

### `peripheral_self_test_result`

| # | Field | Type |
|---:|---|---|
| 1 | `result` | `bool` |
