#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
pub mod diagnostic_operations {
    use crate::models::*;
    pub async fn read(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        location: &str,
        operation_id: &str,
    ) -> std::result::Result<read::Response, read::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Compute/locations/{}/diagnosticOperations/{}",
            operation_config.base_path(),
            subscription_id,
            location,
            operation_id
        );
        let mut url = url::Url::parse(url_str).map_err(read::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(read::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(read::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(read::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => Ok(read::Response::Ok200),
            http::StatusCode::ACCEPTED => Ok(read::Response::Accepted202),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| read::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(read::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod read {
        use crate::{models, models::*};
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
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
}
pub mod disk_inspection {
    use crate::models::*;
    pub async fn create(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        location: &str,
        run_disk_inspection_input: &RunDiskInspectionInput,
    ) -> std::result::Result<String, create::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Compute/locations/{}/diagnostics/diskInspection/run",
            operation_config.base_path(),
            subscription_id,
            location
        );
        let mut url = url::Url::parse(url_str).map_err(create::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(create::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        req_builder = req_builder.header("content-type", "application/json");
        let req_body = azure_core::to_json(run_disk_inspection_input).map_err(create::Error::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(create::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(create::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::ACCEPTED => {
                let rsp_body = rsp.body();
                let rsp_value: String =
                    serde_json::from_slice(rsp_body).map_err(|source| create::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| create::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(create::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod create {
        use crate::{models, models::*};
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
    pub async fn get(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        location: &str,
    ) -> std::result::Result<ComputeDiagnosticBase, get::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Compute/locations/{}/diagnostics/diskInspection",
            operation_config.base_path(),
            subscription_id,
            location
        );
        let mut url = url::Url::parse(url_str).map_err(get::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(get::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(get::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(get::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: ComputeDiagnosticBase =
                    serde_json::from_slice(rsp_body).map_err(|source| get::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| get::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(get::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod get {
        use crate::{models, models::*};
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
}
pub mod diagnostics {
    use crate::models::*;
    pub async fn list(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        location: &str,
    ) -> std::result::Result<ComputeDiagnosticsList, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Compute/locations/{}/diagnostics",
            operation_config.base_path(),
            subscription_id,
            location
        );
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
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(list::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(list::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: ComputeDiagnosticsList =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(list::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list {
        use crate::{models, models::*};
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
}
