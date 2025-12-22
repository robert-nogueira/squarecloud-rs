use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub fn list_app_snapshots(app_id: &str) -> Endpoint {
        Self::build(
            "/apps/{app_id}/snapshots",
            Method::GET,
            &[("app_id", app_id)],
        )
    }

    pub fn app_create_snapshot(app_id: &str) -> Endpoint {
        Self::build(
            "/apps/{app_id}/snapshots",
            Method::POST,
            &[("app_id", app_id)],
        )
    }
}
