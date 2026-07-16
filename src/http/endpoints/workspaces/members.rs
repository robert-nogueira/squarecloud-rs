use super::Endpoint;
use reqwest::Method;

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "get",
    path: "/workspaces/members/code",
    domain: "MemberErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::MemberErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "post",
    path: "/workspaces/members",
    domain: "MemberErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::MemberErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "delete",
    path: "/workspaces/members",
    domain: "MemberErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::MemberErrorCode>,
    }
}

#[cfg(feature = "test-utils")]
inventory::submit! {
    crate::EndpointSpec {
    method: "patch",
    path: "/workspaces/members",
    domain: "MemberErrorCode",
    known_code: crate::errors::code_is_known::<crate::errors::MemberErrorCode>,
    }
}

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
