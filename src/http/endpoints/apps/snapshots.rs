use super::Endpoint;
use reqwest::Method;

#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "get",  path: "/apps/{app_id}/snapshots" } }
#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "post", path: "/apps/{app_id}/snapshots" } }
#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "post", path: "/apps/{app_id}/snapshots/restore" } }

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

    pub(crate) fn restore_app_snapshot(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/snapshots/restore", Method::POST)
            .param("app_id", app_id)
            .build()
    }
}
