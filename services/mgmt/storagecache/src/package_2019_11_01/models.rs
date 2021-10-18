#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiOperation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<api_operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
pub mod api_operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiOperationListResult {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApiOperation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cache {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UrlString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<cache::Properties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<cache::Sku>,
}
pub mod cache {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "cacheSizeGB", default, skip_serializing_if = "Option::is_none")]
        pub cache_size_gb: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub health: Option<CacheHealth>,
        #[serde(rename = "mountAddresses", default, skip_serializing_if = "Vec::is_empty")]
        pub mount_addresses: Vec<String>,
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub subnet: Option<UrlString>,
        #[serde(rename = "upgradeStatus", default, skip_serializing_if = "Option::is_none")]
        pub upgrade_status: Option<CacheUpgradeStatus>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Succeeded,
            Failed,
            Cancelled,
            Creating,
            Deleting,
            Updating,
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Sku {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CachesListResult {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cache>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheHealth {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<cache_health::State>,
    #[serde(rename = "statusDescription", default, skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
}
pub mod cache_health {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Unknown,
        Healthy,
        Degraded,
        Down,
        Transitioning,
        Stopping,
        Stopped,
        Upgrading,
        Flushing,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheUpgradeStatus {
    #[serde(rename = "currentFirmwareVersion", default, skip_serializing_if = "Option::is_none")]
    pub current_firmware_version: Option<String>,
    #[serde(rename = "firmwareUpdateStatus", default, skip_serializing_if = "Option::is_none")]
    pub firmware_update_status: Option<cache_upgrade_status::FirmwareUpdateStatus>,
    #[serde(rename = "firmwareUpdateDeadline", default, skip_serializing_if = "Option::is_none")]
    pub firmware_update_deadline: Option<String>,
    #[serde(rename = "lastFirmwareUpdate", default, skip_serializing_if = "Option::is_none")]
    pub last_firmware_update: Option<String>,
    #[serde(rename = "pendingFirmwareVersion", default, skip_serializing_if = "Option::is_none")]
    pub pending_firmware_version: Option<String>,
}
pub mod cache_upgrade_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirmwareUpdateStatus {
        #[serde(rename = "available")]
        Available,
        #[serde(rename = "unavailable")]
        Unavailable,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnknownProperties {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Nfs3Target {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "usageModel", default, skip_serializing_if = "Option::is_none")]
    pub usage_model: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClfsTarget {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<UrlString>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnknownTarget {
    #[serde(rename = "unknownMap", default, skip_serializing_if = "Option::is_none")]
    pub unknown_map: Option<UnknownProperties>,
}
pub type ResourceName = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSku {
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<ResourceSkuCapabilities>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[serde(rename = "locationInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub location_info: Vec<ResourceSkuLocationInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<Restriction>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Restriction {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<restriction::ReasonCode>,
}
pub mod restriction {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReasonCode {
        QuotaId,
        NotAvailableForSubscription,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkuCapabilities {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkuLocationInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkusResult {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageTarget {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<storage_target::Properties>,
}
pub mod storage_target {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub junctions: Vec<NamespaceJunction>,
        #[serde(rename = "targetType", default, skip_serializing_if = "Option::is_none")]
        pub target_type: Option<properties::TargetType>,
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nfs3: Option<Nfs3Target>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub clfs: Option<ClfsTarget>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unknown: Option<UnknownTarget>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum TargetType {
            #[serde(rename = "nfs3")]
            Nfs3,
            #[serde(rename = "clfs")]
            Clfs,
            #[serde(rename = "unknown")]
            Unknown,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Succeeded,
            Failed,
            Cancelled,
            Creating,
            Deleting,
            Updating,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceJunction {
    #[serde(rename = "namespacePath", default, skip_serializing_if = "Option::is_none")]
    pub namespace_path: Option<String>,
    #[serde(rename = "targetPath", default, skip_serializing_if = "Option::is_none")]
    pub target_path: Option<String>,
    #[serde(rename = "nfsExport", default, skip_serializing_if = "Option::is_none")]
    pub nfs_export: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageTargetsResult {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageTarget>,
}
pub type UrlString = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageModel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<usage_model::Display>,
    #[serde(rename = "modelName", default, skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[serde(rename = "targetType", default, skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}
pub mod usage_model {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageModelsResult {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UsageModel>,
}
