#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileOperationResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boolean: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AclStatus {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    #[serde(rename = "stickyBit", default, skip_serializing_if = "Option::is_none")]
    pub sticky_bit: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AclStatusResult {
    #[serde(rename = "aclStatus", default, skip_serializing_if = "Option::is_none")]
    pub acl_status: Option<AclStatus>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentSummary {
    #[serde(rename = "directoryCount", default, skip_serializing_if = "Option::is_none")]
    pub directory_count: Option<i64>,
    #[serde(rename = "fileCount", default, skip_serializing_if = "Option::is_none")]
    pub file_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    #[serde(rename = "spaceConsumed", default, skip_serializing_if = "Option::is_none")]
    pub space_consumed: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentSummaryResult {
    #[serde(rename = "contentSummary", default, skip_serializing_if = "Option::is_none")]
    pub content_summary: Option<ContentSummary>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileStatusProperties {
    #[serde(rename = "accessTime", default, skip_serializing_if = "Option::is_none")]
    pub access_time: Option<i64>,
    #[serde(rename = "blockSize", default, skip_serializing_if = "Option::is_none")]
    pub block_size: Option<i64>,
    #[serde(rename = "msExpirationTime", default, skip_serializing_if = "Option::is_none")]
    pub ms_expiration_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    #[serde(rename = "modificationTime", default, skip_serializing_if = "Option::is_none")]
    pub modification_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "pathSuffix", default, skip_serializing_if = "Option::is_none")]
    pub path_suffix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<file_status_properties::Type>,
    #[serde(rename = "aclBit", default, skip_serializing_if = "Option::is_none")]
    pub acl_bit: Option<bool>,
}
pub mod file_status_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "FILE")]
        File,
        #[serde(rename = "DIRECTORY")]
        Directory,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileStatuses {
    #[serde(rename = "fileStatus", default, skip_serializing_if = "Vec::is_empty")]
    pub file_status: Vec<FileStatusProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileStatusesResult {
    #[serde(rename = "fileStatuses", default, skip_serializing_if = "Option::is_none")]
    pub file_statuses: Option<FileStatuses>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileStatusResult {
    #[serde(rename = "fileStatus", default, skip_serializing_if = "Option::is_none")]
    pub file_status: Option<FileStatusProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsIllegalArgumentException {
    #[serde(flatten)]
    pub adls_remote_exception: AdlsRemoteException,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsUnsupportedOperationException {
    #[serde(flatten)]
    pub adls_remote_exception: AdlsRemoteException,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsSecurityException {
    #[serde(flatten)]
    pub adls_remote_exception: AdlsRemoteException,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsIoException {
    #[serde(flatten)]
    pub adls_remote_exception: AdlsRemoteException,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsFileNotFoundException {
    #[serde(flatten)]
    pub adls_remote_exception: AdlsRemoteException,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsFileAlreadyExistsException {
    #[serde(flatten)]
    pub adls_remote_exception: AdlsRemoteException,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsBadOffsetException {
    #[serde(flatten)]
    pub adls_remote_exception: AdlsRemoteException,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsRuntimeException {
    #[serde(flatten)]
    pub adls_remote_exception: AdlsRemoteException,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsAccessControlException {
    #[serde(flatten)]
    pub adls_remote_exception: AdlsRemoteException,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsThrottledException {
    #[serde(flatten)]
    pub adls_remote_exception: AdlsRemoteException,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsRemoteException {
    pub exception: String,
    #[serde(rename = "javaClassName", default, skip_serializing_if = "Option::is_none")]
    pub java_class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdlsError {
    #[serde(rename = "remoteException", default, skip_serializing_if = "Option::is_none")]
    pub remote_exception: Option<AdlsRemoteException>,
}
