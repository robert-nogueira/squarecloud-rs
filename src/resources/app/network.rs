use serde_json::json;

use crate::{
    Endpoint,
    http::errors::ApiError,
    types::{Analytics, DnsRecord, NetworkLogEntry},
};

use super::AppResource;

impl AppResource {
    /// Returns edge-network analytics for the application.
    ///
    /// The [`Analytics`] value breaks down visits, requests, data transferred,
    /// and origin metadata (countries, devices, browsers, etc.) across recent
    /// traffic.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn analytics(&self) -> Result<Analytics, ApiError> {
        self.client
            .request_endpoint(Endpoint::get_app_analytics(&self.id))
            .await?
            .into_result_t()
    }

    /// Returns the DNS record the application's domain is expected to point
    /// to.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn dns_record(&self) -> Result<DnsRecord, ApiError> {
        self.client
            .request_endpoint(Endpoint::get_app_dns_record(&self.id))
            .await?
            .into_result_t()
    }

    /// Associates a custom domain with the application.
    ///
    /// `custom` must be a fully-qualified domain name that the caller has
    /// pointed at the address returned by
    /// [`dns_record`](AppResource::dns_record). Returns `Ok(true)` when the
    /// domain is registered.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Api`] with
    /// [`ApiErrorCode::InvalidSubdomain`](crate::ApiErrorCode::InvalidSubdomain)
    /// if the domain is malformed or already in use, or
    /// [`ApiError::Transport`] on network failure.
    pub async fn set_custom_domain(
        &self,
        custom: &str,
    ) -> Result<bool, ApiError> {
        let endpoint = Endpoint::set_app_custom_domain(&self.id);
        let request = endpoint
            .request_builder(&self.client.http_client)
            .json(&json!({"custom": custom}))
            .build()?;
        self.client
            .execute_request::<()>(request)
            .await?
            .into_bool_result()
    }

    /// Returns edge-network request logs for the application.
    ///
    /// Each [`NetworkLogEntry`] contains the timestamp, client information,
    /// request details, and response metadata for a single edge request.
    ///
    /// Requires a Pro or Enterprise plan. The API retains logs for a maximum
    /// of 7 days.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn network_logs(
        &self,
    ) -> Result<Vec<NetworkLogEntry>, ApiError> {
        self.client
            .request_endpoint(Endpoint::network_logs(&self.id))
            .await?
            .into_result_t()
    }

    /// Purges the edge cache for the application.
    ///
    /// Returns `Ok(true)` when the purge has been queued.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn purge_cache(&self) -> Result<bool, ApiError> {
        self.client
            .request_endpoint::<()>(Endpoint::purge_edge_cache(&self.id))
            .await?
            .into_bool_result()
    }
}
