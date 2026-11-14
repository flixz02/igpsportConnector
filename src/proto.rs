//! Minimal protobuf messages used by the BSC200 protocol.
//!
//! These are hand-written prost definitions for only the services/operations
//! this application needs. Field numbers and types come from the generated
//! `com.igpsport.blelib.bean` classes in the decompiled app; see
//! `docs/protobuf/` for the full schema.

use prost::Message;

pub mod service {
    pub const INS: u8 = 1;
    pub const MAP: u8 = 2;
    pub const BACK: u8 = 3;
    pub const FIRMWARE: u8 = 4;
    pub const WIFI: u8 = 5;
    pub const CYCLING_DATA: u8 = 6;
    pub const ROUTE_PLAN: u8 = 7;
    pub const REAL_TIME_TRACE: u8 = 8;
    pub const USER_CONFIG: u8 = 9;
    pub const BLE: u8 = 10;
    pub const FACTORY: u8 = 11;
    pub const CONFIG: u8 = 12;
    pub const DEV_STATUS: u8 = 13;
    pub const SENSOR: u8 = 14;
    pub const TRAINING: u8 = 15;
    pub const TEAM_INFO: u8 = 16;
    pub const DEV_VER_INFO: u8 = 17;
    pub const LANGUAGE: u8 = 18;
    pub const LOG: u8 = 19;
    pub const THEME: u8 = 20;
    pub const FILE_OPERATION: u8 = 21;
    pub const MAP_NEW: u8 = 22;
    pub const ROUTE_BOOK: u8 = 23;
    pub const STAGE: u8 = 24;
    pub const TRAINING_PLAN: u8 = 25;
    pub const ALG_GOMORE: u8 = 26;
    pub const GROUP_RIDE: u8 = 27;
    pub const CLOUD_SWITCH_DEV: u8 = 28;
    pub const MUSIC_CTL: u8 = 29;
    pub const WATCH_CONTROL: u8 = 30;
}

pub mod op {
    // Common.SERVICE_OPERATE_TYPE
    pub const NONE: u8 = 0;
    pub const SET: u8 = 1;
    pub const GET: u8 = 2;
    pub const ADD: u8 = 3;
    pub const DEL: u8 = 4;

    // DevVerInfo.OPERATE_TYPE
    pub const DEV_VER_SET: u8 = 1;
    pub const DEV_VER_GET: u8 = 2;
    pub const DEV_VER_SEND: u8 = 3;

    // CyclingData.CYCLING_DATA_OPERATE_TYPE
    pub const CYCLING_LIST_GET: u8 = 1;
    pub const CYCLING_LIST_SEND: u8 = 2;
    pub const CYCLING_FILE_GET: u8 = 3;
    pub const CYCLING_FILE_SEND: u8 = 4;
    pub const CYCLING_FILE_DEL: u8 = 5;
    pub const CYCLING_ALL_DEL: u8 = 6;
    pub const CYCLING_AUTO_UPLOAD: u8 = 7;
    pub const CYCLING_LIST_NUM_GET: u8 = 8;

    // RoutePlan.ROUTE_PLAN_OPERATE_TYPE
    pub const ROUTE_LIST_GET: u8 = 1;
    pub const ROUTE_LIST_SEND: u8 = 2;
    pub const ROUTE_FILE_DEL: u8 = 3;
    pub const ROUTE_FILE_SEND: u8 = 4;
    pub const ROUTE_FILE_USE: u8 = 5;
    pub const ROUTE_FILES_DEL: u8 = 6;
    pub const ROUTE_LIST_NUM_GET: u8 = 7;
    pub const ROUTE_RENAME: u8 = 8;
    pub const ROUTE_TURN_PROMPT: u8 = 9;
    pub const ROUTE_NOTIFY_TRAJECTORY_DEVIATION: u8 = 10;
    pub const ROUTE_POINT_GET: u8 = 11;
    pub const ROUTE_REROUTE_STATUS: u8 = 12;
    pub const ROUTE_REQUEST_WHETHER_SUPPORT_FUNCTION: u8 = 13;
    pub const ROUTE_SUPPORT_FUNCTION_ENABLE_STATUS: u8 = 14;

    // Ble.BLE_OPERATE_TYPE
    pub const BLE_BOND_INFO: u8 = 1;
    pub const BLE_BOND_REQ: u8 = 2;
    pub const BLE_CONNECT_STATUS: u8 = 3;
    pub const BLE_UNBOND: u8 = 4;
    pub const BLE_WATCH_BOND_USER_GET: u8 = 5;
    pub const BLE_WATCH_BOND_USER_SET: u8 = 7;
    pub const BLE_WATCH_BOND_STATE_UPDATE: u8 = 8;
    pub const BLE_BOND_STATUS_GET: u8 = 9;
}

pub mod file_type {
    pub const GENERAL: u8 = 0;
    pub const TRAINING: u8 = 1;
    pub const ROUTE_PLAN: u8 = 2;
    pub const MAP: u8 = 3;
    pub const THEME: u8 = 4;
    pub const FIRMWARE: u8 = 5;
    pub const LANGUAGE: u8 = 6;
    pub const AGPS: u8 = 7;
    pub const ROUTE_BOOK: u8 = 8;
    pub const STAGE: u8 = 9;
    pub const POWERON_IMG: u8 = 10;
    pub const SCHEDULE: u8 = 11;
    pub const THEME_BACKGROUND: u8 = 12;
    pub const GROUP_RIDE_AVATAR: u8 = 13;
    pub const CLOUD_SWITCH_DEV: u8 = 14;
    pub const GROUP_RIDE_ROUTE_BOOK: u8 = 15;
    pub const WATCH_DIAL: u8 = 16;
    pub const YAW_RENAVI_BOOK: u8 = 17;
    pub const WATCH_PHOTO_DIAL: u8 = 18;
}

pub mod route_file_type {
    pub const INVALID: u8 = 0;
    pub const CNX: u8 = 1;
    pub const GPX: u8 = 2;
    pub const FIT: u8 = 3;
    pub const TCX: u8 = 4;
    pub const XML: u8 = 5;
}

#[derive(Clone, PartialEq, Message)]
pub struct FileListGet {
    #[prost(int32, optional, tag = "1")]
    pub file_num: Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub file_list_support_num_max: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub file_index_start: Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub file_index_end: Option<i32>,
}

#[derive(Clone, PartialEq, Message)]
pub struct VersionMsg {
    #[prost(int32, optional, tag = "1")]
    pub main_boot_ver: Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub main_app_ver: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub ble_boot_ver: Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub ble_app_ver: Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub hardware_ver: Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub protocol_ver: Option<i32>,
    #[prost(string, optional, tag = "7")]
    pub compile_time: Option<String>,
}

impl VersionMsg {
    pub fn display(&self) -> String {
        let mut parts = Vec::new();
        if let Some(v) = self.main_app_ver {
            parts.push(format!("main {v}"));
        }
        if let Some(v) = self.ble_app_ver {
            parts.push(format!("ble {v}"));
        }
        if let Some(v) = self.protocol_ver {
            parts.push(format!("proto {v}"));
        }
        if parts.is_empty() {
            "unknown version".to_string()
        } else {
            parts.join(", ")
        }
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct DevVerInfoMsg {
    #[prost(int32, optional, tag = "1")]
    pub service_type: Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub operate_type: Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub version_message: Option<VersionMsg>,
}

impl DevVerInfoMsg {
    pub fn get() -> Self {
        Self {
            service_type: Some(service::DEV_VER_INFO as i32),
            operate_type: Some(op::DEV_VER_GET as i32),
            version_message: None,
        }
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct CyclingDataFileFlag {
    #[prost(int32, optional, tag = "1")]
    pub timestamp: Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub file_size: Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub user_id: Option<String>,
    #[prost(string, optional, tag = "4")]
    pub device_id: Option<String>,
}

impl CyclingDataFileFlag {
    pub fn for_id(timestamp: i32) -> Self {
        Self {
            timestamp: Some(timestamp),
            ..Default::default()
        }
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct CyclingDataAutoUpload {
    #[prost(int32, optional, tag = "1")]
    pub status: Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub cycling_data_url: Option<String>,
    #[prost(string, optional, tag = "3")]
    pub cycling_data_check_url: Option<String>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CyclingDataMsg {
    #[prost(int32, optional, tag = "1")]
    pub service_type: Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub operate_type: Option<i32>,
    #[prost(message, repeated, tag = "3")]
    pub cycling_data_file_flag_msg: Vec<CyclingDataFileFlag>,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub file_content: Option<Vec<u8>>,
    #[prost(message, optional, tag = "5")]
    pub cycling_data_auto_upload_msg: Option<CyclingDataAutoUpload>,
    #[prost(message, optional, tag = "6")]
    pub list_msg: Option<FileListGet>,
}

impl CyclingDataMsg {
    pub fn list_num_get() -> Self {
        Self {
            service_type: Some(service::CYCLING_DATA as i32),
            operate_type: Some(op::CYCLING_LIST_NUM_GET as i32),
            ..Default::default()
        }
    }

    pub fn list_get(start: i32, end: i32) -> Self {
        Self {
            service_type: Some(service::CYCLING_DATA as i32),
            operate_type: Some(op::CYCLING_LIST_GET as i32),
            list_msg: Some(FileListGet {
                file_index_start: Some(start),
                file_index_end: Some(end),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    pub fn file_get(timestamp: i32) -> Self {
        Self {
            service_type: Some(service::CYCLING_DATA as i32),
            operate_type: Some(op::CYCLING_FILE_GET as i32),
            cycling_data_file_flag_msg: vec![CyclingDataFileFlag::for_id(timestamp)],
            ..Default::default()
        }
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct FileExtraInfo {
    #[prost(int32, optional, tag = "1")]
    pub distance: Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub total_ascent: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub avg_slope: Option<i32>,
}

#[derive(Clone, PartialEq, Message)]
pub struct FileTransferMethodInformation {
    #[prost(bool, optional, tag = "1")]
    pub resume_supported: Option<bool>,
    #[prost(bool, optional, tag = "2")]
    pub packet_ack_supported: Option<bool>,
    #[prost(int32, optional, tag = "3")]
    pub resume_offset: Option<i32>,
}

#[derive(Clone, PartialEq, Message)]
pub struct GeneralFileOperation {
    #[prost(int32, optional, tag = "1")]
    pub service_type: Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub operate_type: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub file_type: Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub file_size: Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub file_id: Option<i32>,
    #[prost(string, optional, tag = "6")]
    pub file_name: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub file_extension: Option<String>,
    #[prost(string, optional, tag = "8")]
    pub file_md5: Option<String>,
    #[prost(int32, optional, tag = "9")]
    pub file_verify_val: Option<i32>,
    #[prost(bool, optional, tag = "10")]
    pub is_temp: Option<bool>,
    #[prost(message, optional, tag = "11")]
    pub extra_info: Option<FileExtraInfo>,
    #[prost(message, optional, tag = "15")]
    pub method_information: Option<FileTransferMethodInformation>,
}

impl GeneralFileOperation {
    pub fn route_plan(
        file_size: i32,
        file_id: i32,
        file_name: impl Into<String>,
        distance_m: Option<i32>,
        ascent_m: Option<i32>,
    ) -> Self {
        Self {
            service_type: Some(service::FILE_OPERATION as i32),
            operate_type: Some(op::ADD as i32),
            file_type: Some(file_type::ROUTE_PLAN as i32),
            file_size: Some(file_size),
            file_id: Some(file_id),
            file_name: Some(file_name.into()),
            file_extension: Some("cnx".to_string()),
            is_temp: Some(false),
            extra_info: Some(FileExtraInfo {
                distance: distance_m.map(|v| v.saturating_mul(100)),
                total_ascent: ascent_m.map(|v| v.saturating_mul(100)),
                avg_slope: None,
            }),
            ..Default::default()
        }
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct FileDownload {
    #[prost(int32, optional, tag = "1")]
    pub file_size: Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub file_type: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub file_id: Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub file_name: Option<String>,
    #[prost(int32, optional, tag = "7")]
    pub file_verify_val: Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub file_crc32: Option<i32>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BleDataMsg {
    #[prost(int32, optional, tag = "1")]
    pub status: Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub member_id: Option<String>,
}

#[derive(Clone, PartialEq, Message)]
pub struct WatchBleBondUserMsg {
    #[prost(string, optional, tag = "1")]
    pub user_id: Option<String>,
    #[prost(int32, optional, tag = "2")]
    pub state: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub bond_allow: Option<i32>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BleBondStatusMsg {
    #[prost(int32, optional, tag = "1")]
    pub bond_status: Option<i32>,
}

#[derive(Clone, PartialEq, Message)]
pub struct BleMsg {
    #[prost(int32, optional, tag = "1")]
    pub service_type: Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub ble_operate_type: Option<i32>,
    #[prost(message, repeated, tag = "3")]
    pub ble_data_msg: Vec<BleDataMsg>,
    #[prost(message, optional, tag = "4")]
    pub ble_bond_user_msg: Option<WatchBleBondUserMsg>,
    #[prost(message, optional, tag = "5")]
    pub ble_bond_status_msg: Option<BleBondStatusMsg>,
}

impl BleMsg {
    pub fn bond_status_get() -> Self {
        Self {
            service_type: Some(service::BLE as i32),
            ble_operate_type: Some(op::BLE_BOND_STATUS_GET as i32),
            ..Default::default()
        }
    }

    pub fn connect_status(member_id: impl Into<String>, status: i32) -> Self {
        Self {
            service_type: Some(service::BLE as i32),
            ble_operate_type: Some(op::BLE_CONNECT_STATUS as i32),
            ble_data_msg: vec![BleDataMsg {
                status: Some(status),
                member_id: Some(member_id.into()),
            }],
            ..Default::default()
        }
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct RoutePlanInfoMsg {
    #[prost(int32, optional, tag = "1")]
    pub id: Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub file_type: Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub name: Option<String>,
    #[prost(int32, optional, tag = "4")]
    pub total_distance: Option<i32>,
    #[prost(double, optional, tag = "5")]
    pub longitude_start: Option<f64>,
    #[prost(double, optional, tag = "6")]
    pub latitude_start: Option<f64>,
    #[prost(int32, optional, tag = "7")]
    pub status: Option<i32>,
}

#[derive(Clone, PartialEq, Message)]
pub struct RoutePointMsg {
    #[prost(double, optional, tag = "1")]
    pub longitude: Option<f64>,
    #[prost(double, optional, tag = "2")]
    pub latitude: Option<f64>,
}

#[derive(Clone, PartialEq, Message)]
pub struct ReroutePointInfoMsg {
    #[prost(message, optional, tag = "1")]
    pub reroute_start_point: Option<RoutePointMsg>,
    #[prost(message, optional, tag = "2")]
    pub reroute_end_point: Option<RoutePointMsg>,
    #[prost(message, repeated, tag = "3")]
    pub reroute_pass_point: Vec<RoutePointMsg>,
}

#[derive(Clone, PartialEq, Message)]
pub struct AppSupportFunctionStatus {
    #[prost(bool, optional, tag = "1")]
    pub is_support_reroute: Option<bool>,
}

#[derive(Clone, PartialEq, Message)]
pub struct RoutePlanDataMsg {
    #[prost(int32, optional, tag = "1")]
    pub service_type: Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub route_plan_operate_type: Option<i32>,
    #[prost(string, repeated, tag = "3")]
    pub line_id: Vec<String>,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub file_content: Option<Vec<u8>>,
    #[prost(message, repeated, tag = "5")]
    pub route_plan_info_msg: Vec<RoutePlanInfoMsg>,
    #[prost(message, optional, tag = "6")]
    pub reroute_point_msg: Option<ReroutePointInfoMsg>,
    #[prost(int32, optional, tag = "7")]
    pub reroute_status: Option<i32>,
    #[prost(message, optional, tag = "8")]
    pub support_function_status: Option<AppSupportFunctionStatus>,
    #[prost(message, optional, tag = "12")]
    pub route_list_get_msg: Option<FileListGet>,
}

impl RoutePlanDataMsg {
    pub fn list_get(start: i32, end: i32) -> Self {
        Self {
            service_type: Some(service::ROUTE_PLAN as i32),
            route_plan_operate_type: Some(op::ROUTE_LIST_GET as i32),
            route_list_get_msg: Some(FileListGet {
                file_index_start: Some(start),
                file_index_end: Some(end),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    pub fn list_num_get() -> Self {
        Self {
            service_type: Some(service::ROUTE_PLAN as i32),
            route_plan_operate_type: Some(op::ROUTE_LIST_NUM_GET as i32),
            ..Default::default()
        }
    }

    pub fn use_file(id: i32, name: impl Into<String>, distance_m: Option<i32>) -> Self {
        Self {
            service_type: Some(service::ROUTE_PLAN as i32),
            route_plan_operate_type: Some(op::ROUTE_FILE_USE as i32),
            line_id: vec![format!("{id}/cnx")],
            route_plan_info_msg: vec![RoutePlanInfoMsg {
                id: Some(id),
                file_type: Some(route_file_type::CNX as i32),
                name: Some(name.into()),
                total_distance: distance_m,
                ..Default::default()
            }],
            ..Default::default()
        }
    }

    pub fn delete_files(ids: &[i32]) -> Self {
        Self {
            service_type: Some(service::ROUTE_PLAN as i32),
            route_plan_operate_type: Some(op::ROUTE_FILES_DEL as i32),
            line_id: ids.iter().map(|id| format!("{id}/cnx")).collect(),
            route_plan_info_msg: ids
                .iter()
                .map(|id| RoutePlanInfoMsg {
                    id: Some(*id),
                    file_type: Some(route_file_type::CNX as i32),
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dev_ver_get_encoding() {
        let msg = DevVerInfoMsg::get();
        assert_eq!(msg.encode_to_vec(), vec![0x08, 0x11, 0x10, 0x02]);
    }

    #[test]
    fn cycling_file_get_encoding_roundtrip() {
        let msg = CyclingDataMsg::file_get(123456);
        let bytes = msg.encode_to_vec();
        let decoded = CyclingDataMsg::decode(bytes.as_slice()).unwrap();
        assert_eq!(decoded.cycling_data_file_flag_msg.len(), 1);
        assert_eq!(
            decoded.cycling_data_file_flag_msg[0].timestamp,
            Some(123456)
        );
        assert_eq!(decoded.operate_type, Some(op::CYCLING_FILE_GET as i32));
    }

    #[test]
    fn route_use_encoding() {
        let msg = RoutePlanDataMsg::use_file(12345678, "Test", Some(1000));
        let bytes = msg.encode_to_vec();
        let decoded = RoutePlanDataMsg::decode(bytes.as_slice()).unwrap();
        assert_eq!(decoded.line_id, vec!["12345678/cnx"]);
    }
}
