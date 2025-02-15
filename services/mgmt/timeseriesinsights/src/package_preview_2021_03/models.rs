#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
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
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<LogSpecification>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub availabilities: Vec<MetricAvailability>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "resourceIdDimensionNameOverride", default, skip_serializing_if = "Option::is_none")]
    pub resource_id_dimension_name_override: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dimension {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricAvailability {
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "creationTime", default, skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Accepted,
    Creating,
    Updating,
    Succeeded,
    Failed,
    Deleting,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: sku::Name,
    pub capacity: i32,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        S1,
        S2,
        P1,
        L1,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WarmStoreConfigurationProperties {
    #[serde(rename = "dataRetention")]
    pub data_retention: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen2StorageConfigurationInput {
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[serde(rename = "managementKey")]
    pub management_key: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen2StorageConfigurationOutput {
    #[serde(rename = "accountName")]
    pub account_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen2StorageConfigurationMutableProperties {
    #[serde(rename = "managementKey")]
    pub management_key: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateTrackedResourceProperties {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentCreateOrUpdateParameters {
    #[serde(flatten)]
    pub create_or_update_tracked_resource_properties: CreateOrUpdateTrackedResourceProperties,
    pub kind: environment_create_or_update_parameters::Kind,
    pub sku: Sku,
}
pub mod environment_create_or_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Gen1,
        Gen2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen1EnvironmentCreateOrUpdateParameters {
    #[serde(flatten)]
    pub environment_create_or_update_parameters: EnvironmentCreateOrUpdateParameters,
    pub properties: Gen1EnvironmentCreationProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen2EnvironmentCreateOrUpdateParameters {
    #[serde(flatten)]
    pub environment_create_or_update_parameters: EnvironmentCreateOrUpdateParameters,
    pub properties: Gen2EnvironmentCreationProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentUpdateParameters {
    pub kind: environment_update_parameters::Kind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
pub mod environment_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Gen1,
        Gen2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen1EnvironmentUpdateParameters {
    #[serde(flatten)]
    pub environment_update_parameters: EnvironmentUpdateParameters,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Gen1EnvironmentMutableProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen2EnvironmentUpdateParameters {
    #[serde(flatten)]
    pub environment_update_parameters: EnvironmentUpdateParameters,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Gen2EnvironmentMutableProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentListResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EnvironmentResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub sku: Sku,
    pub kind: environment_resource::Kind,
}
pub mod environment_resource {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Gen1,
        Gen2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen1EnvironmentResource {
    #[serde(flatten)]
    pub environment_resource: EnvironmentResource,
    pub properties: Gen1EnvironmentResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen2EnvironmentResource {
    #[serde(flatten)]
    pub environment_resource: EnvironmentResource,
    pub properties: Gen2EnvironmentResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen1EnvironmentCreationProperties {
    #[serde(rename = "dataRetentionTime")]
    pub data_retention_time: String,
    #[serde(rename = "storageLimitExceededBehavior", default, skip_serializing_if = "Option::is_none")]
    pub storage_limit_exceeded_behavior: Option<gen1_environment_creation_properties::StorageLimitExceededBehavior>,
    #[serde(rename = "partitionKeyProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub partition_key_properties: Vec<TimeSeriesIdProperty>,
}
pub mod gen1_environment_creation_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StorageLimitExceededBehavior {
        PurgeOldData,
        PauseIngress,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen2EnvironmentCreationProperties {
    #[serde(rename = "timeSeriesIdProperties")]
    pub time_series_id_properties: Vec<TimeSeriesIdProperty>,
    #[serde(rename = "storageConfiguration")]
    pub storage_configuration: Gen2StorageConfigurationInput,
    #[serde(rename = "warmStoreConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub warm_store_configuration: Option<WarmStoreConfigurationProperties>,
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<gen2_environment_creation_properties::PublicNetworkAccess>,
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
}
pub mod gen2_environment_creation_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PublicNetworkAccess {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentResourceProperties {
    #[serde(flatten)]
    pub resource_properties: ResourceProperties,
    #[serde(rename = "dataAccessId", default, skip_serializing_if = "Option::is_none")]
    pub data_access_id: Option<String>,
    #[serde(rename = "dataAccessFqdn", default, skip_serializing_if = "Option::is_none")]
    pub data_access_fqdn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EnvironmentStatus>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen1EnvironmentResourceProperties {
    #[serde(flatten)]
    pub gen1_environment_creation_properties: Gen1EnvironmentCreationProperties,
    #[serde(flatten)]
    pub environment_resource_properties: EnvironmentResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen2EnvironmentResourceProperties {
    #[serde(flatten)]
    pub environment_resource_properties: EnvironmentResourceProperties,
    #[serde(rename = "timeSeriesIdProperties")]
    pub time_series_id_properties: Vec<TimeSeriesIdProperty>,
    #[serde(rename = "storageConfiguration")]
    pub storage_configuration: Gen2StorageConfigurationOutput,
    #[serde(rename = "warmStoreConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub warm_store_configuration: Option<WarmStoreConfigurationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen1EnvironmentMutableProperties {
    #[serde(rename = "dataRetentionTime", default, skip_serializing_if = "Option::is_none")]
    pub data_retention_time: Option<String>,
    #[serde(rename = "storageLimitExceededBehavior", default, skip_serializing_if = "Option::is_none")]
    pub storage_limit_exceeded_behavior: Option<gen1_environment_mutable_properties::StorageLimitExceededBehavior>,
}
pub mod gen1_environment_mutable_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StorageLimitExceededBehavior {
        PurgeOldData,
        PauseIngress,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gen2EnvironmentMutableProperties {
    #[serde(rename = "storageConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub storage_configuration: Option<Gen2StorageConfigurationMutableProperties>,
    #[serde(rename = "warmStoreConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub warm_store_configuration: Option<WarmStoreConfigurationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesIdProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<time_series_id_property::Type>,
}
pub mod time_series_id_property {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        String,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<IngressEnvironmentStatus>,
    #[serde(rename = "warmStorage", default, skip_serializing_if = "Option::is_none")]
    pub warm_storage: Option<WarmStorageEnvironmentStatus>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IngressEnvironmentStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ingress_environment_status::State>,
    #[serde(rename = "stateDetails", default, skip_serializing_if = "Option::is_none")]
    pub state_details: Option<EnvironmentStateDetails>,
}
pub mod ingress_environment_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Disabled,
        Ready,
        Running,
        Paused,
        Unknown,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentStateDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WarmStorageEnvironmentStatus {
    #[serde(rename = "propertiesUsage", default, skip_serializing_if = "Option::is_none")]
    pub properties_usage: Option<WarmStoragePropertiesUsage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WarmStoragePropertiesUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<warm_storage_properties_usage::State>,
    #[serde(rename = "stateDetails", default, skip_serializing_if = "Option::is_none")]
    pub state_details: Option<WarmStoragePropertiesUsageStateDetails>,
}
pub mod warm_storage_properties_usage {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Ok,
        Error,
        Unknown,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WarmStoragePropertiesUsageStateDetails {
    #[serde(rename = "currentCount", default, skip_serializing_if = "Option::is_none")]
    pub current_count: Option<i32>,
    #[serde(rename = "maxCount", default, skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IngressStartAtProperties {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ingress_start_at_properties::Type>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}
pub mod ingress_start_at_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        EarliestAvailable,
        EventSourceCreationTime,
        CustomEnqueuedTime,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSourceCreateOrUpdateParameters {
    #[serde(flatten)]
    pub create_or_update_tracked_resource_properties: CreateOrUpdateTrackedResourceProperties,
    pub kind: event_source_create_or_update_parameters::Kind,
    #[serde(rename = "localTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub local_timestamp: Option<LocalTimestamp>,
}
pub mod event_source_create_or_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "Microsoft.EventHub")]
        MicrosoftEventHub,
        #[serde(rename = "Microsoft.IoTHub")]
        MicrosoftIoTHub,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSourceCreateOrUpdateParameters {
    #[serde(flatten)]
    pub event_source_create_or_update_parameters: EventSourceCreateOrUpdateParameters,
    pub properties: EventHubEventSourceCreationProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubEventSourceCreateOrUpdateParameters {
    #[serde(flatten)]
    pub event_source_create_or_update_parameters: EventSourceCreateOrUpdateParameters,
    pub properties: IoTHubEventSourceCreationProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSourceUpdateParameters {
    pub kind: event_source_update_parameters::Kind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
pub mod event_source_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "Microsoft.EventHub")]
        MicrosoftEventHub,
        #[serde(rename = "Microsoft.IoTHub")]
        MicrosoftIoTHub,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSourceUpdateParameters {
    #[serde(flatten)]
    pub event_source_update_parameters: EventSourceUpdateParameters,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventHubEventSourceMutableProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubEventSourceUpdateParameters {
    #[serde(flatten)]
    pub event_source_update_parameters: EventSourceUpdateParameters,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IoTHubEventSourceMutableProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSourceListResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EventSourceResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSourceResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub kind: event_source_resource::Kind,
}
pub mod event_source_resource {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "Microsoft.EventHub")]
        MicrosoftEventHub,
        #[serde(rename = "Microsoft.IoTHub")]
        MicrosoftIoTHub,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSourceResource {
    #[serde(flatten)]
    pub event_source_resource: EventSourceResource,
    pub properties: EventHubEventSourceResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubEventSourceResource {
    #[serde(flatten)]
    pub event_source_resource: EventSourceResource,
    pub properties: IoTHubEventSourceResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSourceCommonProperties {
    #[serde(flatten)]
    pub resource_properties: ResourceProperties,
    #[serde(rename = "timestampPropertyName", default, skip_serializing_if = "Option::is_none")]
    pub timestamp_property_name: Option<String>,
    #[serde(rename = "localTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub local_timestamp: Option<LocalTimestamp>,
    #[serde(rename = "ingressStartAt", default, skip_serializing_if = "Option::is_none")]
    pub ingress_start_at: Option<IngressStartAtProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureEventSourceProperties {
    #[serde(flatten)]
    pub event_source_common_properties: EventSourceCommonProperties,
    #[serde(rename = "eventSourceResourceId")]
    pub event_source_resource_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSourceCommonProperties {
    #[serde(flatten)]
    pub azure_event_source_properties: AzureEventSourceProperties,
    #[serde(rename = "serviceBusNamespace")]
    pub service_bus_namespace: String,
    #[serde(rename = "eventHubName")]
    pub event_hub_name: String,
    #[serde(rename = "consumerGroupName")]
    pub consumer_group_name: String,
    #[serde(rename = "keyName")]
    pub key_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSourceCreationProperties {
    #[serde(flatten)]
    pub event_hub_event_source_common_properties: EventHubEventSourceCommonProperties,
    #[serde(rename = "sharedAccessKey")]
    pub shared_access_key: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSourceResourceProperties {
    #[serde(flatten)]
    pub event_hub_event_source_common_properties: EventHubEventSourceCommonProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubEventSourceCommonProperties {
    #[serde(flatten)]
    pub azure_event_source_properties: AzureEventSourceProperties,
    #[serde(rename = "iotHubName")]
    pub iot_hub_name: String,
    #[serde(rename = "consumerGroupName")]
    pub consumer_group_name: String,
    #[serde(rename = "keyName")]
    pub key_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubEventSourceCreationProperties {
    #[serde(flatten)]
    pub io_t_hub_event_source_common_properties: IoTHubEventSourceCommonProperties,
    #[serde(rename = "sharedAccessKey")]
    pub shared_access_key: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubEventSourceResourceProperties {
    #[serde(flatten)]
    pub io_t_hub_event_source_common_properties: IoTHubEventSourceCommonProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalTimestamp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<local_timestamp::Format>,
    #[serde(rename = "timeZoneOffset", default, skip_serializing_if = "Option::is_none")]
    pub time_zone_offset: Option<local_timestamp::TimeZoneOffset>,
}
pub mod local_timestamp {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Format {
        Embedded,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct TimeZoneOffset {
        #[serde(rename = "propertyName", default, skip_serializing_if = "Option::is_none")]
        pub property_name: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSourceMutableProperties {
    #[serde(rename = "timestampPropertyName", default, skip_serializing_if = "Option::is_none")]
    pub timestamp_property_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubEventSourceMutableProperties {
    #[serde(flatten)]
    pub event_source_mutable_properties: EventSourceMutableProperties,
    #[serde(rename = "sharedAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub shared_access_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoTHubEventSourceMutableProperties {
    #[serde(flatten)]
    pub event_source_mutable_properties: EventSourceMutableProperties,
    #[serde(rename = "sharedAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub shared_access_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceDataSetCreateOrUpdateParameters {
    #[serde(flatten)]
    pub create_or_update_tracked_resource_properties: CreateOrUpdateTrackedResourceProperties,
    pub properties: ReferenceDataSetCreationProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceDataSetUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceDataSetListResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReferenceDataSetResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceDataSetResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReferenceDataSetResourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceDataSetCreationProperties {
    #[serde(rename = "keyProperties")]
    pub key_properties: Vec<ReferenceDataSetKeyProperty>,
    #[serde(rename = "dataStringComparisonBehavior", default, skip_serializing_if = "Option::is_none")]
    pub data_string_comparison_behavior: Option<reference_data_set_creation_properties::DataStringComparisonBehavior>,
}
pub mod reference_data_set_creation_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataStringComparisonBehavior {
        Ordinal,
        OrdinalIgnoreCase,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceDataSetResourceProperties {
    #[serde(flatten)]
    pub reference_data_set_creation_properties: ReferenceDataSetCreationProperties,
    #[serde(flatten)]
    pub resource_properties: ResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceDataSetKeyProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<reference_data_set_key_property::Type>,
}
pub mod reference_data_set_key_property {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        String,
        Double,
        Bool,
        DateTime,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicyCreateOrUpdateParameters {
    pub properties: AccessPolicyResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicyUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessPolicyMutableProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicyListResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AccessPolicyResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicyResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessPolicyResourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicyResourceProperties {
    #[serde(rename = "principalObjectId", default, skip_serializing_if = "Option::is_none")]
    pub principal_object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicyMutableProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpoint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServiceConnectionState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Deleting,
    Failed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceProperties {
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
