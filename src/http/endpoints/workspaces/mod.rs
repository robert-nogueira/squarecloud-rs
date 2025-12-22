pub mod applications;
pub mod members;

use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub fn create_workspace() -> Endpoint {
        Self::build("/workspaces", Method::POST, &[])
    }

    pub fn delete_workspace() -> Endpoint {
        Self::build("/workspaces", Method::DELETE, &[])
    }

    pub fn get_workspace(workspace_id: &str) -> Endpoint {
        Self::build(
            "/workspaces/{workspace_id}",
            Method::POST,
            &[("workspace_id", workspace_id)],
        )
    }

    pub fn left_workspace(workspace_id: &str) -> Endpoint {
        Self::build(
            "/workspaces/{workspace_id}/leave",
            Method::DELETE,
            &[("workspace_id", workspace_id)],
        )
    }

    pub fn list_workspaces(workspace_id: &str) -> Endpoint {
        Self::build(
            "/workspaces/{workspace_id}",
            Method::GET,
            &[("workspace_id", workspace_id)],
        )
    }
}
