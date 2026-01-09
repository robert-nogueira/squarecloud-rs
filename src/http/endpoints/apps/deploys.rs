use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub(crate) fn list_app_deploys(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/deploys", Method::GET)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn get_current_app_deploy(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/deploys/current", Method::GET)
            .param("app_id", app_id)
            .build()
    }
}
