use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub(crate) fn set_app_webhook_integration(app_id: &str) -> Endpoint {
        Self::build(
            "/apps/{app_id}/webhook",
            Method::POST,
            &[("app_id", app_id)],
        )
    }

    pub(crate) fn get_app_dns_record(app_id: &str) -> Endpoint {
        Self::build(
            "/apps/{app_id}/network/dns",
            Method::GET,
            &[("app_id", app_id)],
        )
    }

    pub(crate) fn get_app_analytics(app_id: &str) -> Endpoint {
        Self::build(
            "/apps/{app_id}/network/analytics",
            Method::GET,
            &[("app_id", app_id)],
        )
    }

    pub(crate) fn set_app_custom_domain(app_id: &str) -> Endpoint {
        Self::build(
            "/apps/{app_id}/network/custom",
            Method::POST,
            &[("app_id", app_id)],
        )
    }

    pub(crate) fn purge_edge_cache(app_id: &str) -> Endpoint {
        Self::build(
            "/apps/{app_id}/network/purge_cache",
            Method::POST,
            &[("app_id", app_id)],
        )
    }
}
