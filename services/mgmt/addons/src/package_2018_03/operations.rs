#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use super::{models, models::*, API_VERSION};
pub mod operations {
    use super::{models, models::*, API_VERSION};
    pub async fn list(operation_config: &crate::OperationConfig) -> std::result::Result<OperationListValue, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/providers/Microsoft.Addons/operations", operation_config.base_path(),);
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
                let rsp_value: OperationListValue =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorDefinition =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(list::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list {
        use super::{models, models::*, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorDefinition,
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
pub mod support_plan_types {
    use super::{models, models::*, API_VERSION};
    pub async fn get(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        provider_name: &str,
        plan_type_name: &str,
    ) -> std::result::Result<CanonicalSupportPlanResponseEnvelope, get::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Addons/supportProviders/{}/supportPlanTypes/{}",
            operation_config.base_path(),
            subscription_id,
            provider_name,
            plan_type_name
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
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(get::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(get::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: CanonicalSupportPlanResponseEnvelope =
                    serde_json::from_slice(rsp_body).map_err(|source| get::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            http::StatusCode::NOT_FOUND => Err(get::Error::NotFound404 {}),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorDefinition =
                    serde_json::from_slice(rsp_body).map_err(|source| get::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(get::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod get {
        use super::{models, models::*, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Error response #response_type")]
            NotFound404 {},
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorDefinition,
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
    pub async fn create_or_update(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        provider_name: &str,
        plan_type_name: &str,
    ) -> std::result::Result<create_or_update::Response, create_or_update::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Addons/supportProviders/{}/supportPlanTypes/{}",
            operation_config.base_path(),
            subscription_id,
            provider_name,
            plan_type_name
        );
        let mut url = url::Url::parse(url_str).map_err(create_or_update::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PUT);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(create_or_update::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(create_or_update::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(create_or_update::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: CanonicalSupportPlanResponseEnvelope = serde_json::from_slice(rsp_body)
                    .map_err(|source| create_or_update::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(create_or_update::Response::Ok200(rsp_value))
            }
            http::StatusCode::CREATED => {
                let rsp_body = rsp.body();
                let rsp_value: CanonicalSupportPlanResponseEnvelope = serde_json::from_slice(rsp_body)
                    .map_err(|source| create_or_update::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(create_or_update::Response::Created201(rsp_value))
            }
            http::StatusCode::NOT_FOUND => Err(create_or_update::Error::NotFound404 {}),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorDefinition = serde_json::from_slice(rsp_body)
                    .map_err(|source| create_or_update::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(create_or_update::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod create_or_update {
        use super::{models, models::*, API_VERSION};
        #[derive(Debug)]
        pub enum Response {
            Ok200(CanonicalSupportPlanResponseEnvelope),
            Created201(CanonicalSupportPlanResponseEnvelope),
        }
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Error response #response_type")]
            NotFound404 {},
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorDefinition,
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
    pub async fn delete(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        provider_name: &str,
        plan_type_name: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Addons/supportProviders/{}/supportPlanTypes/{}",
            operation_config.base_path(),
            subscription_id,
            provider_name,
            plan_type_name
        );
        let mut url = url::Url::parse(url_str).map_err(delete::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::DELETE);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(delete::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(delete::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(delete::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::ACCEPTED => {
                let rsp_body = rsp.body();
                let rsp_value: CanonicalSupportPlanResponseEnvelope =
                    serde_json::from_slice(rsp_body).map_err(|source| delete::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(delete::Response::Accepted202(rsp_value))
            }
            http::StatusCode::NO_CONTENT => Ok(delete::Response::NoContent204),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorDefinition =
                    serde_json::from_slice(rsp_body).map_err(|source| delete::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(delete::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod delete {
        use super::{models, models::*, API_VERSION};
        #[derive(Debug)]
        pub enum Response {
            Accepted202(CanonicalSupportPlanResponseEnvelope),
            NoContent204,
        }
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorDefinition,
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
    pub async fn list_info(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
    ) -> std::result::Result<CanonicalSupportPlanInfo, list_info::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Addons/supportProviders/canonical/listSupportPlanInfo",
            operation_config.base_path(),
            subscription_id
        );
        let mut url = url::Url::parse(url_str).map_err(list_info::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(list_info::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.header(http::header::CONTENT_LENGTH, 0);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(list_info::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(list_info::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: CanonicalSupportPlanInfo =
                    serde_json::from_slice(rsp_body).map_err(|source| list_info::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            http::StatusCode::NOT_FOUND => Err(list_info::Error::NotFound404 {}),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorDefinition =
                    serde_json::from_slice(rsp_body).map_err(|source| list_info::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(list_info::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list_info {
        use super::{models, models::*, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Error response #response_type")]
            NotFound404 {},
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorDefinition,
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
