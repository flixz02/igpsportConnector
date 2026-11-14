# PeripheralHr

_Generated from `com.igpsport.blelib.bean.PeripheralHr`._

## Enums

### `PERIPHERAL_HR_OPERATE`

| Name | Value |
|---|---:|
| `PHO_NSPECIFIED` | 0 |
| `UNRECOGNIZED` | -1 |

### `PERIPHERAL_HR_SERVICE`

| Name | Value |
|---|---:|
| `PHS_NSPECIFIED` | 0 |
| `PHS_HRM_WARN` | 6 |
| `PHS_HRM_ZONE` | 11 |
| `PHS_SPORT_MODE` | 16 |
| `UNRECOGNIZED` | -1 |

### `PERIPHERAL_HR_SPORT_MODE_TYPE`

| Name | Value |
|---|---:|
| `PHSM_UNSPECIFIED` | 0 |
| `PHSM_DEFAULT` | 1 |
| `PHSM_RUN` | 2 |
| `PHSM_BIKE` | 3 |
| `PHSM_FITNESS` | 4 |
| `UNRECOGNIZED` | -1 |

### `PERIPHERAL_HR_WARN_TYPE`

| Name | Value |
|---|---:|
| `PHW_UNSPECIFIED` | 0 |
| `PHW_CUSTOM` | 1 |
| `PHW_ZONE` | 2 |
| `UNRECOGNIZED` | -1 |

## Messages

### `peripheral_hr_data_sport_mode`

| # | Field | Type |
|---:|---|---|
| 1 | `key_switch` | `bool` |
| 2 | `mode` | `PERIPHERAL_HR_SPORT_MODE_TYPE` |

### `peripheral_hr_data_warn`

| # | Field | Type |
|---:|---|---|
| 1 | `warn_switch` | `bool` |
| 2 | `high_switch` | `bool` |
| 3 | `low_switch` | `bool` |
| 4 | `high_warn_type` | `PERIPHERAL_HR_WARN_TYPE` |
| 5 | `low_warn_type` | `PERIPHERAL_HR_WARN_TYPE` |
| 6 | `high_value` | `int64` |
| 7 | `low_value` | `int64` |

### `peripheral_hr_data_zone`

| # | Field | Type |
|---:|---|---|
| 1 | `value` | `repeated Long` |

### `peripheral_hr_format`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `PeripheralCommon.PERIPHERAL_SERVICE_TYPE` |
| 2 | `operate_type` | `PeripheralCommon.PERIPHERAL_OPERATE_TYPE` |
| 3 | `sub_service_type` | `PERIPHERAL_HR_SERVICE` |
| 4 | `sub_operate_type` | `PERIPHERAL_HR_OPERATE` |
| 5 | `message` | `peripheral_hr_message` |

### `peripheral_hr_message`

| # | Field | Type |
|---:|---|---|
| 6 | `hrm_warn` | `peripheral_hr_data_warn` |
| 11 | `hr_zone` | `peripheral_hr_data_zone` |
| 16 | `sport_mode` | `peripheral_hr_data_sport_mode` |
