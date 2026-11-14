# PeripheralRadar

_Generated from `com.igpsport.blelib.bean.PeripheralRadar`._

## Enums

### `PERIPHERAL_RADAR_OPERATE`

| Name | Value |
|---|---:|
| `PHO_NSPECIFIED_RADAR` | 0 |
| `PHO_RADAR_TARGET` | 1 |
| `PHO_ACC_SAMPLE` | 2 |
| `UNRECOGNIZED` | -1 |

### `PERIPHERAL_RADAR_SERVICE`

| Name | Value |
|---|---:|
| `PHS_NSPECIFIED_RADAR` | 0 |
| `UNRECOGNIZED` | -1 |

## Messages

### `peripheral_acc_message`

| # | Field | Type |
|---:|---|---|
| 6 | `x` | `int32` |
| 7 | `y` | `int32` |
| 8 | `z` | `int32` |

### `peripheral_radar_format`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `PeripheralCommon.PERIPHERAL_SERVICE_TYPE` |
| 2 | `operate_type` | `PeripheralCommon.PERIPHERAL_OPERATE_TYPE` |
| 3 | `sub_service_type` | `PERIPHERAL_RADAR_SERVICE` |
| 4 | `sub_operate_type` | `PERIPHERAL_RADAR_OPERATE` |
| 5 | `meaagse` | `peripheral_radar_message` |

### `peripheral_radar_message`

| # | Field | Type |
|---:|---|---|
| 6 | `radar_info` | `repeated peripheral_radar_target_message` |
| 7 | `acc_info` | `repeated peripheral_acc_message` |

### `peripheral_radar_target_message`

| # | Field | Type |
|---:|---|---|
| 6 | `level` | `int32` |
| 7 | `range` | `int32` |
| 8 | `speed` | `int32` |
