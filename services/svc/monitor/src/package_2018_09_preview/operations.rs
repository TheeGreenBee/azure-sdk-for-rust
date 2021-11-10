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
    Metrics_Create(#[from] metrics::create::Error),
}
pub mod metrics {
    use super::{models, API_VERSION};
    pub async fn create(
        operation_config: &crate::OperationConfig,
        content_type: &str,
        content_length: i32,
        authorization: &str,
        subscription_id: &str,
        resource_group_name: &str,
        resource_provider: &str,
        resource_type_name: &str,
        resource_name: &str,
        body: &models::AzureMetricsDocument,
    ) -> std::result::Result<models::AzureMetricsResult, create::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourcegroups/{}/providers/{}/{}/{}/metrics",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            resource_provider,
            resource_type_name,
            resource_name
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
        req_builder = req_builder.header("Content-Type", content_type);
        req_builder = req_builder.header("Content-Length", content_length);
        req_builder = req_builder.header("Authorization", authorization);
        let req_body = azure_core::to_json(body).map_err(create::Error::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(create::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(create::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: models::AzureMetricsResult =
                    serde_json::from_slice(rsp_body).map_err(|source| create::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: models::AzureMetricsResult =
                    serde_json::from_slice(rsp_body).map_err(|source| create::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(create::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod create {
        use super::{models, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::AzureMetricsResult,
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
