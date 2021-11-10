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
    RoleAssignments_CheckPrincipalAccess(#[from] role_assignments::check_principal_access::Error),
    #[error(transparent)]
    RoleAssignments_ListRoleAssignments(#[from] role_assignments::list_role_assignments::Error),
    #[error(transparent)]
    RoleAssignments_GetRoleAssignmentById(#[from] role_assignments::get_role_assignment_by_id::Error),
    #[error(transparent)]
    RoleAssignments_CreateRoleAssignment(#[from] role_assignments::create_role_assignment::Error),
    #[error(transparent)]
    RoleAssignments_DeleteRoleAssignmentById(#[from] role_assignments::delete_role_assignment_by_id::Error),
    #[error(transparent)]
    RoleDefinitions_ListRoleDefinitions(#[from] role_definitions::list_role_definitions::Error),
    #[error(transparent)]
    RoleDefinitions_GetRoleDefinitionById(#[from] role_definitions::get_role_definition_by_id::Error),
    #[error(transparent)]
    RoleDefinitions_ListScopes(#[from] role_definitions::list_scopes::Error),
}
pub mod role_assignments {
    use super::{models, API_VERSION};
    pub async fn check_principal_access(
        operation_config: &crate::OperationConfig,
        request: &models::CheckPrincipalAccessRequest,
    ) -> std::result::Result<models::CheckPrincipalAccessResponse, check_principal_access::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/checkAccessSynapseRbac", operation_config.base_path(),);
        let mut url = url::Url::parse(url_str).map_err(check_principal_access::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(check_principal_access::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        req_builder = req_builder.header("content-type", "application/json");
        let req_body = azure_core::to_json(request).map_err(check_principal_access::Error::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(check_principal_access::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(check_principal_access::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: models::CheckPrincipalAccessResponse = serde_json::from_slice(rsp_body)
                    .map_err(|source| check_principal_access::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: models::ErrorContract = serde_json::from_slice(rsp_body)
                    .map_err(|source| check_principal_access::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(check_principal_access::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod check_principal_access {
        use super::{models, API_VERSION};
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
    pub async fn list_role_assignments(
        operation_config: &crate::OperationConfig,
        role_id: Option<&str>,
        principal_id: Option<&str>,
        scope: Option<&str>,
        x_ms_continuation: Option<&str>,
    ) -> std::result::Result<models::RoleAssignmentDetailsList, list_role_assignments::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/roleAssignments", operation_config.base_path(),);
        let mut url = url::Url::parse(url_str).map_err(list_role_assignments::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(list_role_assignments::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        if let Some(role_id) = role_id {
            url.query_pairs_mut().append_pair("roleId", role_id);
        }
        if let Some(principal_id) = principal_id {
            url.query_pairs_mut().append_pair("principalId", principal_id);
        }
        if let Some(scope) = scope {
            url.query_pairs_mut().append_pair("scope", scope);
        }
        if let Some(x_ms_continuation) = x_ms_continuation {
            req_builder = req_builder.header("x-ms-continuation", x_ms_continuation);
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(list_role_assignments::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(list_role_assignments::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: models::RoleAssignmentDetailsList = serde_json::from_slice(rsp_body)
                    .map_err(|source| list_role_assignments::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: models::ErrorContract = serde_json::from_slice(rsp_body)
                    .map_err(|source| list_role_assignments::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(list_role_assignments::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list_role_assignments {
        use super::{models, API_VERSION};
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
    pub async fn get_role_assignment_by_id(
        operation_config: &crate::OperationConfig,
        role_assignment_id: &str,
    ) -> std::result::Result<models::RoleAssignmentDetails, get_role_assignment_by_id::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/roleAssignments/{}", operation_config.base_path(), role_assignment_id);
        let mut url = url::Url::parse(url_str).map_err(get_role_assignment_by_id::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(get_role_assignment_by_id::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(get_role_assignment_by_id::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(get_role_assignment_by_id::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: models::RoleAssignmentDetails = serde_json::from_slice(rsp_body)
                    .map_err(|source| get_role_assignment_by_id::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: models::ErrorContract = serde_json::from_slice(rsp_body)
                    .map_err(|source| get_role_assignment_by_id::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(get_role_assignment_by_id::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod get_role_assignment_by_id {
        use super::{models, API_VERSION};
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
    pub async fn create_role_assignment(
        operation_config: &crate::OperationConfig,
        request: &models::RoleAssignmentRequest,
        role_assignment_id: &str,
    ) -> std::result::Result<models::RoleAssignmentDetails, create_role_assignment::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/roleAssignments/{}", operation_config.base_path(), role_assignment_id);
        let mut url = url::Url::parse(url_str).map_err(create_role_assignment::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PUT);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(create_role_assignment::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        req_builder = req_builder.header("content-type", "application/json");
        let req_body = azure_core::to_json(request).map_err(create_role_assignment::Error::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(create_role_assignment::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(create_role_assignment::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: models::RoleAssignmentDetails = serde_json::from_slice(rsp_body)
                    .map_err(|source| create_role_assignment::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: models::ErrorContract = serde_json::from_slice(rsp_body)
                    .map_err(|source| create_role_assignment::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(create_role_assignment::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod create_role_assignment {
        use super::{models, API_VERSION};
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
    pub async fn delete_role_assignment_by_id(
        operation_config: &crate::OperationConfig,
        role_assignment_id: &str,
        scope: Option<&str>,
    ) -> std::result::Result<delete_role_assignment_by_id::Response, delete_role_assignment_by_id::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/roleAssignments/{}", operation_config.base_path(), role_assignment_id);
        let mut url = url::Url::parse(url_str).map_err(delete_role_assignment_by_id::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::DELETE);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(delete_role_assignment_by_id::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        if let Some(scope) = scope {
            url.query_pairs_mut().append_pair("scope", scope);
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(delete_role_assignment_by_id::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(delete_role_assignment_by_id::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => Ok(delete_role_assignment_by_id::Response::Ok200),
            http::StatusCode::NO_CONTENT => Ok(delete_role_assignment_by_id::Response::NoContent204),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: models::ErrorContract = serde_json::from_slice(rsp_body)
                    .map_err(|source| delete_role_assignment_by_id::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(delete_role_assignment_by_id::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod delete_role_assignment_by_id {
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
pub mod role_definitions {
    use super::{models, API_VERSION};
    pub async fn list_role_definitions(
        operation_config: &crate::OperationConfig,
        is_built_in: Option<bool>,
        scope: Option<&str>,
    ) -> std::result::Result<models::RoleDefinitionsListResponse, list_role_definitions::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/roleDefinitions", operation_config.base_path(),);
        let mut url = url::Url::parse(url_str).map_err(list_role_definitions::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(list_role_definitions::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        if let Some(is_built_in) = is_built_in {
            url.query_pairs_mut().append_pair("isBuiltIn", is_built_in.to_string().as_str());
        }
        if let Some(scope) = scope {
            url.query_pairs_mut().append_pair("scope", scope);
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(list_role_definitions::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(list_role_definitions::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: models::RoleDefinitionsListResponse = serde_json::from_slice(rsp_body)
                    .map_err(|source| list_role_definitions::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: models::ErrorContract = serde_json::from_slice(rsp_body)
                    .map_err(|source| list_role_definitions::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(list_role_definitions::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list_role_definitions {
        use super::{models, API_VERSION};
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
    pub async fn get_role_definition_by_id(
        operation_config: &crate::OperationConfig,
        role_definition_id: &str,
    ) -> std::result::Result<models::SynapseRoleDefinition, get_role_definition_by_id::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/roleDefinitions/{}", operation_config.base_path(), role_definition_id);
        let mut url = url::Url::parse(url_str).map_err(get_role_definition_by_id::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(get_role_definition_by_id::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(get_role_definition_by_id::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(get_role_definition_by_id::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: models::SynapseRoleDefinition = serde_json::from_slice(rsp_body)
                    .map_err(|source| get_role_definition_by_id::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: models::ErrorContract = serde_json::from_slice(rsp_body)
                    .map_err(|source| get_role_definition_by_id::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(get_role_definition_by_id::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod get_role_definition_by_id {
        use super::{models, API_VERSION};
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
    pub async fn list_scopes(operation_config: &crate::OperationConfig) -> std::result::Result<Vec<String>, list_scopes::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/rbacScopes", operation_config.base_path(),);
        let mut url = url::Url::parse(url_str).map_err(list_scopes::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(list_scopes::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(list_scopes::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(list_scopes::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: Vec<String> =
                    serde_json::from_slice(rsp_body).map_err(|source| list_scopes::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: models::ErrorContract =
                    serde_json::from_slice(rsp_body).map_err(|source| list_scopes::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(list_scopes::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list_scopes {
        use super::{models, API_VERSION};
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
