use super::Endpoint;
use reqwest::Method;

#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "get",  path: "/apps/{app_id}/deployments" } }
#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "get",  path: "/apps/{app_id}/deployments/current" } }
#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "post", path: "/apps/{app_id}/deploy/webhook" } }

impl Endpoint {
    pub(crate) fn list_app_deploys(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/deployments", Method::GET)
            .param("app_id", app_id)
            .build()
    }

    pub(crate) fn get_current_app_deploy(app_id: &str) -> Endpoint {
        Self::builder(
            "/apps/{app_id}/deployments/current",
            Method::GET,
        )
        .param("app_id", app_id)
        .build()
    }

    pub(crate) fn set_webhook_integration(app_id: &str) -> Endpoint {
        Self::builder("/apps/{app_id}/deploy/webhook", Method::POST)
            .param("app_id", app_id)
            .build()
    }
}
