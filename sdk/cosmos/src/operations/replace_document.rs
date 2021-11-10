use crate::cosmos_entity::{add_as_partition_key_header_serialized2, serialize_partition_key};
use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::document::DocumentAttributes;
use crate::ResourceQuota;

use azure_core::headers::session_token_from_headers;
use azure_core::prelude::*;
use azure_core::SessionToken;
use azure_core::{collect_pinned_stream, Request as HttpRequest, Response as HttpResponse};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct ReplaceDocumentOptions<'a> {
    partition_key: Option<String>,
    indexing_directive: IndexingDirective,
    if_match_condition: Option<IfMatchCondition<'a>>,
    if_modified_since: Option<IfModifiedSince<'a>>,
    consistency_level: Option<ConsistencyLevel>,
    allow_tentative_writes: TentativeWritesAllowance,
}

impl<'a> ReplaceDocumentOptions<'a> {
    pub fn new() -> Self {
        Self {
            partition_key: None,
            indexing_directive: IndexingDirective::Default,
            if_match_condition: None,
            if_modified_since: None,
            consistency_level: None,
            allow_tentative_writes: TentativeWritesAllowance::Deny,
        }
    }
}

impl<'a> ReplaceDocumentOptions<'a> {
    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition<'a> => Some(if_match_condition),
        if_modified_since: &'a DateTime<Utc> => Some(IfModifiedSince::new(if_modified_since)),
        allow_tentative_writes: TentativeWritesAllowance,
        indexing_directive: IndexingDirective,
    }

    pub fn partition_key<T: Serialize>(&mut self, partition_key: &T) -> crate::Result<()> {
        self.partition_key = Some(serialize_partition_key(partition_key)?);
        Ok(())
    }

    pub fn decorate_request<'b, D>(
        &self,
        request: &mut HttpRequest,
        document: &'b D,
        serialized_partition_key: &str,
    ) -> crate::Result<()>
    where
        D: Serialize,
    {
        let partition_key = self
            .partition_key
            .as_deref()
            .unwrap_or(serialized_partition_key);
        add_as_partition_key_header_serialized2(partition_key, request);

        azure_core::headers::add_mandatory_header2(&self.indexing_directive, request)?;
        azure_core::headers::add_optional_header2(&self.if_match_condition, request)?;
        azure_core::headers::add_optional_header2(&self.if_modified_since, request)?;
        azure_core::headers::add_optional_header2(&self.consistency_level, request)?;
        azure_core::headers::add_mandatory_header2(&self.allow_tentative_writes, request)?;

        let serialized = azure_core::to_json(document)?;
        request.set_body(serialized.into());

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ReplaceDocumentResponse {
    pub document_attributes: DocumentAttributes,
    pub content_location: String,
    pub last_state_change: DateTime<Utc>,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub lsn: u64,
    pub schema_version: String,
    pub alt_content_path: String,
    pub content_path: String,
    pub quorum_acked_lsn: Option<u64>,
    pub current_write_quorum: Option<u64>,
    pub current_replica_set_size: Option<u64>,
    pub role: u32,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub cosmos_quorum_acked_llsn: Option<u64>,
    pub session_token: SessionToken,
    pub charge: f64,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
    pub date: DateTime<Utc>,
}

impl ReplaceDocumentResponse {
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;
        let document_attributes = serde_json::from_slice(&*body)?;

        Ok(Self {
            content_location: content_location_from_headers(&headers)?.to_owned(),
            last_state_change: last_state_change_from_headers(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
            lsn: lsn_from_headers(&headers)?,
            schema_version: schema_version_from_headers(&headers)?.to_owned(),
            alt_content_path: alt_content_path_from_headers(&headers)?.to_owned(),
            content_path: content_path_from_headers(&headers)?.to_owned(),
            quorum_acked_lsn: quorum_acked_lsn_from_headers_optional(&headers)?,
            current_write_quorum: current_write_quorum_from_headers_optional(&headers)?,
            current_replica_set_size: current_replica_set_size_from_headers_optional(&headers)?,
            role: role_from_headers(&headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(&headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(&headers)?,
            transport_request_id: transport_request_id_from_headers(&headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(&headers)?,
            cosmos_quorum_acked_llsn: cosmos_quorum_acked_llsn_from_headers_optional(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            charge: request_charge_from_headers(&headers)?,
            service_version: service_version_from_headers(&headers)?.to_owned(),
            activity_id: activity_id_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?.to_owned(),
            date: date_from_headers(&headers)?,
            document_attributes,
        })
    }
}
