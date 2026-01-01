use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub fn list_app_envs(app_id: &str) -> Endpoint {
        Self::build("/apps/{app_id}/envs", Method::GET, &[("app_id", app_id)])
    }

    pub fn post_app_envs(app_id: &str) -> Endpoint {
        Self::build("/apps/{app_id}/envs", Method::POST, &[("app_id", app_id)])
    }

    pub fn overwrite_app_envs(app_id: &str) -> Endpoint {
        Self::build("/apps/{app_id}/envs", Method::PUT, &[("app_id", app_id)])
    }

    pub fn delete_app_envs(app_id: &str) -> Endpoint {
        Self::build(
            "/apps/{app_id}/envs",
            Method::DELETE,
            &[("app_id", app_id)],
        )
    }
}
