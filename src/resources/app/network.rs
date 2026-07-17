use chrono::{DateTime, SecondsFormat, Utc};
use serde_json::json;

use crate::{
    Endpoint,
    http::errors::{ApiError, NetworkErrorCode},
    types::{
        Analytics, AnalyticsFilters, DnsRecord, NetworkErrors,
        NetworkLogEntry, NetworkPerformance,
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
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// on an API-level error.
    pub async fn analytics(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Analytics, ApiError<NetworkErrorCode>> {
        self.analytics_filtered(start, end, AnalyticsFilters::default())
            .await
    }

    /// Returns edge-network analytics restricted by drill-down filters.
    ///
    /// Every filter set on [`AnalyticsFilters`] is applied to all
    /// breakdowns at once (e.g. filtering by country restricts the paths,
    /// browsers and IPs tables to that country's traffic).
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Service`] with
    /// [`NetworkErrorCode::InvalidFilter`] if a filter value does not
    /// match its expected format, or [`ApiError::Transport`] on network
    /// failure.
    pub async fn analytics_filtered(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        filters: AnalyticsFilters,
    ) -> Result<Analytics, ApiError<NetworkErrorCode>> {
        self.client
            .request_endpoint(Endpoint::get_app_analytics(
                &self.id,
                &start.to_rfc3339_opts(SecondsFormat::Secs, true),
                &end.to_rfc3339_opts(SecondsFormat::Secs, true),
                &filters,
            ))
            .await?
            .into_result_t()
    }

    /// Returns the DNS records the caller must configure at their domain
    /// registrar so the edge provider can validate ownership and issue
    /// SSL for the attached custom domain.
    ///
    /// Typically one or two ownership-validation `txt` records plus the
    /// routing `cname` record pointing at `cname.squareweb.app`. Returns
    /// an empty `Vec` when the custom hostname is not registered on the
    /// edge yet. Each record's `status` reflects the edge provider's
    /// validation state (e.g. `"pending"`, `"pending_validation"`,
    /// `"active"`). Cached for 30 seconds per (owner, application).
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// on an API-level error.
    pub async fn dns_records(
        &self,
    ) -> Result<Vec<DnsRecord>, ApiError<NetworkErrorCode>> {
        self.client
            .request_endpoint(Endpoint::get_app_dns_records(&self.id))
            .await?
            .into_result_t()
    }

    /// Associates a custom domain with the application.
    ///
    /// `custom` must be a fully-qualified domain name that the caller has
    /// pointed at the records returned by
    /// [`dns_records`](AppResource::dns_records). Returns `Ok(true)` when the
    /// domain is registered.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Service`] with
    /// [`NetworkErrorCode::InvalidDomain`]
    /// if the domain is malformed or already in use, or
    /// [`ApiError::Transport`] on network failure.
    pub async fn set_custom_domain(
        &self,
        custom: &str,
    ) -> Result<bool, ApiError<NetworkErrorCode>> {
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
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// on an API-level error.
    pub async fn network_errors(
        &self,
        include_4xx: bool,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<NetworkErrors, ApiError<NetworkErrorCode>> {
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
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// on an API-level error.
    pub async fn network_logs(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<NetworkLogEntry>, ApiError<NetworkErrorCode>> {
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
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// on an API-level error.
    pub async fn network_performance(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<NetworkPerformance, ApiError<NetworkErrorCode>> {
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
    /// Returns [`ApiError::Transport`] on network failure or [`ApiError::Service`]
    /// on an API-level error.
    pub async fn purge_cache(
        &self,
    ) -> Result<bool, ApiError<NetworkErrorCode>> {
        self.client
            .request_endpoint::<()>(Endpoint::purge_edge_cache(&self.id))
            .await?
            .into_bool_result()
    }
}
