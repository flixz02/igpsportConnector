# Ble

_Generated from `com.igpsport.blelib.bean.Ble`._

## Enums

### `BLE_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_BLE_OPERATE_TYPE_NONE` | 0 |
| `enum_BLE_OPERATE_TYPE_BOND_INFO` | 1 |
| `enum_BLE_OPERATE_TYPE_BOND_REQ` | 2 |
| `enum_BLE_OPERATE_TYPE_CONNECT_STATUS` | 3 |
| `enum_BLE_OPERATE_TYPE_UNBOND` | 4 |
| `enum_BLE_OPERATE_TYPE_WATCH_BOND_USER_GET` | 5 |
| `enum_BLE_OPERATE_TYPE_WATCH_BOND_USER_SET` | 7 |
| `enum_BLE_OPERATE_TYPE_WATCH_BOND_STATE_UPDATE` | 8 |
| `enum_BLE_OPERATE_TYPE_BOND_STATUS_GET` | 9 |

## Messages

### `ble_bond_status_message`

| # | Field | Type |
|---:|---|---|
| 1 | `bond_status` | `int32` |

### `ble_data_message`

| # | Field | Type |
|---:|---|---|
| 1 | `status` | `int32` |
| 2 | `member_id` | `string` |

### `ble_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `ble_operate_type` | `BLE_OPERATE_TYPE` |
| 3 | `ble_data_msg` | `repeated ble_data_message` |
| 4 | `ble_bond_user_msg` | `watch_ble_bond_user_message` |
| 5 | `ble_bond_status_msg` | `ble_bond_status_message` |

### `watch_ble_bond_user_message`

| # | Field | Type |
|---:|---|---|
| 1 | `user_id` | `string` |
| 2 | `state` | `int32` |
| 3 | `bond_allow` | `int32` |
