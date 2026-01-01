use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub(crate) fn get_workspace_invite() -> Endpoint {
        Self::build("/workspaces/members/code", Method::GET, &[])
    }

    pub(crate) fn workspace_invite_member() -> Endpoint {
        Self::build("/workspaces/members", Method::POST, &[])
    }

    pub(crate) fn remove_workspace_member() -> Endpoint {
        Self::build("/workspaces/members", Method::DELETE, &[])
    }

    pub(crate) fn workspace_change_member_permissions() -> Endpoint {
        Self::build("/workspaces/members", Method::PATCH, &[])
    }
}
