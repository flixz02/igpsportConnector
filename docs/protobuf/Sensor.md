# Sensor

_Generated from `com.igpsport.blelib.bean.Sensor`._

## Enums

### `DI2_BUTTON_OP_TYPE`

| Name | Value |
|---|---:|
| `enum_DI2_BUTTON_OP_TYPE_LONG` | 0 |
| `enum_DI2_BUTTON_OP_TYPE_SINGLE` | 1 |
| `enum_DI2_BUTTON_OP_TYPE_DOUBLE` | 2 |

### `DI2_CHN_NUM`

| Name | Value |
|---|---:|
| `enum_DI2_CHN_NUM_0` | 0 |
| `enum_DI2_CHN_NUM_1` | 1 |
| `enum_DI2_CHN_NUM_2` | 2 |
| `enum_DI2_CHN_NUM_3` | 3 |

### `DI2_FUNC_TYPE`

| Name | Value |
|---|---:|
| `enum_DI2_FUNC_INVALID` | 0 |
| `enum_DI2_FUNC_PAGE_UP` | 1 |
| `enum_DI2_FUNC_PAGE_DOWN` | 2 |
| `enum_DI2_FUNC_LAP` | 3 |
| `enum_DI2_FUNC_START_PAUSE` | 4 |
| `enum_DI2_FUNC_UNDEFINE` | 15 |

### `SENSOR_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_SENSOR_OPERATE_TYPE_NONE` | 0 |
| `enum_SENSOR_OPERATE_TYPE_GET` | 1 |
| `enum_SENSOR_OPERATE_TYPE_SET` | 2 |
| `enum_SENSOR_OPERATE_TYPE_DEL` | 3 |
| `enum_SENSOR_OPERATE_TYPE_CONNECT` | 4 |
| `enum_SENSOR_OPERATE_TYPE_SEND` | 5 |
| `enum_SENSOR_OPERATE_TYPE_EXIT` | 6 |
| `enum_SENSOR_OPERATE_TYPE_POW_CALIB` | 7 |
| `enum_SENSOR_OPERATE_TYPE_SYNC` | 8 |
| `enum_SENSOR_OPERATE_TYPE_QUERY` | 9 |

### `SENSOR_RADIO_TYPE`

| Name | Value |
|---|---:|
| `enum_SENSOR_RADIO_TYPE_INVALID` | 0 |
| `enum_SENSOR_RADIO_TYPE_BLE` | 1 |
| `enum_SENSOR_RADIO_TYPE_ANT` | 2 |

### `SENSOR_STATUS_TYPE`

| Name | Value |
|---|---:|
| `enum_SENSOR_STATUS_TYPE_SAVED` | 0 |
| `enum_SENSOR_STATUS_TYPE_CONNECTED` | 1 |
| `enum_SENSOR_STATUS_TYPE_NO_SAVED` | 2 |

### `SENSOR_TYPE`

| Name | Value |
|---|---:|
| `enum_SENSOR_TYPE_INVALID` | 0 |
| `enum_SENSOR_TYPE_HRM` | 1 |
| `enum_SENSOR_TYPE_CBSC` | 2 |
| `enum_SENSOR_TYPE_PEDAL_BPWR` | 3 |
| `enum_SENSOR_TYPE_OTHER_BPWR` | 4 |
| `enum_SENSOR_TYPE_CAD` | 5 |
| `enum_SENSOR_TYPE_SPD` | 6 |
| `enum_SENSOR_TYPE_SHFT` | 7 |
| `enum_SENSOR_TYPE_DI2` | 8 |
| `enum_SENSOR_TYPE_FEC` | 9 |
| `enum_SENSOR_TYPE_LEV` | 10 |
| `enum_SENSOR_TYPE_RD` | 11 |
| `enum_SENSOR_TYPE_RADAR` | 12 |
| `enum_SENSOR_TYPE_LIGHT` | 13 |
| `enum_SENSOR_TYPE_CORE_TEMP` | 14 |
| `enum_SENSOR_TYPE_CAMERA` | 15 |

## Messages

### `di2_sensor_set_message`

| # | Field | Type |
|---:|---|---|
| 1 | `num` | `DI2_CHN_NUM` |
| 2 | `button_op_type` | `DI2_BUTTON_OP_TYPE` |
| 3 | `func` | `DI2_FUNC_TYPE` |

### `radar_sensor_set_message`

| # | Field | Type |
|---:|---|---|
| 1 | `alert_bar_side` | `int32` |
| 2 | `alert_sound_open` | `int32` |

### `sensor_data_message`

| # | Field | Type |
|---:|---|---|
| 1 | `sensor_type` | `SENSOR_TYPE` |
| 2 | `sensor_radio_type` | `SENSOR_RADIO_TYPE` |
| 3 | `sensor_status_type` | `SENSOR_STATUS_TYPE` |
| 4 | `sensor_key` | `string` |
| 5 | `sensor_ble_name` | `string` |
| 6 | `sensor_rssi` | `int32` |
| 7 | `sensor_pwr` | `int32` |
| 8 | `wheel_size` | `int32` |
| 9 | `crank_length` | `int32` |
| 10 | `sensor_forbidden` | `int32` |
| 11 | `auto_wheel_size` | `int32` |
| 12 | `radar_sensor_set_msg` | `radar_sensor_set_message` |
| 13 | `di2_sensor_set_msg` | `repeated di2_sensor_set_message` |

### `sensor_message`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `sensor_operate_type` | `SENSOR_OPERATE_TYPE` |
| 3 | `sensor_data_msg` | `repeated sensor_data_message` |
