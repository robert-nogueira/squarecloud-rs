use super::Endpoint;
use reqwest::Method;

use crate::types::AnalyticsFilters;

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "get",
    path: "/apps/{app_id}/network/dns",
    domain: "NetworkErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::NetworkErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "get",
    path: "/apps/{app_id}/network/analytics",
    domain: "NetworkErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::NetworkErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "post",
    path: "/apps/{app_id}/network/custom",
    domain: "NetworkErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::NetworkErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "post",
    path: "/apps/{app_id}/network/purge_cache",
    domain: "NetworkErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::NetworkErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "get",
    path: "/apps/{app_id}/network/logs",
    domain: "NetworkErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::NetworkErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "get",
    path: "/apps/{app_id}/network/errors",
    domain: "NetworkErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::NetworkErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "get",
    path: "/apps/{app_id}/network/performance",
    domain: "NetworkErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::NetworkErrorCode>,
    }
}

impl Endpoint {
    pub(crate) fn get_app_dns_record(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/network/dns", Method::GET)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn get_app_analytics(
        app_id: &str,
        start: &str,
        end: &str,
        filters: &AnalyticsFilters,
    ) -> Endpoint {
        let mut b =
            Self::builder("/apps/{app_id}/network/analytics", Method::GET)
                .param("app_id", app_id)
                .query("start", start)
                .query("end", end);
        for (name, value) in filters.entries() {
            if let Some(value) = value {
                b = b.query(name, value);
            }
        }
        b.build()
    }

    pub(crate) fn set_app_custom_domain(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/network/custom", Method::POST)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn purge_edge_cache(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/network/purge_cache", Method::POST)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn network_logs(
        app_id: &str,
        start: &str,
        end: &str,
    ) -> Endpoint {
        Self::builder("/apps/{app_id}/network/logs", Method::GET)
            .param("app_id", app_id)
            .query("start", start)
            .query("end", end)
            .build()
    }

    pub(crate) fn network_errors(
        app_id: &str,
        include_4xx: bool,
        start: &str,
        end: &str,
    ) -> Endpoint {
        let mut b =
            Self::builder("/apps/{app_id}/network/errors", Method::GET)
                .param("app_id", app_id)
                .query("start", start)
                .query("end", end);
        if include_4xx {
            b = b.query("include_4xx", "true");
        }
        b.build()
    }

    pub(crate) fn network_performance(
        app_id: &str,
        start: &str,
        end: &str,
    ) -> Endpoint {
        Self::builder("/apps/{app_id}/network/performance", Method::GET)
            .param("app_id", app_id)
            .query("start", start)
            .query("end", end)
            .build()
    }
}
