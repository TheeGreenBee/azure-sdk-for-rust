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
    AzureBareMetalInstances_Start(#[from] azure_bare_metal_instances::start::Error),
    #[error(transparent)]
    AzureBareMetalInstances_Restart(#[from] azure_bare_metal_instances::restart::Error),
    #[error(transparent)]
    AzureBareMetalInstances_Shutdown(#[from] azure_bare_metal_instances::shutdown::Error),
    #[error(transparent)]
    AzureBareMetalInstances_ListBySubscription(#[from] azure_bare_metal_instances::list_by_subscription::Error),
    #[error(transparent)]
    AzureBareMetalInstances_List(#[from] azure_bare_metal_instances::list::Error),
    #[error(transparent)]
    Operations_List(#[from] operations::list::Error),
    #[error(transparent)]
    AzureBareMetalInstances_Get(#[from] azure_bare_metal_instances::get::Error),
    #[error(transparent)]
    AzureBareMetalInstances_Update(#[from] azure_bare_metal_instances::update::Error),
    #[error(transparent)]
    AzureBareMetalInstances_Delete(#[from] azure_bare_metal_instances::delete::Error),
}
pub mod azure_bare_metal_instances {
    use super::{models, API_VERSION};
    pub async fn start(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        azure_bare_metal_instance_name: &str,
    ) -> std::result::Result<start::Response, start::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.BareMetalInfrastructure/bareMetalInstances/{}/start",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            azure_bare_metal_instance_name
        );
        let mut url = url::Url::parse(url_str).map_err(start::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(start::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.header(http::header::CONTENT_LENGTH, 0);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(start::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(start::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => Ok(start::Response::Ok200),
            http::StatusCode::ACCEPTED => Ok(start::Response::Accepted202),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: models::ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| start::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(start::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod start {
        use super::{models, API_VERSION};
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
    pub async fn restart(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        azure_bare_metal_instance_name: &str,
    ) -> std::result::Result<restart::Response, restart::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.BareMetalInfrastructure/bareMetalInstances/{}/restart",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            azure_bare_metal_instance_name
        );
        let mut url = url::Url::parse(url_str).map_err(restart::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(restart::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.header(http::header::CONTENT_LENGTH, 0);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(restart::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(restart::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => Ok(restart::Response::Ok200),
            http::StatusCode::ACCEPTED => Ok(restart::Response::Accepted202),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: models::ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| restart::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(restart::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod restart {
        use super::{models, API_VERSION};
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
    pub async fn shutdown(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        azure_bare_metal_instance_name: &str,
    ) -> std::result::Result<shutdown::Response, shutdown::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.BareMetalInfrastructure/bareMetalInstances/{}/shutdown",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            azure_bare_metal_instance_name
        );
        let mut url = url::Url::parse(url_str).map_err(shutdown::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(shutdown::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.header(http::header::CONTENT_LENGTH, 0);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(shutdown::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(shutdown::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => Ok(shutdown::Response::Ok200),
            http::StatusCode::ACCEPTED => Ok(shutdown::Response::Accepted202),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: models::ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| shutdown::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(shutdown::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod shutdown {
        use super::{models, API_VERSION};
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
    pub async fn list_by_subscription(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
    ) -> std::result::Result<models::AzureBareMetalInstancesListResult, list_by_subscription::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.BareMetalInfrastructure/bareMetalInstances",
            operation_config.base_path(),
            subscription_id
        );
        let mut url = url::Url::parse(url_str).map_err(list_by_subscription::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(list_by_subscription::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(list_by_subscription::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(list_by_subscription::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: models::AzureBareMetalInstancesListResult = serde_json::from_slice(rsp_body)
                    .map_err(|source| list_by_subscription::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: models::ErrorResponse = serde_json::from_slice(rsp_body)
                    .map_err(|source| list_by_subscription::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(list_by_subscription::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list_by_subscription {
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
    pub async fn list(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> std::result::Result<models::AzureBareMetalInstancesListResult, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.BareMetalInfrastructure/bareMetalInstances",
            operation_config.base_path(),
            subscription_id,
            resource_group_name
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
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(list::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(list::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: models::AzureBareMetalInstancesListResult =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: models::ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(list::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list {
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
    pub async fn get(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        azure_bare_metal_instance_name: &str,
    ) -> std::result::Result<models::AzureBareMetalInstance, get::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.BareMetalInfrastructure/bareMetalInstances/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            azure_bare_metal_instance_name
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
                let rsp_value: models::AzureBareMetalInstance =
                    serde_json::from_slice(rsp_body).map_err(|source| get::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: models::ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| get::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(get::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod get {
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
    pub async fn update(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        azure_bare_metal_instance_name: &str,
        tags_parameter: &models::Tags,
    ) -> std::result::Result<models::AzureBareMetalInstance, update::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.BareMetalInfrastructure/bareMetalInstances/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            azure_bare_metal_instance_name
        );
        let mut url = url::Url::parse(url_str).map_err(update::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PATCH);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(update::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        req_builder = req_builder.header("content-type", "application/json");
        let req_body = azure_core::to_json(tags_parameter).map_err(update::Error::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(update::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(update::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: models::AzureBareMetalInstance =
                    serde_json::from_slice(rsp_body).map_err(|source| update::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: models::ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| update::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(update::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod update {
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
    pub async fn delete(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        azure_bare_metal_instance_name: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.BareMetalInfrastructure/bareMetalInstances/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            azure_bare_metal_instance_name
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
            http::StatusCode::OK => Ok(delete::Response::Ok200),
            http::StatusCode::ACCEPTED => Ok(delete::Response::Accepted202),
            http::StatusCode::NO_CONTENT => Ok(delete::Response::NoContent204),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: models::ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| delete::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(delete::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod delete {
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
}
pub mod operations {
    use super::{models, API_VERSION};
    pub async fn list(operation_config: &crate::OperationConfig) -> std::result::Result<models::OperationList, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/providers/Microsoft.BareMetalInfrastructure/operations",
            operation_config.base_path(),
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
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(list::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(list::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: models::OperationList =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: models::ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(list::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list {
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
}
