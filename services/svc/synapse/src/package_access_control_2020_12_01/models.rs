#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckPrincipalAccessRequest {
    pub subject: SubjectInfo,
    pub actions: Vec<RequiredAction>,
    pub scope: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequiredAction {
    pub id: String,
    #[serde(rename = "isDataAction")]
    pub is_data_action: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckPrincipalAccessResponse {
    #[serde(rename = "accessDecisions", default, skip_serializing_if = "Vec::is_empty")]
    pub access_decisions: Vec<CheckAccessDecision>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubjectInfo {
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[serde(rename = "groupIds", default, skip_serializing_if = "Vec::is_empty")]
    pub group_ids: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckAccessDecision {
    #[serde(rename = "accessDecision", default, skip_serializing_if = "Option::is_none")]
    pub access_decision: Option<String>,
    #[serde(rename = "actionId", default, skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(rename = "roleAssignment", default, skip_serializing_if = "Option::is_none")]
    pub role_assignment: Option<RoleAssignmentDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentRequest {
    #[serde(rename = "roleId")]
    pub role_id: String,
    #[serde(rename = "principalId")]
    pub principal_id: String,
    pub scope: String,
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentDetailsList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleAssignmentDetails>,
}
pub type RoleDefinitionsListResponse = Vec<SynapseRoleDefinition>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SynapseRoleDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isBuiltIn", default, skip_serializing_if = "Option::is_none")]
    pub is_built_in: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<SynapseRbacPermission>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    #[serde(rename = "availabilityStatus", default, skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SynapseRbacPermission {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<String>,
    #[serde(rename = "notActions", default, skip_serializing_if = "Vec::is_empty")]
    pub not_actions: Vec<String>,
    #[serde(rename = "dataActions", default, skip_serializing_if = "Vec::is_empty")]
    pub data_actions: Vec<String>,
    #[serde(rename = "notDataActions", default, skip_serializing_if = "Vec::is_empty")]
    pub not_data_actions: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorContract {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponse>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
