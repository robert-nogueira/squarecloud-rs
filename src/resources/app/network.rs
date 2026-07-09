use chrono::{DateTime, SecondsFormat, Utc};
use serde_json::json;

use crate::{
    Endpoint,
    http::errors::ApiError,
    types::{
        Analytics, DnsRecord, NetworkErrors, NetworkLogEntry,
        NetworkPerformance,
    },
};

use super::AppResource;

impl AppResource {
    /// Returns edge-network analytics for the application over a time window.
    ///
    /// The maximum retention window is 7 days. Returns an [`Analytics`] with
    /// empty vectors when the requested window precedes the application's
    /// creation date.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn analytics(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Analytics, ApiError> {
        self.client
            .request_endpoint(Endpoint::get_app_analytics(
                &self.id,
                &start.to_rfc3339_opts(SecondsFormat::Secs, true),
                &end.to_rfc3339_opts(SecondsFormat::Secs, true),
            ))
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
            .request_builder(&self.client.http_client, &self.client.base_url)
            .json(&json!({"custom": custom}))
            .build()?;
        self.client
            .execute_request::<()>(request)
            .await?
            .into_bool_result()
    }

    /// Returns edge-network error analytics for the application.
    ///
    /// The [`NetworkErrors`] value includes aggregate totals, a per-status
    /// breakdown, time-bucketed timeseries data, and the most error-prone
    /// paths. Set `include_4xx` to `true` to include 4xx client errors in
    /// the response (default: 5xx only).
    ///
    /// Requires a Pro or Enterprise plan.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn network_errors(
        &self,
        include_4xx: bool,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<NetworkErrors, ApiError> {
        self.client
            .request_endpoint(Endpoint::network_errors(
                &self.id,
                include_4xx,
                &start.to_rfc3339_opts(SecondsFormat::Secs, true),
                &end.to_rfc3339_opts(SecondsFormat::Secs, true),
            ))
            .await?
            .into_result_t()
    }

    /// Returns edge-network request logs for the application.
    ///
    /// Each [`NetworkLogEntry`] contains the timestamp, client information,
    /// request details, and response metadata for a single edge request.
    ///
    /// The API retains logs for a maximum of 7 days.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn network_logs(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<NetworkLogEntry>, ApiError> {
        self.client
            .request_endpoint(Endpoint::network_logs(
                &self.id,
                &start.to_rfc3339_opts(SecondsFormat::Secs, true),
                &end.to_rfc3339_opts(SecondsFormat::Secs, true),
            ))
            .await?
            .into_result_t()
    }

    /// Returns edge-network latency performance analytics for the application.
    ///
    /// The [`NetworkPerformance`] value includes aggregate latency
    /// percentiles (p50/p95/p99), time-bucketed timeseries, and breakdowns
    /// by country, datacenter, and the slowest request paths.
    ///
    /// Maximum window is 7 days.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Api`]
    /// on an API-level error.
    pub async fn network_performance(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<NetworkPerformance, ApiError> {
        self.client
            .request_endpoint(Endpoint::network_performance(
                &self.id,
                &start.to_rfc3339_opts(SecondsFormat::Secs, true),
                &end.to_rfc3339_opts(SecondsFormat::Secs, true),
            ))
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
