use serde_json::json;

use crate::{
    Endpoint,
    http::errors::ApiError,
    types::{Analytics, DnsRecord},
};

use super::AppResource;

impl AppResource {
    pub async fn analytics(&self) -> Result<Analytics, ApiError> {
        self.api
            .request_endpoint(Endpoint::get_app_analytics(&self.id))
            .await?
            .into_result_t()
    }

    pub async fn dns_record(&self) -> Result<DnsRecord, ApiError> {
        self.api
            .request_endpoint(Endpoint::get_app_dns_record(&self.id))
            .await?
            .into_result_t()
    }

    pub async fn set_custom_domain(
        &self,
        custom: &str,
    ) -> Result<bool, ApiError> {
        let endpoint = Endpoint::set_app_custom_domain(&self.id);
        let request = endpoint
            .request_builder(&self.api.http_client)
            .json(&json!({"custom": custom}))
            .build()?;
        self.api
            .execute_request::<()>(request)
            .await?
            .into_bool_result()
    }
}
