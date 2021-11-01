#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2021-08-27")]
pub mod package_2021_08_27;
#[cfg(all(feature = "package-2021-08-27", not(feature = "no-default-version")))]
pub use package_2021_08_27::{models, operations};
#[cfg(feature = "package-2021-01")]
pub mod package_2021_01;
#[cfg(all(feature = "package-2021-01", not(feature = "no-default-version")))]
pub use package_2021_01::{models, operations};
#[cfg(feature = "package-2020-09-18")]
pub mod package_2020_09_18;
#[cfg(all(feature = "package-2020-09-18", not(feature = "no-default-version")))]
pub use package_2020_09_18::{models, operations};
#[cfg(feature = "package-2020-06-14")]
pub mod package_2020_06_14;
#[cfg(all(feature = "package-2020-06-14", not(feature = "no-default-version")))]
pub use package_2020_06_14::{models, operations};
#[cfg(feature = "package-2020-02-15")]
pub mod package_2020_02_15;
#[cfg(all(feature = "package-2020-02-15", not(feature = "no-default-version")))]
pub use package_2020_02_15::{models, operations};
#[cfg(feature = "package-2019-11-09")]
pub mod package_2019_11_09;
#[cfg(all(feature = "package-2019-11-09", not(feature = "no-default-version")))]
pub use package_2019_11_09::{models, operations};
#[cfg(feature = "package-2019-09-07")]
pub mod package_2019_09_07;
#[cfg(all(feature = "package-2019-09-07", not(feature = "no-default-version")))]
pub use package_2019_09_07::{models, operations};
#[cfg(feature = "package-2019-05-15")]
pub mod package_2019_05_15;
#[cfg(all(feature = "package-2019-05-15", not(feature = "no-default-version")))]
pub use package_2019_05_15::{models, operations};
#[cfg(feature = "package-2019-01-21")]
pub mod package_2019_01_21;
#[cfg(all(feature = "package-2019-01-21", not(feature = "no-default-version")))]
pub use package_2019_01_21::{models, operations};
#[cfg(feature = "package-2018-09-07-preview")]
pub mod package_2018_09_07_preview;
#[cfg(all(feature = "package-2018-09-07-preview", not(feature = "no-default-version")))]
pub use package_2018_09_07_preview::{models, operations};
#[cfg(feature = "package-2017-09-07-privatepreview")]
pub mod package_2017_09_07_privatepreview;
#[cfg(all(feature = "package-2017-09-07-privatepreview", not(feature = "no-default-version")))]
pub use package_2017_09_07_privatepreview::{models, operations};
#[cfg(feature = "schema-2019-09-07")]
pub mod schema_2019_09_07;
#[cfg(all(feature = "schema-2019-09-07", not(feature = "no-default-version")))]
pub use schema_2019_09_07::{models, operations};
#[cfg(feature = "schema-2019-05-15")]
pub mod schema_2019_05_15;
#[cfg(all(feature = "schema-2019-05-15", not(feature = "no-default-version")))]
pub use schema_2019_05_15::{models, operations};
#[cfg(feature = "schema-2019-01-21")]
pub mod schema_2019_01_21;
#[cfg(all(feature = "schema-2019-01-21", not(feature = "no-default-version")))]
pub use schema_2019_01_21::{models, operations};
#[cfg(feature = "schema-2018-09-07-preview")]
pub mod schema_2018_09_07_preview;
#[cfg(all(feature = "schema-2018-09-07-preview", not(feature = "no-default-version")))]
pub use schema_2018_09_07_preview::{models, operations};
#[cfg(feature = "schema-2017-09-07-privatepreview")]
pub mod schema_2017_09_07_privatepreview;
use azure_core::setters;
#[cfg(all(feature = "schema-2017-09-07-privatepreview", not(feature = "no-default-version")))]
pub use schema_2017_09_07_privatepreview::{models, operations};
pub fn config(
    http_client: std::sync::Arc<dyn azure_core::HttpClient>,
    token_credential: Box<dyn azure_core::TokenCredential>,
) -> OperationConfigBuilder {
    OperationConfigBuilder {
        http_client,
        base_path: None,
        token_credential,
        token_credential_resource: None,
    }
}
pub struct OperationConfigBuilder {
    http_client: std::sync::Arc<dyn azure_core::HttpClient>,
    base_path: Option<String>,
    token_credential: Box<dyn azure_core::TokenCredential>,
    token_credential_resource: Option<String>,
}
impl OperationConfigBuilder {
    setters! { base_path : String => Some (base_path) , token_credential_resource : String => Some (token_credential_resource) , }
    pub fn build(self) -> OperationConfig {
        OperationConfig {
            http_client: self.http_client,
            base_path: self.base_path.unwrap_or_else(|| "https://management.azure.com".to_owned()),
            token_credential: Some(self.token_credential),
            token_credential_resource: self
                .token_credential_resource
                .unwrap_or_else(|| "https://management.azure.com/".to_owned()),
        }
    }
}
pub struct OperationConfig {
    http_client: std::sync::Arc<dyn azure_core::HttpClient>,
    base_path: String,
    token_credential: Option<Box<dyn azure_core::TokenCredential>>,
    token_credential_resource: String,
}
impl OperationConfig {
    pub fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.http_client.as_ref()
    }
    pub fn base_path(&self) -> &str {
        self.base_path.as_str()
    }
    pub fn token_credential(&self) -> Option<&dyn azure_core::TokenCredential> {
        self.token_credential.as_deref()
    }
    pub fn token_credential_resource(&self) -> &str {
        self.token_credential_resource.as_str()
    }
}
