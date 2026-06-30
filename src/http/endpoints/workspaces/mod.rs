pub mod applications;
pub mod members;

use super::Endpoint;
use reqwest::Method;

#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "post",   path: "/workspaces" } }
#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "get",    path: "/workspaces" } }
#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "delete", path: "/workspaces" } }
#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "get",    path: "/workspaces/{workspace_id}" } }
#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "delete", path: "/workspaces/leave" } }

impl Endpoint {
    pub(crate) fn create_workspace() -> Endpoint {
        Self::builder("/workspaces", Method::POST).build()
    }

    pub(crate) fn delete_workspace() -> Endpoint {
        Self::builder("/workspaces", Method::DELETE).build()
    }

    pub(crate) fn get_workspace(workspace_id: &str) -> Endpoint {
        Self::builder("/workspaces/{workspace_id}", Method::GET)
            .param("workspace_id", workspace_id)
            .build()
    }

    pub(crate) fn leave_workspace() -> Endpoint {
        Self::builder("/workspaces/leave", Method::DELETE).build()
    }

    pub(crate) fn list_workspaces() -> Endpoint {
        Self::builder("/workspaces", Method::GET).build()
    }
}
