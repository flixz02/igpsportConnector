# AppWatch

_Generated from `com.igpsport.blelib.bean.AppWatch`._

## Enums

### `WATCH_CONTROL_SERVICE`

| Name | Value |
|---|---:|
| `WCS_UNSPECIFIED` | 0 |
| `WCS_DEVICE_INFO` | 1 |
| `WCS_ADV_CONTROL` | 2 |

## Messages

### `watch_control_format`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `sub_service_type` | `WATCH_CONTROL_SERVICE` |
| 3 | `operate_type` | `Common.SERVICE_OPERATE_TYPE` |
| 4 | `device_info` | `watch_device_info_message` |

### `watch_device_info_message`

| # | Field | Type |
|---:|---|---|
| 1 | `device_name` | `string` |
| 2 | `manufa_infor` | `string` |
