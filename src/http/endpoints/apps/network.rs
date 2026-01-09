use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub(crate) fn set_app_webhook_integration(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/webhook", Method::POST)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn get_app_dns_record(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/network/dns", Method::GET)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn get_app_analytics(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/network/analytics", Method::GET)
            .param("app_id", app_id)
            .build()
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
}
