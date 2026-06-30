use super::Endpoint;
use reqwest::Method;

#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "get",    path: "/apps/{app_id}/envs" } }
#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "post",   path: "/apps/{app_id}/envs" } }
#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "put",    path: "/apps/{app_id}/envs" } }
#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "delete", path: "/apps/{app_id}/envs" } }

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
