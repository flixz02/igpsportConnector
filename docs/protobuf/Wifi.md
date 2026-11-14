# Wifi

_Generated from `com.igpsport.blelib.bean.Wifi`._

## Enums

### `WIFI_ENCRYPT_TYPE`

| Name | Value |
|---|---:|
| `enum_WIFI_ENCRYPT_TYPE_NONE` | 0 |
| `enum_WIFI_ENCRYPT_TYPE_OPEN` | 1 |
| `enum_WIFI_ENCRYPT_TYPE_WEP_PSK` | 2 |
| `enum_WIFI_ENCRYPT_TYPE_WPA_TKIP_PSK` | 3 |
| `enum_WIFI_ENCRYPT_TYPE_WPA_TKIP_8021X` | 4 |
| `enum_WIFI_ENCRYPT_TYPE_WPA_AES_PSK` | 5 |
| `enum_WIFI_ENCRYPT_TYPE_WPA_AES_8021X` | 6 |
| `enum_WIFI_ENCRYPT_TYPE_WPA2_AES_PSK` | 7 |
| `enum_WIFI_ENCRYPT_TYPE_WPA2_AES_8021X` | 8 |
| `enum_WIFI_ENCRYPT_TYPE_WPA2_TKIP_PSK` | 9 |
| `enum_WIFI_ENCRYPT_TYPE_WPA2_TKIP_8021X` | 10 |
| `enum_WIFI_ENCRYPT_TYPE_WPA2_MIXED_PSK` | 11 |
| `enum_WIFI_ENCRYPT_TYPE_WPA_WPA2_MIXED_PSK` | 12 |
| `enum_WIFI_ENCRYPT_TYPE_WPA_WPA2_MIXED_8021X` | 13 |
| `enum_WIFI_ENCRYPT_TYPE_WPA3` | 14 |

### `WIFI_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_WIFI_OPERATE_TYPE_NONE` | 0 |
| `enum_WIFI_OPERATE_TYPE_STATUS_GET` | 1 |
| `enum_WIFI_OPERATE_TYPE_STATUS_SEND` | 2 |
| `enum_WIFI_OPERATE_TYPE_CTRL` | 3 |
| `enum_WIFI_OPERATE_TYPE_LIST_GET` | 4 |
| `enum_WIFI_OPERATE_TYPE_LIST_SEND` | 5 |
| `enum_WIFI_OPERATE_TYPE_ASSIGN_SSID` | 6 |
| `enum_WIFI_OPERATE_TYPE_AUTO_STATUS_GET` | 7 |
| `enum_WIFI_OPERATE_TYPE_AUTO_STATUS_SEND` | 8 |
| `enum_WIFI_OPERATE_TYPE_AUTO_STATUS_SET` | 9 |
| `enum_WIFI_OPERATE_TYPE_DELETE_SSID` | 10 |
| `enum_WIFI_OPERATE_TYPE_SYNC_STATES_GET` | 11 |
| `enum_WIFI_OPERATE_TYPE_STORED_LIST_GET` | 12 |
| `enum_WIFI_OPERATE_TYPE_URL_SEND` | 13 |
| `enum_WIFI_OPERATE_TYPE_CPU_UUID_GET` | 14 |

### `WIFI_SYNC_TYPE`

| Name | Value |
|---|---:|
| `enum_WIFI_SYNC_TYPE_NONE` | 0 |
| `enum_WIFI_SYNC_TYPE_ACTIVITY` | 1 |
| `enum_WIFI_SYNC_TYPE_AGPS` | 2 |
| `enum_WIFI_SYNC_TYPE_FIRMWARE` | 3 |

## Messages

### `wifi_data_message`

| # | Field | Type |
|---:|---|---|
| 1 | `status` | `int32` |
| 2 | `ssid` | `string` |
| 3 | `key` | `string` |
| 4 | `open_status` | `int32` |
| 5 | `signal_strength` | `int32` |
| 6 | `encryption_type` | `WIFI_ENCRYPT_TYPE` |
| 7 | `hidessid` | `int32` |

### `wifi_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `wifi_operate_type` | `WIFI_OPERATE_TYPE` |
| 3 | `wifi_data_msg` | `repeated wifi_data_message` |
| 4 | `wifi_sync_states` | `repeated wifi_sync_message` |
| 5 | `wifi_url_msg` | `wifi_url_message` |
| 6 | `cpu_uuid` | `string` |

### `wifi_sync_message`

| # | Field | Type |
|---:|---|---|
| 1 | `sync_type` | `WIFI_SYNC_TYPE` |

### `wifi_url_message`

| # | Field | Type |
|---:|---|---|
| 1 | `encryptedmemberid` | `string` |
| 2 | `baseurl` | `string` |
| 3 | `summarylisturl` | `string` |
| 4 | `memberid` | `int32` |
| 5 | `accept_language` | `string` |
| 6 | `timezone` | `string` |
