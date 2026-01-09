pub mod applications;
pub mod members;

use super::Endpoint;
use reqwest::Method;

impl Endpoint {
    pub(crate) fn create_workspace() -> Endpoint {
        Self::builder("/workspaces", Method::POST).build()
    }

    pub(crate) fn delete_workspace() -> Endpoint {
        Self::builder("/workspaces", Method::DELETE).build()
    }

    pub(crate) fn get_workspace(workspace_id: &str) -> Endpoint {
        Self::builder("/workspaces/{workspace_id}", Method::POST)
            .param("workspace_id", workspace_id)
            .build()
    }

    pub(crate) fn leave_workspace(workspace_id: &str) -> Endpoint {
        Self::builder("/workspaces/{workspace_id}/leave", Method::DELETE)
            .param("workspace_id", workspace_id)
            .build()
    }

    pub(crate) fn list_workspaces(workspace_id: &str) -> Endpoint {
        Self::builder("/workspaces/{workspace_id}", Method::GET)
            .param("workspace_id", workspace_id)
            .build()
    }
}
