use super::Endpoint;
use reqwest::Method;

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "post",
    path: "/workspaces/applications",
    domain: "WorkspaceErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::WorkspaceErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "delete",
    path: "/workspaces/applications",
    domain: "WorkspaceErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::WorkspaceErrorCode>,
    }
}

impl Endpoint {
    pub(crate) fn workspace_add_app() -> Endpoint {
        Self::builder("/workspaces/applications", Method::POST).build()
    }

    pub(crate) fn workspace_remove_app() -> Endpoint {
        Self::builder("/workspaces/applications", Method::DELETE).build()
    }
}
