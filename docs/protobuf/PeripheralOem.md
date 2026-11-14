# PeripheralOem

_Generated from `com.igpsport.blelib.bean.PeripheralOem`._

## Enums

### `PERIPHERAL_OEM_OPERATE`

| Name | Value |
|---|---:|
| `POO_UNSPECIFIED` | 0 |
| `UNRECOGNIZED` | -1 |

### `PERIPHERAL_OEM_SERVICE`

| Name | Value |
|---|---:|
| `POS_UNSPECIFIED` | 0 |
| `POS_BLE_INFO` | 1 |
| `POS_DEVICE_INFO` | 2 |
| `POS_MANUFACTURER_INFO` | 3 |
| `POS_ALL_OEM` | 11 |
| `UNRECOGNIZED` | -1 |

## Messages

### `peripheral_oem_data_ble`

| # | Field | Type |
|---:|---|---|
| 1 | `name` | `string` |

### `peripheral_oem_data_device`

| # | Field | Type |
|---:|---|---|
| 1 | `model` | `PeripheralInfo.PERIPHERAL_INFO_MODEL_TYPE` |
| 2 | `name` | `string` |
| 3 | `id` | `int32` |

### `peripheral_oem_data_manufacturer`

| # | Field | Type |
|---:|---|---|
| 1 | `name` | `string` |
| 2 | `id` | `int32` |

### `peripheral_oem_format`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `PeripheralCommon.PERIPHERAL_SERVICE_TYPE` |
| 2 | `operate_type` | `PeripheralCommon.PERIPHERAL_OPERATE_TYPE` |
| 3 | `sub_service_type` | `PERIPHERAL_OEM_SERVICE` |
| 4 | `sub_operate_type` | `PERIPHERAL_OEM_OPERATE` |
| 5 | `message` | `peripheral_oem_message` |

### `peripheral_oem_message`

| # | Field | Type |
|---:|---|---|
| 1 | `ble_info` | `peripheral_oem_data_ble` |
| 2 | `device_info` | `peripheral_oem_data_device` |
| 3 | `manufacturer_info` | `peripheral_oem_data_manufacturer` |
