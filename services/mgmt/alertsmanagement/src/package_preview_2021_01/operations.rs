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
    Operations_List(#[from] operations::list::Error),
    #[error(transparent)]
    MigrateFromSmartDetection_StartMigration(#[from] migrate_from_smart_detection::start_migration::Error),
}
pub mod operations {
    use super::{models, API_VERSION};
    pub async fn list(operation_config: &crate::OperationConfig) -> std::result::Result<models::OperationsList, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/providers/Microsoft.AlertsManagement/operations", operation_config.base_path(),);
        let mut url = url::Url::parse(url_str).map_err(list::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(list::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(list::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(list::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: models::OperationsList =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                Err(list::Error::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                })
            }
        }
    }
    pub mod list {
        use super::{models, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
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
}
pub mod migrate_from_smart_detection {
    use super::{models, API_VERSION};
    pub async fn start_migration(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        smart_detection_migration_request: &models::SmartDetectionMigrationRequest,
    ) -> std::result::Result<start_migration::Response, start_migration::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.AlertsManagement/migrateFromSmartDetection",
            operation_config.base_path(),
            subscription_id
        );
        let mut url = url::Url::parse(url_str).map_err(start_migration::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(start_migration::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        req_builder = req_builder.header("content-type", "application/json");
        let req_body = azure_core::to_json(smart_detection_migration_request).map_err(start_migration::Error::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(start_migration::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(start_migration::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::ACCEPTED => {
                let rsp_body = rsp.body();
                let rsp_value: models::MigrationStatusResponse = serde_json::from_slice(rsp_body)
                    .map_err(|source| start_migration::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(start_migration::Response::Accepted202(rsp_value))
            }
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: models::MigrationStatusResponse = serde_json::from_slice(rsp_body)
                    .map_err(|source| start_migration::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(start_migration::Response::Ok200(rsp_value))
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: models::MigrationErrorResponse = serde_json::from_slice(rsp_body)
                    .map_err(|source| start_migration::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(start_migration::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod start_migration {
        use super::{models, API_VERSION};
        #[derive(Debug)]
        pub enum Response {
            Accepted202(models::MigrationStatusResponse),
            Ok200(models::MigrationStatusResponse),
        }
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::MigrationErrorResponse,
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
}
