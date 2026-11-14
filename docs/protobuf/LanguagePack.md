# LanguagePack

_Generated from `com.igpsport.blelib.bean.LanguagePack`._

## Enums

### `LANGUAGE_PACK_FONT_TYPE`

| Name | Value |
|---|---:|
| `enum_LANGUAGE_PACK_FONT_TYPE_INVALID` | 0 |
| `enum_DianDian` | 1 |

### `LANGUAGE_PACK_SUB_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_LANG_SUB_OPERATE_TYPE_INVALID` | 0 |
| `enum_LANG_SUB_OPERATE_TYPE_SET_DOWNLOAD_URL` | 1 |
| `enum_LANG_SUB_OPERATE_TYPE_GET_MODULE_INFO` | 16 |
| `enum_LANG_SUB_OPERATE_TYPE_GET_LIST` | 17 |
| `enum_LANG_SUB_OPERATE_TYPE_ADD_PACK` | 32 |
| `enum_LANG_SUB_OPERATE_TYPE_DEL_PACK` | 48 |

## Messages

### `language_pack_info_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `language_type` | `Config.LANGUAGE_TYPE` |
| 2 | `language_versin` | `int32` |
| 3 | `font_type` | `LANGUAGE_PACK_FONT_TYPE` |
| 4 | `font_size` | `int32` |
| 5 | `pack_size` | `int32` |
| 6 | `url` | `string` |
| 7 | `md5_code` | `bytes` |

### `language_pack_module_info_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `support_cmd` | `repeated LANGUAGE_PACK_SUB_OPERATE_TYPE` |
| 2 | `language_pack_module_version` | `int32` |

### `language_pack_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `operate_type` | `Common.SERVICE_OPERATE_TYPE` |
| 3 | `language_sub_operate` | `LANGUAGE_PACK_SUB_OPERATE_TYPE` |
| 4 | `language_pack_info_message` | `repeated language_pack_info_msg` |
| 5 | `content` | `bytes` |
| 16 | `module_info_message` | `language_pack_module_info_msg` |
