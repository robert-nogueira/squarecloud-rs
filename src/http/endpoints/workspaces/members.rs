use super::Endpoint;
use reqwest::Method;

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
