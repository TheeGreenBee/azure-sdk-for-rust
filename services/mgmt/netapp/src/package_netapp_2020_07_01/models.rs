#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationProperties {
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceSpecification {
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<Dimension>,
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "resourceIdDimensionNameOverride", default, skip_serializing_if = "Option::is_none")]
    pub resource_id_dimension_name_override: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dimension {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckAvailabilityResponse {
    #[serde(rename = "isAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_availability_response::Reason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
pub mod check_availability_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceNameAvailabilityRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: resource_name_availability_request::Type,
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
}
pub mod resource_name_availability_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.NetApp/netAppAccounts")]
        MicrosoftNetAppNetAppAccounts,
        #[serde(rename = "Microsoft.NetApp/netAppAccounts/capacityPools")]
        MicrosoftNetAppNetAppAccountsCapacityPools,
        #[serde(rename = "Microsoft.NetApp/netAppAccounts/capacityPools/volumes")]
        MicrosoftNetAppNetAppAccountsCapacityPoolsVolumes,
        #[serde(rename = "Microsoft.NetApp/netAppAccounts/capacityPools/volumes/snapshots")]
        MicrosoftNetAppNetAppAccountsCapacityPoolsVolumesSnapshots,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaAvailabilityRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: quota_availability_request::Type,
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
}
pub mod quota_availability_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.NetApp/netAppAccounts")]
        MicrosoftNetAppNetAppAccounts,
        #[serde(rename = "Microsoft.NetApp/netAppAccounts/capacityPools")]
        MicrosoftNetAppNetAppAccountsCapacityPools,
        #[serde(rename = "Microsoft.NetApp/netAppAccounts/capacityPools/volumes")]
        MicrosoftNetAppNetAppAccountsCapacityPoolsVolumes,
        #[serde(rename = "Microsoft.NetApp/netAppAccounts/capacityPools/volumes/snapshots")]
        MicrosoftNetAppNetAppAccountsCapacityPoolsVolumesSnapshots,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetAppAccountList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NetAppAccount>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetAppAccount {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccountProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetAppAccountPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccountProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "activeDirectories", default, skip_serializing_if = "Vec::is_empty")]
    pub active_directories: Vec<ActiveDirectory>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveDirectory {
    #[serde(rename = "activeDirectoryId", default, skip_serializing_if = "Option::is_none")]
    pub active_directory_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<active_directory::Status>,
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[serde(rename = "smbServerName", default, skip_serializing_if = "Option::is_none")]
    pub smb_server_name: Option<String>,
    #[serde(rename = "organizationalUnit", default, skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
    #[serde(rename = "backupOperators", default, skip_serializing_if = "Vec::is_empty")]
    pub backup_operators: Vec<String>,
    #[serde(rename = "kdcIP", default, skip_serializing_if = "Option::is_none")]
    pub kdc_ip: Option<String>,
    #[serde(rename = "adName", default, skip_serializing_if = "Option::is_none")]
    pub ad_name: Option<String>,
    #[serde(rename = "serverRootCACertificate", default, skip_serializing_if = "Option::is_none")]
    pub server_root_ca_certificate: Option<String>,
    #[serde(rename = "aesEncryption", default, skip_serializing_if = "Option::is_none")]
    pub aes_encryption: Option<bool>,
    #[serde(rename = "ldapSigning", default, skip_serializing_if = "Option::is_none")]
    pub ldap_signing: Option<bool>,
}
pub mod active_directory {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Created,
        Updating,
        InUse,
        Deleted,
        Error,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapacityPoolList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CapacityPool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapacityPool {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    pub properties: PoolProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolProperties {
    #[serde(rename = "poolId", default, skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
    pub size: i64,
    #[serde(rename = "serviceLevel")]
    pub service_level: pool_properties::ServiceLevel,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "totalThroughputMibps", default, skip_serializing_if = "Option::is_none")]
    pub total_throughput_mibps: Option<f64>,
    #[serde(rename = "utilizedThroughputMibps", default, skip_serializing_if = "Option::is_none")]
    pub utilized_throughput_mibps: Option<f64>,
    #[serde(rename = "qosType", default, skip_serializing_if = "Option::is_none")]
    pub qos_type: Option<pool_properties::QosType>,
}
pub mod pool_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ServiceLevel {
        Standard,
        Premium,
        Ultra,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum QosType {
        Auto,
        Manual,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapacityPoolPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PoolPatchProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolPatchProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "qosType", default, skip_serializing_if = "Option::is_none")]
    pub qos_type: Option<pool_patch_properties::QosType>,
}
pub mod pool_patch_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum QosType {
        Auto,
        Manual,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Volume>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    pub properties: VolumeProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceTags {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeProperties {
    #[serde(rename = "fileSystemId", default, skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "creationToken")]
    pub creation_token: String,
    #[serde(rename = "serviceLevel", default, skip_serializing_if = "Option::is_none")]
    pub service_level: Option<volume_properties::ServiceLevel>,
    #[serde(rename = "usageThreshold")]
    pub usage_threshold: i64,
    #[serde(rename = "exportPolicy", default, skip_serializing_if = "Option::is_none")]
    pub export_policy: Option<volume_properties::ExportPolicy>,
    #[serde(rename = "protocolTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub protocol_types: Vec<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "snapshotId", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "backupId", default, skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    #[serde(rename = "baremetalTenantId", default, skip_serializing_if = "Option::is_none")]
    pub baremetal_tenant_id: Option<String>,
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[serde(rename = "mountTargets", default, skip_serializing_if = "Vec::is_empty")]
    pub mount_targets: Vec<MountTargetProperties>,
    #[serde(rename = "volumeType", default, skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
    #[serde(rename = "dataProtection", default, skip_serializing_if = "Option::is_none")]
    pub data_protection: Option<volume_properties::DataProtection>,
    #[serde(rename = "isRestoring", default, skip_serializing_if = "Option::is_none")]
    pub is_restoring: Option<bool>,
    #[serde(rename = "snapshotDirectoryVisible", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_directory_visible: Option<bool>,
    #[serde(rename = "kerberosEnabled", default, skip_serializing_if = "Option::is_none")]
    pub kerberos_enabled: Option<bool>,
    #[serde(rename = "securityStyle", default, skip_serializing_if = "Option::is_none")]
    pub security_style: Option<volume_properties::SecurityStyle>,
    #[serde(rename = "throughputMibps", default, skip_serializing_if = "Option::is_none")]
    pub throughput_mibps: Option<f64>,
}
pub mod volume_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ServiceLevel {
        Standard,
        Premium,
        Ultra,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ExportPolicy {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub rules: Vec<ExportPolicyRule>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DataProtection {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub backup: Option<VolumeBackupProperties>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub replication: Option<ReplicationObject>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub snapshot: Option<VolumeSnapshotProperties>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SecurityStyle {
        #[serde(rename = "ntfs")]
        Ntfs,
        #[serde(rename = "unix")]
        Unix,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportPolicyRule {
    #[serde(rename = "ruleIndex", default, skip_serializing_if = "Option::is_none")]
    pub rule_index: Option<i32>,
    #[serde(rename = "unixReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub unix_read_only: Option<bool>,
    #[serde(rename = "unixReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub unix_read_write: Option<bool>,
    #[serde(rename = "kerberos5ReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub kerberos5_read_only: Option<bool>,
    #[serde(rename = "kerberos5ReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub kerberos5_read_write: Option<bool>,
    #[serde(rename = "kerberos5iReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub kerberos5i_read_only: Option<bool>,
    #[serde(rename = "kerberos5iReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub kerberos5i_read_write: Option<bool>,
    #[serde(rename = "kerberos5pReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub kerberos5p_read_only: Option<bool>,
    #[serde(rename = "kerberos5pReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub kerberos5p_read_write: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cifs: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nfsv3: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nfsv41: Option<bool>,
    #[serde(rename = "allowedClients", default, skip_serializing_if = "Option::is_none")]
    pub allowed_clients: Option<String>,
    #[serde(rename = "hasRootAccess", default, skip_serializing_if = "Option::is_none")]
    pub has_root_access: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeBackupProperties {
    #[serde(rename = "backupPolicyId", default, skip_serializing_if = "Option::is_none")]
    pub backup_policy_id: Option<String>,
    #[serde(rename = "policyEnforced", default, skip_serializing_if = "Option::is_none")]
    pub policy_enforced: Option<bool>,
    #[serde(rename = "vaultId", default, skip_serializing_if = "Option::is_none")]
    pub vault_id: Option<String>,
    #[serde(rename = "backupEnabled", default, skip_serializing_if = "Option::is_none")]
    pub backup_enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicationObject {
    #[serde(rename = "replicationId", default, skip_serializing_if = "Option::is_none")]
    pub replication_id: Option<String>,
    #[serde(rename = "endpointType", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<replication_object::EndpointType>,
    #[serde(rename = "replicationSchedule")]
    pub replication_schedule: replication_object::ReplicationSchedule,
    #[serde(rename = "remoteVolumeResourceId")]
    pub remote_volume_resource_id: String,
    #[serde(rename = "remoteVolumeRegion", default, skip_serializing_if = "Option::is_none")]
    pub remote_volume_region: Option<String>,
}
pub mod replication_object {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EndpointType {
        #[serde(rename = "src")]
        Src,
        #[serde(rename = "dst")]
        Dst,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReplicationSchedule {
        #[serde(rename = "_10minutely")]
        N10minutely,
        #[serde(rename = "hourly")]
        Hourly,
        #[serde(rename = "daily")]
        Daily,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeSnapshotProperties {
    #[serde(rename = "snapshotPolicyId", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_policy_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub healthy: Option<bool>,
    #[serde(rename = "relationshipStatus", default, skip_serializing_if = "Option::is_none")]
    pub relationship_status: Option<replication_status::RelationshipStatus>,
    #[serde(rename = "mirrorState", default, skip_serializing_if = "Option::is_none")]
    pub mirror_state: Option<replication_status::MirrorState>,
    #[serde(rename = "totalProgress", default, skip_serializing_if = "Option::is_none")]
    pub total_progress: Option<String>,
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
pub mod replication_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RelationshipStatus {
        Idle,
        Transferring,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MirrorState {
        Uninitialized,
        Mirrored,
        Broken,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumePatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VolumePatchProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumePatchProperties {
    #[serde(rename = "serviceLevel", default, skip_serializing_if = "Option::is_none")]
    pub service_level: Option<volume_patch_properties::ServiceLevel>,
    #[serde(rename = "usageThreshold", default, skip_serializing_if = "Option::is_none")]
    pub usage_threshold: Option<i64>,
    #[serde(rename = "exportPolicy", default, skip_serializing_if = "Option::is_none")]
    pub export_policy: Option<volume_patch_properties::ExportPolicy>,
    #[serde(rename = "throughputMibps", default, skip_serializing_if = "Option::is_none")]
    pub throughput_mibps: Option<f64>,
    #[serde(rename = "dataProtection", default, skip_serializing_if = "Option::is_none")]
    pub data_protection: Option<volume_patch_properties::DataProtection>,
}
pub mod volume_patch_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ServiceLevel {
        Standard,
        Premium,
        Ultra,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ExportPolicy {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub rules: Vec<ExportPolicyRule>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DataProtection {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub backup: Option<VolumeBackupProperties>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MountTarget {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    pub properties: MountTargetProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MountTargetProperties {
    #[serde(rename = "mountTargetId", default, skip_serializing_if = "Option::is_none")]
    pub mount_target_id: Option<String>,
    #[serde(rename = "fileSystemId")]
    pub file_system_id: String,
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "smbServerFqdn", default, skip_serializing_if = "Option::is_none")]
    pub smb_server_fqdn: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Snapshot>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SnapshotProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotPatch {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotProperties {
    #[serde(rename = "snapshotId", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotPolicyProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "hourlySchedule", default, skip_serializing_if = "Option::is_none")]
    pub hourly_schedule: Option<HourlySchedule>,
    #[serde(rename = "dailySchedule", default, skip_serializing_if = "Option::is_none")]
    pub daily_schedule: Option<DailySchedule>,
    #[serde(rename = "weeklySchedule", default, skip_serializing_if = "Option::is_none")]
    pub weekly_schedule: Option<WeeklySchedule>,
    #[serde(rename = "monthlySchedule", default, skip_serializing_if = "Option::is_none")]
    pub monthly_schedule: Option<MonthlySchedule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotPolicy {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    pub properties: SnapshotPolicyProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotPoliciesList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SnapshotPolicy>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotPolicyDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SnapshotPolicyProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotPolicyPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SnapshotPolicyProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotPolicyVolumeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HourlySchedule {
    #[serde(rename = "snapshotsToKeep", default, skip_serializing_if = "Option::is_none")]
    pub snapshots_to_keep: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minute: Option<i32>,
    #[serde(rename = "usedBytes", default, skip_serializing_if = "Option::is_none")]
    pub used_bytes: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DailySchedule {
    #[serde(rename = "snapshotsToKeep", default, skip_serializing_if = "Option::is_none")]
    pub snapshots_to_keep: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hour: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minute: Option<i32>,
    #[serde(rename = "usedBytes", default, skip_serializing_if = "Option::is_none")]
    pub used_bytes: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WeeklySchedule {
    #[serde(rename = "snapshotsToKeep", default, skip_serializing_if = "Option::is_none")]
    pub snapshots_to_keep: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hour: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minute: Option<i32>,
    #[serde(rename = "usedBytes", default, skip_serializing_if = "Option::is_none")]
    pub used_bytes: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonthlySchedule {
    #[serde(rename = "snapshotsToKeep", default, skip_serializing_if = "Option::is_none")]
    pub snapshots_to_keep: Option<i32>,
    #[serde(rename = "daysOfMonth", default, skip_serializing_if = "Option::is_none")]
    pub days_of_month: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hour: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minute: Option<i32>,
    #[serde(rename = "usedBytes", default, skip_serializing_if = "Option::is_none")]
    pub used_bytes: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeRevert {
    #[serde(rename = "snapshotId", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizeRequest {
    #[serde(rename = "remoteVolumeResourceId", default, skip_serializing_if = "Option::is_none")]
    pub remote_volume_resource_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BreakReplicationRequest {
    #[serde(rename = "forceBreakReplication", default, skip_serializing_if = "Option::is_none")]
    pub force_break_replication: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolChangeRequest {
    #[serde(rename = "newPoolResourceId")]
    pub new_pool_resource_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Backup>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Backup {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub properties: BackupProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackupProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupProperties {
    #[serde(rename = "backupId", default, skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    #[serde(rename = "creationDate", default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "backupType", default, skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<backup_properties::BackupType>,
}
pub mod backup_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BackupType {
        Manual,
        Scheduled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupPolicyProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "dailyBackupsToKeep", default, skip_serializing_if = "Option::is_none")]
    pub daily_backups_to_keep: Option<i32>,
    #[serde(rename = "weeklyBackupsToKeep", default, skip_serializing_if = "Option::is_none")]
    pub weekly_backups_to_keep: Option<i32>,
    #[serde(rename = "monthlyBackupsToKeep", default, skip_serializing_if = "Option::is_none")]
    pub monthly_backups_to_keep: Option<i32>,
    #[serde(rename = "yearlyBackupsToKeep", default, skip_serializing_if = "Option::is_none")]
    pub yearly_backups_to_keep: Option<i32>,
    #[serde(rename = "volumesAssigned", default, skip_serializing_if = "Option::is_none")]
    pub volumes_assigned: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "volumeBackups", default, skip_serializing_if = "Vec::is_empty")]
    pub volume_backups: Vec<VolumeBackups>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupPolicy {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    pub properties: BackupPolicyProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupPoliciesList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BackupPolicy>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupPolicyDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackupPolicyProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupPolicyPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ResourceTags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackupPolicyProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeBackups {
    #[serde(rename = "volumeName", default, skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<String>,
    #[serde(rename = "backupsCount", default, skip_serializing_if = "Option::is_none")]
    pub backups_count: Option<i32>,
    #[serde(rename = "policyEnabled", default, skip_serializing_if = "Option::is_none")]
    pub policy_enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Vault>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vault {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub properties: VaultProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultProperties {
    #[serde(rename = "vaultName", default, skip_serializing_if = "Option::is_none")]
    pub vault_name: Option<String>,
}
