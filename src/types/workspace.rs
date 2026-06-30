use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{http::ApiClient, resources::WorkspaceResource};

/// A member of a SquareCloud workspace.
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceMember {
    /// The member's account identifier.
    pub id: String,
    /// The member's display name.
    pub name: String,
    /// The permission group assigned to this member (e.g. `"admin"`,
    /// `"member"`).
    pub group: String,
    /// The UTC timestamp when this member joined the workspace.
    #[serde(rename = "joinedAt")]
    pub joined_at: DateTime<Utc>,
}

/// Summary of an application that belongs to a workspace.
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceApp {
    /// The application's unique identifier.
    pub id: String,
    /// The application's display name.
    pub name: String,
    /// RAM allocation in megabytes.
    pub ram: u32,
    /// The programming language or runtime the application uses.
    pub language: String,
}

/// Full details for a SquareCloud workspace.
///
/// Returned by [`ApiClient::all_workspaces`](crate::ApiClient::all_workspaces),
/// [`ApiClient::create_workspace`](crate::ApiClient::create_workspace), and
/// [`WorkspaceResource::info`](crate::resources::WorkspaceResource::info). To
/// obtain a [`WorkspaceResource`] handle from this value, call
/// [`into_resource`](WorkspaceInfo::into_resource).
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceInfo {
    /// The workspace's unique identifier.
    pub id: String,
    /// The workspace's display name.
    pub name: String,
    /// The account identifier of the workspace owner.
    pub owner: String,
    /// All current members of the workspace.
    #[serde(default)]
    pub members: Vec<WorkspaceMember>,
    /// All applications currently assigned to the workspace.
    #[serde(default)]
    pub applications: Vec<WorkspaceApp>,
    /// The UTC timestamp when the workspace was created.
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

impl WorkspaceInfo {
    /// Converts this value into a [`WorkspaceResource`] handle bound to `api`.
    pub fn into_resource(&self, api: Arc<ApiClient>) -> WorkspaceResource {
        WorkspaceResource::new(api, &self.id)
    }
}
