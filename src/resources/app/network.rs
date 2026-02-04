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
}
