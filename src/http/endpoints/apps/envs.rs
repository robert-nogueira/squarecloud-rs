use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub(crate) fn list_app_envs(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/envs", Method::GET)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn post_app_envs(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/envs", Method::POST)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn overwrite_app_envs(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/envs", Method::PUT)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn delete_app_envs(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/envs", Method::DELETE)
            .param("app_id", app_id)
            .build()
    }
}
