use super::Endpoint;
use reqwest::Method;

#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "get",    path: "/workspaces/members/code" } }
#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "post",   path: "/workspaces/members" } }
#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "delete", path: "/workspaces/members" } }
#[cfg(feature = "test-utils")]
inventory::submit! { crate::EndpointSpec { method: "patch",  path: "/workspaces/members" } }

impl Endpoint {
    pub(crate) fn get_workspace_invite() -> Endpoint {
        Self::builder("/workspaces/members/code", Method::GET).build()
    }

    pub(crate) fn workspace_invite_member() -> Endpoint {
        Self::builder("/workspaces/members", Method::POST).build()
    }

    pub(crate) fn remove_workspace_member() -> Endpoint {
        Self::builder("/workspaces/members", Method::DELETE).build()
    }

    pub(crate) fn workspace_change_member_permissions() -> Endpoint {
        Self::builder("/workspaces/members", Method::PATCH).build()
    }
}
