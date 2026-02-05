use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
