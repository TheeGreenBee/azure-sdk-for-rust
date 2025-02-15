#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfidentialLedgerEnclaves {
    #[serde(rename = "currentNodeId")]
    pub current_node_id: EntityId,
    #[serde(rename = "enclaveQuotes")]
    pub enclave_quotes: EnclaveQuotes,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Consortium {
    pub members: Vec<ConsortiumMember>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsortiumMember {
    pub certificate: String,
    pub id: EntityId,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Constitution {
    pub digest: String,
    pub script: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnclaveQuote {
    #[serde(rename = "nodeId")]
    pub node_id: EntityId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mrenclave: Option<String>,
    #[serde(rename = "quoteVersion")]
    pub quote_version: String,
    pub raw: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnclaveQuotes {}
pub type EntityId = String;
pub type LedgerEntries = Vec<LedgerEntry>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub contents: String,
    #[serde(rename = "subLedgerId", default, skip_serializing_if = "Option::is_none")]
    pub sub_ledger_id: Option<SubLedgerId>,
    #[serde(rename = "transactionId", default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<TransactionId>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerQueryResult {
    pub state: LedgerQueryState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry: Option<LedgerEntry>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LedgerQueryState {
    Loading,
    Ready,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerUser {
    #[serde(rename = "assignedRole")]
    pub assigned_role: LedgerUserRole,
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LedgerUserRole {
    Administrator,
    Contributor,
    Reader,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerWriteResult {
    #[serde(rename = "subLedgerId")]
    pub sub_ledger_id: SubLedgerId,
}
pub type MerkleProof = Vec<MerkleProofElement>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MerkleProofElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedLedgerEntries {
    pub state: LedgerQueryState,
    #[serde(rename = "@nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    pub entries: LedgerEntries,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReceiptContents {
    pub leaf: String,
    #[serde(rename = "nodeId")]
    pub node_id: EntityId,
    pub proof: MerkleProof,
    pub root: String,
    pub signature: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignment {
    #[serde(rename = "roleName")]
    pub role_name: LedgerUserRole,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
pub type SubLedgerId = String;
pub type TransactionId = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionReceipt {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt: Option<ReceiptContents>,
    pub state: LedgerQueryState,
    #[serde(rename = "transactionId")]
    pub transaction_id: TransactionId,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TransactionState {
    Committed,
    Pending,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionStatus {
    pub state: TransactionState,
    #[serde(rename = "transactionId")]
    pub transaction_id: TransactionId,
}
pub type UserId = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfidentialLedgerError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ConfidentialLedgerErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfidentialLedgerErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Box<Option<ConfidentialLedgerErrorBody>>,
}
