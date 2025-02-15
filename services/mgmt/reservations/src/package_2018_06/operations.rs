#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
pub async fn get_catalog(
    operation_config: &crate::OperationConfig,
    subscription_id: &str,
    reserved_resource_type: &str,
    location: Option<&str>,
) -> std::result::Result<Vec<Catalog>, get_catalog::Error> {
    let http_client = operation_config.http_client();
    let url_str = &format!(
        "{}/subscriptions/{}/providers/Microsoft.Capacity/catalogs",
        operation_config.base_path(),
        subscription_id
    );
    let mut url = url::Url::parse(url_str).map_err(get_catalog::Error::ParseUrlError)?;
    let mut req_builder = http::request::Builder::new();
    req_builder = req_builder.method(http::Method::GET);
    if let Some(token_credential) = operation_config.token_credential() {
        let token_response = token_credential
            .get_token(operation_config.token_credential_resource())
            .await
            .map_err(get_catalog::Error::GetTokenError)?;
        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
    }
    url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
    url.query_pairs_mut().append_pair("reservedResourceType", reserved_resource_type);
    if let Some(location) = location {
        url.query_pairs_mut().append_pair("location", location);
    }
    let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
    req_builder = req_builder.uri(url.as_str());
    let req = req_builder.body(req_body).map_err(get_catalog::Error::BuildRequestError)?;
    let rsp = http_client
        .execute_request(req)
        .await
        .map_err(get_catalog::Error::ExecuteRequestError)?;
    match rsp.status() {
        http::StatusCode::OK => {
            let rsp_body = rsp.body();
            let rsp_value: Vec<Catalog> =
                serde_json::from_slice(rsp_body).map_err(|source| get_catalog::Error::DeserializeError(source, rsp_body.clone()))?;
            Ok(rsp_value)
        }
        status_code => {
            let rsp_body = rsp.body();
            let rsp_value: Error =
                serde_json::from_slice(rsp_body).map_err(|source| get_catalog::Error::DeserializeError(source, rsp_body.clone()))?;
            Err(get_catalog::Error::DefaultResponse {
                status_code,
                value: rsp_value,
            })
        }
    }
}
pub mod get_catalog {
    use crate::{models, models::*};
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("HTTP status code {}", status_code)]
        DefaultResponse {
            status_code: http::StatusCode,
            value: models::Error,
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
pub async fn get_applied_reservation_list(
    operation_config: &crate::OperationConfig,
    subscription_id: &str,
) -> std::result::Result<AppliedReservations, get_applied_reservation_list::Error> {
    let http_client = operation_config.http_client();
    let url_str = &format!(
        "{}/subscriptions/{}/providers/Microsoft.Capacity/appliedReservations",
        operation_config.base_path(),
        subscription_id
    );
    let mut url = url::Url::parse(url_str).map_err(get_applied_reservation_list::Error::ParseUrlError)?;
    let mut req_builder = http::request::Builder::new();
    req_builder = req_builder.method(http::Method::GET);
    if let Some(token_credential) = operation_config.token_credential() {
        let token_response = token_credential
            .get_token(operation_config.token_credential_resource())
            .await
            .map_err(get_applied_reservation_list::Error::GetTokenError)?;
        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
    }
    url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
    let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
    req_builder = req_builder.uri(url.as_str());
    let req = req_builder
        .body(req_body)
        .map_err(get_applied_reservation_list::Error::BuildRequestError)?;
    let rsp = http_client
        .execute_request(req)
        .await
        .map_err(get_applied_reservation_list::Error::ExecuteRequestError)?;
    match rsp.status() {
        http::StatusCode::OK => {
            let rsp_body = rsp.body();
            let rsp_value: AppliedReservations = serde_json::from_slice(rsp_body)
                .map_err(|source| get_applied_reservation_list::Error::DeserializeError(source, rsp_body.clone()))?;
            Ok(rsp_value)
        }
        status_code => {
            let rsp_body = rsp.body();
            let rsp_value: Error = serde_json::from_slice(rsp_body)
                .map_err(|source| get_applied_reservation_list::Error::DeserializeError(source, rsp_body.clone()))?;
            Err(get_applied_reservation_list::Error::DefaultResponse {
                status_code,
                value: rsp_value,
            })
        }
    }
}
pub mod get_applied_reservation_list {
    use crate::{models, models::*};
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("HTTP status code {}", status_code)]
        DefaultResponse {
            status_code: http::StatusCode,
            value: models::Error,
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
pub mod reservation_order {
    use crate::models::*;
    pub async fn list(operation_config: &crate::OperationConfig) -> std::result::Result<ReservationOrderList, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/providers/Microsoft.Capacity/reservationOrders", operation_config.base_path(),);
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
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(list::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(list::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: ReservationOrderList =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: Error =
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
                value: models::Error,
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
        reservation_order_id: &str,
    ) -> std::result::Result<ReservationOrderResponse, get::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/providers/Microsoft.Capacity/reservationOrders/{}",
            operation_config.base_path(),
            reservation_order_id
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
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(get::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(get::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: ReservationOrderResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| get::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: Error =
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
                value: models::Error,
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
pub mod reservation {
    use crate::models::*;
    pub async fn split(
        operation_config: &crate::OperationConfig,
        reservation_order_id: &str,
        body: &SplitRequest,
    ) -> std::result::Result<split::Response, split::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/providers/Microsoft.Capacity/reservationOrders/{}/split",
            operation_config.base_path(),
            reservation_order_id
        );
        let mut url = url::Url::parse(url_str).map_err(split::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(split::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        req_builder = req_builder.header("content-type", "application/json");
        let req_body = azure_core::to_json(body).map_err(split::Error::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(split::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(split::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: Vec<ReservationResponse> =
                    serde_json::from_slice(rsp_body).map_err(|source| split::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(split::Response::Ok200(rsp_value))
            }
            http::StatusCode::ACCEPTED => Ok(split::Response::Accepted202),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: Error =
                    serde_json::from_slice(rsp_body).map_err(|source| split::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(split::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod split {
        use crate::{models, models::*};
        #[derive(Debug)]
        pub enum Response {
            Ok200(Vec<ReservationResponse>),
            Accepted202,
        }
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::Error,
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
    pub async fn merge(
        operation_config: &crate::OperationConfig,
        reservation_order_id: &str,
        body: &MergeRequest,
    ) -> std::result::Result<merge::Response, merge::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/providers/Microsoft.Capacity/reservationOrders/{}/merge",
            operation_config.base_path(),
            reservation_order_id
        );
        let mut url = url::Url::parse(url_str).map_err(merge::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(merge::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        req_builder = req_builder.header("content-type", "application/json");
        let req_body = azure_core::to_json(body).map_err(merge::Error::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(merge::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(merge::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: Vec<ReservationResponse> =
                    serde_json::from_slice(rsp_body).map_err(|source| merge::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(merge::Response::Ok200(rsp_value))
            }
            http::StatusCode::ACCEPTED => Ok(merge::Response::Accepted202),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: Error =
                    serde_json::from_slice(rsp_body).map_err(|source| merge::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(merge::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod merge {
        use crate::{models, models::*};
        #[derive(Debug)]
        pub enum Response {
            Ok200(Vec<ReservationResponse>),
            Accepted202,
        }
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::Error,
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
        reservation_order_id: &str,
    ) -> std::result::Result<ReservationList, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/providers/Microsoft.Capacity/reservationOrders/{}/reservations",
            operation_config.base_path(),
            reservation_order_id
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
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(list::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(list::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: ReservationList =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: Error =
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
                value: models::Error,
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
        reservation_id: &str,
        reservation_order_id: &str,
    ) -> std::result::Result<ReservationResponse, get::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/providers/Microsoft.Capacity/reservationOrders/{}/reservations/{}",
            operation_config.base_path(),
            reservation_order_id,
            reservation_id
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
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(get::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(get::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: ReservationResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| get::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: Error =
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
                value: models::Error,
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
        reservation_order_id: &str,
        reservation_id: &str,
        parameters: &Patch,
    ) -> std::result::Result<update::Response, update::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/providers/Microsoft.Capacity/reservationOrders/{}/reservations/{}",
            operation_config.base_path(),
            reservation_order_id,
            reservation_id
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
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        req_builder = req_builder.header("content-type", "application/json");
        let req_body = azure_core::to_json(parameters).map_err(update::Error::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(update::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(update::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: ReservationResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| update::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(update::Response::Ok200(rsp_value))
            }
            http::StatusCode::ACCEPTED => Ok(update::Response::Accepted202),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: Error =
                    serde_json::from_slice(rsp_body).map_err(|source| update::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(update::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod update {
        use crate::{models, models::*};
        #[derive(Debug)]
        pub enum Response {
            Ok200(ReservationResponse),
            Accepted202,
        }
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::Error,
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
    pub async fn list_revisions(
        operation_config: &crate::OperationConfig,
        reservation_id: &str,
        reservation_order_id: &str,
    ) -> std::result::Result<ReservationList, list_revisions::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/providers/Microsoft.Capacity/reservationOrders/{}/reservations/{}/revisions",
            operation_config.base_path(),
            reservation_order_id,
            reservation_id
        );
        let mut url = url::Url::parse(url_str).map_err(list_revisions::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(list_revisions::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(list_revisions::Error::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(list_revisions::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: ReservationList =
                    serde_json::from_slice(rsp_body).map_err(|source| list_revisions::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: Error =
                    serde_json::from_slice(rsp_body).map_err(|source| list_revisions::Error::DeserializeError(source, rsp_body.clone()))?;
                Err(list_revisions::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list_revisions {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::Error,
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
pub mod operation {
    use crate::models::*;
    pub async fn list(operation_config: &crate::OperationConfig) -> std::result::Result<OperationList, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/providers/Microsoft.Capacity/operations", operation_config.base_path(),);
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
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(list::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(list::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: OperationList =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: Error =
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
                value: models::Error,
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
