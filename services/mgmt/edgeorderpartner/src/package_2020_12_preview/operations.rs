#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use super::{models, API_VERSION};
#[non_exhaustive]
#[derive(Debug, thiserror :: Error)]
#[allow(non_camel_case_types)]
pub enum Error {
    #[error(transparent)]
    ListOperationsPartner(#[from] list_operations_partner::Error),
    #[error(transparent)]
    ManageInventoryMetadata(#[from] manage_inventory_metadata::Error),
    #[error(transparent)]
    ManageLink(#[from] manage_link::Error),
    #[error(transparent)]
    SearchInventories(#[from] search_inventories::Error),
}
pub async fn list_operations_partner(
    operation_config: &crate::OperationConfig,
) -> std::result::Result<models::OperationListResult, list_operations_partner::Error> {
    let http_client = operation_config.http_client();
    let url_str = &format!("{}/providers/Microsoft.EdgeOrderPartner/operations", operation_config.base_path(),);
    let mut url = url::Url::parse(url_str).map_err(list_operations_partner::Error::ParseUrlError)?;
    let mut req_builder = http::request::Builder::new();
    req_builder = req_builder.method(http::Method::GET);
    if let Some(token_credential) = operation_config.token_credential() {
        let token_response = token_credential
            .get_token(operation_config.token_credential_resource())
            .await
            .map_err(list_operations_partner::Error::GetTokenError)?;
        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
    }
    url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
    let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
    req_builder = req_builder.uri(url.as_str());
    let req = req_builder
        .body(req_body)
        .map_err(list_operations_partner::Error::BuildRequestError)?;
    let rsp = http_client
        .execute_request(req)
        .await
        .map_err(list_operations_partner::Error::ExecuteRequestError)?;
    match rsp.status() {
        http::StatusCode::OK => {
            let rsp_body = rsp.body();
            let rsp_value: models::OperationListResult = serde_json::from_slice(rsp_body)
                .map_err(|source| list_operations_partner::Error::DeserializeError(source, rsp_body.clone()))?;
            Ok(rsp_value)
        }
        status_code => {
            let rsp_body = rsp.body();
            let rsp_value: models::ErrorResponse = serde_json::from_slice(rsp_body)
                .map_err(|source| list_operations_partner::Error::DeserializeError(source, rsp_body.clone()))?;
            Err(list_operations_partner::Error::DefaultResponse {
                status_code,
                value: rsp_value,
            })
        }
    }
}
pub mod list_operations_partner {
    use super::{models, API_VERSION};
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("HTTP status code {}", status_code)]
        DefaultResponse {
            status_code: http::StatusCode,
            value: models::ErrorResponse,
        },
        #[error("Failed to parse request URL: {0}")]
        ParseUrlError(url::ParseError),
        #[error("Failed to build request: {0}")]
        BuildRequestError(http::Error),
        #[error("Failed to execute request: {0}")]
        ExecuteRequestError(azure_core::HttpError),
        #[error("Failed to serialize request body: {0}")]
        SerializeError(serde_json::Error),
        #[error("Failed to deserialize response: {0}, body: {1:?}")]
        DeserializeError(serde_json::Error, bytes::Bytes),
        #[error("Failed to get access token: {0}")]
        GetTokenError(azure_core::Error),
    }
}
pub async fn manage_inventory_metadata(
    operation_config: &crate::OperationConfig,
    family_identifier: &str,
    subscription_id: &str,
    location: &str,
    serial_number: &str,
    manage_inventory_metadata_request: &models::ManageInventoryMetadataRequest,
) -> std::result::Result<manage_inventory_metadata::Response, manage_inventory_metadata::Error> {
    let http_client = operation_config.http_client();
    let url_str = &format!(
        "{}/subscriptions/{}/providers/Microsoft.EdgeOrderPartner/locations/{}/productFamilies/{}/inventories/{}/manageInventoryMetadata",
        operation_config.base_path(),
        subscription_id,
        location,
        family_identifier,
        serial_number
    );
    let mut url = url::Url::parse(url_str).map_err(manage_inventory_metadata::Error::ParseUrlError)?;
    let mut req_builder = http::request::Builder::new();
    req_builder = req_builder.method(http::Method::POST);
    if let Some(token_credential) = operation_config.token_credential() {
        let token_response = token_credential
            .get_token(operation_config.token_credential_resource())
            .await
            .map_err(manage_inventory_metadata::Error::GetTokenError)?;
        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
    }
    url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
    req_builder = req_builder.header("content-type", "application/json");
    let req_body = azure_core::to_json(manage_inventory_metadata_request).map_err(manage_inventory_metadata::Error::SerializeError)?;
    req_builder = req_builder.uri(url.as_str());
    let req = req_builder
        .body(req_body)
        .map_err(manage_inventory_metadata::Error::BuildRequestError)?;
    let rsp = http_client
        .execute_request(req)
        .await
        .map_err(manage_inventory_metadata::Error::ExecuteRequestError)?;
    match rsp.status() {
        http::StatusCode::OK => Ok(manage_inventory_metadata::Response::Ok200),
        http::StatusCode::ACCEPTED => Ok(manage_inventory_metadata::Response::Accepted202),
        http::StatusCode::NO_CONTENT => Ok(manage_inventory_metadata::Response::NoContent204),
        status_code => {
            let rsp_body = rsp.body();
            let rsp_value: models::ErrorResponse = serde_json::from_slice(rsp_body)
                .map_err(|source| manage_inventory_metadata::Error::DeserializeError(source, rsp_body.clone()))?;
            Err(manage_inventory_metadata::Error::DefaultResponse {
                status_code,
                value: rsp_value,
            })
        }
    }
}
pub mod manage_inventory_metadata {
    use super::{models, API_VERSION};
    #[derive(Debug)]
    pub enum Response {
        Ok200,
        Accepted202,
        NoContent204,
    }
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("HTTP status code {}", status_code)]
        DefaultResponse {
            status_code: http::StatusCode,
            value: models::ErrorResponse,
        },
        #[error("Failed to parse request URL: {0}")]
        ParseUrlError(url::ParseError),
        #[error("Failed to build request: {0}")]
        BuildRequestError(http::Error),
        #[error("Failed to execute request: {0}")]
        ExecuteRequestError(azure_core::HttpError),
        #[error("Failed to serialize request body: {0}")]
        SerializeError(serde_json::Error),
        #[error("Failed to deserialize response: {0}, body: {1:?}")]
        DeserializeError(serde_json::Error, bytes::Bytes),
        #[error("Failed to get access token: {0}")]
        GetTokenError(azure_core::Error),
    }
}
pub async fn manage_link(
    operation_config: &crate::OperationConfig,
    family_identifier: &str,
    subscription_id: &str,
    location: &str,
    serial_number: &str,
    manage_link_request: &models::ManageLinkRequest,
) -> std::result::Result<manage_link::Response, manage_link::Error> {
    let http_client = operation_config.http_client();
    let url_str = &format!(
        "{}/subscriptions/{}/providers/Microsoft.EdgeOrderPartner/locations/{}/productFamilies/{}/inventories/{}/manageLink",
        operation_config.base_path(),
        subscription_id,
        location,
        family_identifier,
        serial_number
    );
    let mut url = url::Url::parse(url_str).map_err(manage_link::Error::ParseUrlError)?;
    let mut req_builder = http::request::Builder::new();
    req_builder = req_builder.method(http::Method::POST);
    if let Some(token_credential) = operation_config.token_credential() {
        let token_response = token_credential
            .get_token(operation_config.token_credential_resource())
            .await
            .map_err(manage_link::Error::GetTokenError)?;
        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
    }
    url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
    req_builder = req_builder.header("content-type", "application/json");
    let req_body = azure_core::to_json(manage_link_request).map_err(manage_link::Error::SerializeError)?;
    req_builder = req_builder.uri(url.as_str());
    let req = req_builder.body(req_body).map_err(manage_link::Error::BuildRequestError)?;
    let rsp = http_client
        .execute_request(req)
        .await
        .map_err(manage_link::Error::ExecuteRequestError)?;
    match rsp.status() {
        http::StatusCode::OK => Ok(manage_link::Response::Ok200),
        http::StatusCode::NO_CONTENT => Ok(manage_link::Response::NoContent204),
        status_code => {
            let rsp_body = rsp.body();
            let rsp_value: models::ErrorResponse =
                serde_json::from_slice(rsp_body).map_err(|source| manage_link::Error::DeserializeError(source, rsp_body.clone()))?;
            Err(manage_link::Error::DefaultResponse {
                status_code,
                value: rsp_value,
            })
        }
    }
}
pub mod manage_link {
    use super::{models, API_VERSION};
    #[derive(Debug)]
    pub enum Response {
        Ok200,
        NoContent204,
    }
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("HTTP status code {}", status_code)]
        DefaultResponse {
            status_code: http::StatusCode,
            value: models::ErrorResponse,
        },
        #[error("Failed to parse request URL: {0}")]
        ParseUrlError(url::ParseError),
        #[error("Failed to build request: {0}")]
        BuildRequestError(http::Error),
        #[error("Failed to execute request: {0}")]
        ExecuteRequestError(azure_core::HttpError),
        #[error("Failed to serialize request body: {0}")]
        SerializeError(serde_json::Error),
        #[error("Failed to deserialize response: {0}, body: {1:?}")]
        DeserializeError(serde_json::Error, bytes::Bytes),
        #[error("Failed to get access token: {0}")]
        GetTokenError(azure_core::Error),
    }
}
pub async fn search_inventories(
    operation_config: &crate::OperationConfig,
    subscription_id: &str,
    search_inventories_request: &models::SearchInventoriesRequest,
) -> std::result::Result<models::PartnerInventoryList, search_inventories::Error> {
    let http_client = operation_config.http_client();
    let url_str = &format!(
        "{}/subscriptions/{}/providers/Microsoft.EdgeOrderPartner/searchInventories",
        operation_config.base_path(),
        subscription_id
    );
    let mut url = url::Url::parse(url_str).map_err(search_inventories::Error::ParseUrlError)?;
    let mut req_builder = http::request::Builder::new();
    req_builder = req_builder.method(http::Method::POST);
    if let Some(token_credential) = operation_config.token_credential() {
        let token_response = token_credential
            .get_token(operation_config.token_credential_resource())
            .await
            .map_err(search_inventories::Error::GetTokenError)?;
        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
    }
    url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
    req_builder = req_builder.header("content-type", "application/json");
    let req_body = azure_core::to_json(search_inventories_request).map_err(search_inventories::Error::SerializeError)?;
    req_builder = req_builder.uri(url.as_str());
    let req = req_builder.body(req_body).map_err(search_inventories::Error::BuildRequestError)?;
    let rsp = http_client
        .execute_request(req)
        .await
        .map_err(search_inventories::Error::ExecuteRequestError)?;
    match rsp.status() {
        http::StatusCode::OK => {
            let rsp_body = rsp.body();
            let rsp_value: models::PartnerInventoryList =
                serde_json::from_slice(rsp_body).map_err(|source| search_inventories::Error::DeserializeError(source, rsp_body.clone()))?;
            Ok(rsp_value)
        }
        status_code => {
            let rsp_body = rsp.body();
            let rsp_value: models::ErrorResponse =
                serde_json::from_slice(rsp_body).map_err(|source| search_inventories::Error::DeserializeError(source, rsp_body.clone()))?;
            Err(search_inventories::Error::DefaultResponse {
                status_code,
                value: rsp_value,
            })
        }
    }
}
pub mod search_inventories {
    use super::{models, API_VERSION};
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("HTTP status code {}", status_code)]
        DefaultResponse {
            status_code: http::StatusCode,
            value: models::ErrorResponse,
        },
        #[error("Failed to parse request URL: {0}")]
        ParseUrlError(url::ParseError),
        #[error("Failed to build request: {0}")]
        BuildRequestError(http::Error),
        #[error("Failed to execute request: {0}")]
        ExecuteRequestError(azure_core::HttpError),
        #[error("Failed to serialize request body: {0}")]
        SerializeError(serde_json::Error),
        #[error("Failed to deserialize response: {0}, body: {1:?}")]
        DeserializeError(serde_json::Error, bytes::Bytes),
        #[error("Failed to get access token: {0}")]
        GetTokenError(azure_core::Error),
    }
}
