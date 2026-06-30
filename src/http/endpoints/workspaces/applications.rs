use super::Endpoint;
use reqwest::Method;

#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "post",   path: "/workspaces/applications" } }
#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "delete", path: "/workspaces/applications" } }

impl Endpoint {
    pub(crate) fn workspace_add_app() -> Endpoint {
        Self::builder("/workspaces/applications", Method::POST).build()
    }

    pub(crate) fn workspace_remove_app() -> Endpoint {
        Self::builder("/workspaces/applications", Method::DELETE).build()
    }
}
