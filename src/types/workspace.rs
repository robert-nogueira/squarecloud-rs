use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{http::ApiClient, resources::WorkspaceResource};

#[derive(Serialize, Deserialize)]
pub struct WorkspaceMember {
    pub id: String,
    pub name: String,
    pub group: String,
    #[serde(rename = "joinedAt")]
    pub joined_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct WorkspaceApp {
    pub id: String,
    pub name: String,
    pub ram: u32,
    pub language: String,
}

#[derive(Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub owner: String,
    #[serde(default)]
    pub members: Vec<WorkspaceMember>,
    #[serde(default)]
    pub applications: Vec<WorkspaceApp>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

impl Workspace {
    pub fn into_resource(&self, api: Arc<ApiClient>) -> WorkspaceResource {
        WorkspaceResource::new(api, &self.id)
    }
}
