use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub(crate) fn list_app_deploys(app_id: &str) -> Endpoint {
        Self::build(
            "/apps/{app_id}/deploys",
            Method::GET,
            &[("app_id", app_id)],
        )
    }

    pub(crate) fn get_current_app_deploy(app_id: &str) -> Endpoint {
        Self::build(
            "/apps/{app_id}/deploys/current",
            Method::GET,
            &[("app_id", app_id)],
        )
    }
}
