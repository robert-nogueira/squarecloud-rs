pub mod applications;
pub mod members;

use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub(crate) fn create_workspace() -> Endpoint {
        Self::build("/workspaces", Method::POST, &[])
    }

    pub(crate) fn delete_workspace() -> Endpoint {
        Self::build("/workspaces", Method::DELETE, &[])
    }

    pub(crate) fn get_workspace(workspace_id: &str) -> Endpoint {
        Self::build(
            "/workspaces/{workspace_id}",
            Method::POST,
            &[("workspace_id", workspace_id)],
        )
    }

    pub(crate) fn left_workspace(workspace_id: &str) -> Endpoint {
        Self::build(
            "/workspaces/{workspace_id}/leave",
            Method::DELETE,
            &[("workspace_id", workspace_id)],
        )
    }

    pub(crate) fn list_workspaces(workspace_id: &str) -> Endpoint {
        Self::build(
            "/workspaces/{workspace_id}",
            Method::GET,
            &[("workspace_id", workspace_id)],
        )
    }
}
