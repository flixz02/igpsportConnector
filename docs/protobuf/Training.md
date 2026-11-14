# Training

_Generated from `com.igpsport.blelib.bean.Training`._

## Enums

### `TRAINING_FILE_STATUS`

| Name | Value |
|---|---:|
| `enum_TRAINING_INVALID_STATUS` | 0 |
| `enum_TRAINING_USED_STATUS` | 1 |
| `enum_TRAINING_UNUSED_STATUS` | 2 |

### `TRAINING_FILE_TYPE`

| Name | Value |
|---|---:|
| `enum_TRAINING_FILE_TYPE_INVALIDE` | 0 |
| `enum_TRAINING_FILE_TYPE_JSON` | 1 |
| `enum_TRAINING_FILE_TYPE_XML` | 2 |
| `enum_TRAINING_FILE_TYPE_FIT` | 3 |
| `enum_TRAINING_FILE_TYPE_CSV` | 4 |
| `enum_TRAINING_FILE_TYPE_GPX` | 5 |
| `enum_TRAINING_FILE_TYPE_GZ` | 6 |
| `enum_TRAINING_FILE_TYPE_HRM` | 7 |
| `enum_TRAINING_FILE_TYPE_HST` | 8 |
| `enum_TRAINING_FILE_TYPE_PWPB` | 9 |
| `enum_TRAINING_FILE_TYPE_PWX` | 10 |
| `enum_TRAINING_FILE_TYPE_SDF` | 11 |
| `enum_TRAINING_FILE_TYPE_SRM` | 12 |
| `enum_TRAINING_FILE_TYPE_TCX` | 13 |
| `enum_TRAINING_FILE_TYPE_TXT` | 14 |
| `enum_TRAINING_FILE_TYPE_WKO` | 15 |
| `enum_TRAINING_FILE_TYPE_XLSX` | 16 |
| `enum_TRAINING_FILE_TYPE_CUSTOM` | 254 |

### `TRAINING_OBJECT`

| Name | Value |
|---|---:|
| `enum_TRAINING_OBJECT_INVALID` | 0 |
| `enum_TRAINING_OBJECT_TIME` | 1 |
| `enum_TRAINING_OBJECT_DISTANCE` | 2 |
| `enum_TRAINING_OBJECT_SPD` | 3 |
| `enum_TRAINING_OBJECT_CALORIE` | 4 |
| `enum_TRAINING_OBJECT_CLIMB` | 5 |
| `enum_TRAINING_OBJECT_PWR` | 6 |
| `enum_TRAINING_OBJECT_PWR_ZONE` | 7 |
| `enum_TRAINING_OBJECT_HRM` | 8 |
| `enum_TRAINING_OBJECT_HRM_ZONE` | 9 |
| `enum_TRAINING_OBJECT_CUSTOM` | 254 |

### `TRAINING_SUB_OPERATE_TYPE`

| Name | Value |
|---|---:|
| `enum_TRAINING_SUB_OPERATE_TYPE_INAVLIDE` | 0 |
| `enum_TRAINING_SUB_OPERATE_TYPE_SET_USE` | 1 |
| `enum_TRAINING_SUB_OPERATE_TYPE_SET_NAME` | 2 |
| `enum_TRAINING_SUB_OPERATE_TYPE_SET_REMIND` | 3 |
| `enum_TRAINING_SUB_OPERATE_TYPE_GET_MODULE_INFO` | 17 |
| `enum_TRAINING_SUB_OPERATE_TYPE_GET_LIST` | 18 |
| `enum_TRAINING_SUB_OPERATE_TYPE_GET_LIST_NUM` | 19 |
| `enum_TRAINING_SUB_OPERATE_TYPE_ADD_FILE` | 33 |
| `enum_TRAINING_SUB_OPERATE_TYPE_DEL_FILE` | 49 |

### `TRAINING_TYPE`

| Name | Value |
|---|---:|
| `enum_TRAINING_TYPE_INVALID` | 0 |
| `enum_TRAINING_TYPE_WORKOUT` | 1 |
| `enum_TRAINING_TYPE_SEGMENT` | 2 |
| `enum_TRAINING_TYPE_INDOOR` | 3 |
| `enum_TRAINING_TYPE_CUSTOM` | 254 |

## Messages

### `training_data_message`

| # | Field | Type |
|---:|---|---|
| 1 | `file_id` | `int32` |
| 2 | `file_name` | `string` |
| 3 | `file_type` | `TRAINING_FILE_TYPE` |
| 4 | `training_period` | `int32` |
| 5 | `training_type` | `TRAINING_TYPE` |
| 6 | `training_object_message` | `training_object_msg` |
| 7 | `training_time` | `int32` |
| 13 | `file_content` | `bytes` |

### `training_list_message`

| # | Field | Type |
|---:|---|---|
| 1 | `file_id` | `int32` |
| 2 | `file_type` | `TRAINING_FILE_TYPE` |
| 3 | `file_name` | `string` |
| 4 | `status` | `TRAINING_FILE_STATUS` |
| 5 | `training_period` | `int32` |

### `training_message`

| # | Field | Type |
|---:|---|---|
| 1 | `service_type` | `Common.service_type_index` |
| 2 | `operate_type` | `Common.SERVICE_OPERATE_TYPE` |
| 3 | `training_sub_operate_type` | `TRAINING_SUB_OPERATE_TYPE` |
| 4 | `training_data_msg` | `training_data_message` |
| 5 | `training_list_get_msg` | `Common.file_list_get_message` |
| 6 | `training_list_msg` | `repeated training_list_message` |
| 13 | `module_info_msg` | `training_module_info_message` |

### `training_module_info_message`

| # | Field | Type |
|---:|---|---|
| 1 | `sub_operate_type` | `repeated TRAINING_SUB_OPERATE_TYPE` |
| 2 | `training_module_version` | `int32` |

### `training_object_msg`

| # | Field | Type |
|---:|---|---|
| 1 | `trainning_obj` | `TRAINING_OBJECT` |
| 2 | `data_max` | `int32` |
| 3 | `data_min` | `int32` |
