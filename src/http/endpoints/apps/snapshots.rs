use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub(crate) fn list_app_snapshots(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/snapshots", Method::GET)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn app_create_snapshot(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/snapshots", Method::POST)
            .param("app_id", app_id)
            .build()
    }
}
