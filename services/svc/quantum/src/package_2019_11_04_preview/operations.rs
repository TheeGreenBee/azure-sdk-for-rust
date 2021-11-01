#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use super::{models, models::*, API_VERSION};
pub mod jobs {
    use super::{models, models::*, API_VERSION};
    pub async fn list(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
    ) -> std::result::Result<JobDetailsList, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Quantum/workspaces/{}/jobs",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            workspace_name
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
                let rsp_value: JobDetailsList =
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
        use super::{models, models::*, API_VERSION};
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
    pub async fn get(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
        job_id: &str,
    ) -> std::result::Result<JobDetails, get::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Quantum/workspaces/{}/jobs/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            workspace_name,
            job_id
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
                let rsp_value: JobDetails =
                    serde_json::from_slice(rsp_body).map_err(|source| get::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: RestError =
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
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::RestError,
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
    pub async fn create(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
        job_id: &str,
        job: &JobDetails,
    ) -> std::result::Result<create::Response, create::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Quantum/workspaces/{}/jobs/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            workspace_name,
            job_id
        );
        let mut url = url::Url::parse(url_str).map_err(create::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PUT);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(create::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        req_builder = req_builder.header("content-type", "application/json");
        let req_body = azure_core::to_json(job).map_err(create::Error::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(create::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(create::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: JobDetails =
                    serde_json::from_slice(rsp_body).map_err(|source| create::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(create::Response::Ok200(rsp_value))
            }
            http::StatusCode::CREATED => {
                let rsp_body = rsp.body();
                let rsp_value: JobDetails =
                    serde_json::from_slice(rsp_body).map_err(|source| create::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(create::Response::Created201(rsp_value))
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: RestError =
                    serde_json::from_slice(rsp_body).map_err(|source| create::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(create::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod create {
        use super::{models, models::*, API_VERSION};
        #[derive(Debug)]
        pub enum Response {
            Ok200(JobDetails),
            Created201(JobDetails),
        }
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::RestError,
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
    pub async fn cancel(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
        job_id: &str,
    ) -> std::result::Result<(), cancel::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Quantum/workspaces/{}/jobs/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            workspace_name,
            job_id
        );
        let mut url = url::Url::parse(url_str).map_err(cancel::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::DELETE);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(cancel::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(cancel::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(cancel::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::NO_CONTENT => Ok(()),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: RestError =
                    serde_json::from_slice(rsp_body).map_err(|source| cancel::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(cancel::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod cancel {
        use super::{models, models::*, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::RestError,
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
pub mod providers {
    use super::{models, models::*, API_VERSION};
    pub async fn get_status(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
    ) -> std::result::Result<ProviderStatusList, get_status::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Quantum/workspaces/{}/providerStatus",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            workspace_name
        );
        let mut url = url::Url::parse(url_str).map_err(get_status::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(get_status::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(get_status::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(get_status::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: ProviderStatusList =
                    serde_json::from_slice(rsp_body).map_err(|source| get_status::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: RestError =
                    serde_json::from_slice(rsp_body).map_err(|source| get_status::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(get_status::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod get_status {
        use super::{models, models::*, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::RestError,
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
pub mod storage {
    use super::{models, models::*, API_VERSION};
    pub async fn sas_uri(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
        blob_details: &BlobDetails,
    ) -> std::result::Result<SasUriResponse, sas_uri::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Quantum/workspaces/{}/storage/sasUri",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            workspace_name
        );
        let mut url = url::Url::parse(url_str).map_err(sas_uri::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(sas_uri::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        req_builder = req_builder.header("content-type", "application/json");
        let req_body = azure_core::to_json(blob_details).map_err(sas_uri::Error::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(sas_uri::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(sas_uri::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: SasUriResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| sas_uri::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: RestError =
                    serde_json::from_slice(rsp_body).map_err(|source| sas_uri::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(sas_uri::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod sas_uri {
        use super::{models, models::*, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::RestError,
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
pub mod quotas {
    use super::{models, models::*, API_VERSION};
    pub async fn list(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
    ) -> std::result::Result<QuotaList, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Quantum/workspaces/{}/quotas",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            workspace_name
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
                let rsp_value: QuotaList =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: RestError =
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
                value: models::RestError,
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
