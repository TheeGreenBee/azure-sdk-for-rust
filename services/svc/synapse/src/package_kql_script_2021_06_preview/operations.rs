#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
pub mod kql_scripts {
    use crate::models::*;
    pub async fn get_all(
        operation_config: &crate::OperationConfig,
    ) -> std::result::Result<KqlScriptsResourceCollectionResponse, get_all::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/kqlScripts", operation_config.base_path(),);
        let mut url = url::Url::parse(url_str).map_err(get_all::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(get_all::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(get_all::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(get_all::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: KqlScriptsResourceCollectionResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| get_all::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorContract =
                    serde_json::from_slice(rsp_body).map_err(|source| get_all::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(get_all::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod get_all {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorContract,
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
pub mod kql_script {
    use crate::models::*;
    pub async fn get_by_name(
        operation_config: &crate::OperationConfig,
        kql_script_name: &str,
    ) -> std::result::Result<KqlScriptResource, get_by_name::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/kqlScripts/{}", operation_config.base_path(), kql_script_name);
        let mut url = url::Url::parse(url_str).map_err(get_by_name::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(get_by_name::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(get_by_name::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(get_by_name::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: KqlScriptResource =
                    serde_json::from_slice(rsp_body).map_err(|source| get_by_name::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorContract =
                    serde_json::from_slice(rsp_body).map_err(|source| get_by_name::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(get_by_name::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod get_by_name {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorContract,
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
        kql_script_name: &str,
        kql_script: &KqlScriptResource,
    ) -> std::result::Result<create_or_update::Response, create_or_update::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/kqlScripts/{}", operation_config.base_path(), kql_script_name);
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
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        req_builder = req_builder.header("content-type", "application/json");
        let req_body = azure_core::to_json(kql_script).map_err(create_or_update::Error::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(create_or_update::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(create_or_update::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: KqlScriptResource = serde_json::from_slice(rsp_body)
                    .map_err(|source| create_or_update::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(create_or_update::Response::Ok200(rsp_value))
            }
            http::StatusCode::ACCEPTED => Ok(create_or_update::Response::Accepted202),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorContract = serde_json::from_slice(rsp_body)
                    .map_err(|source| create_or_update::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(create_or_update::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod create_or_update {
        use crate::{models, models::*};
        #[derive(Debug)]
        pub enum Response {
            Ok200(KqlScriptResource),
            Accepted202,
        }
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorContract,
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
    pub async fn delete_by_name(
        operation_config: &crate::OperationConfig,
        kql_script_name: &str,
    ) -> std::result::Result<delete_by_name::Response, delete_by_name::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/kqlScripts/{}", operation_config.base_path(), kql_script_name);
        let mut url = url::Url::parse(url_str).map_err(delete_by_name::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::DELETE);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(delete_by_name::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(delete_by_name::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(delete_by_name::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => Ok(delete_by_name::Response::Ok200),
            http::StatusCode::ACCEPTED => Ok(delete_by_name::Response::Accepted202),
            http::StatusCode::NO_CONTENT => Ok(delete_by_name::Response::NoContent204),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorContract =
                    serde_json::from_slice(rsp_body).map_err(|source| delete_by_name::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(delete_by_name::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod delete_by_name {
        use crate::{models, models::*};
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
                value: models::ErrorContract,
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
    pub async fn rename(
        operation_config: &crate::OperationConfig,
        kql_script_name: &str,
        rename_request: &ArtifactRenameRequest,
    ) -> std::result::Result<rename::Response, rename::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/kqlScripts/{}/rename", operation_config.base_path(), kql_script_name);
        let mut url = url::Url::parse(url_str).map_err(rename::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(rename::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        req_builder = req_builder.header("content-type", "application/json");
        let req_body = azure_core::to_json(rename_request).map_err(rename::Error::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(rename::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(rename::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => Ok(rename::Response::Ok200),
            http::StatusCode::ACCEPTED => Ok(rename::Response::Accepted202),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorContract =
                    serde_json::from_slice(rsp_body).map_err(|source| rename::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(rename::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod rename {
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
                value: models::ErrorContract,
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
