use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct WorkspaceMember {
    pub id: String,
    pub name: String,
    pub group: String,
    #[serde(rename = "joinedAt")]
    pub joined_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct WorkspaceApp {
    pub id: String,
    pub name: String,
    pub ram: u32,
    pub language: String,
}

#[derive(Serialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub members: Vec<WorkspaceMember>,
    pub applications: Vec<WorkspaceApp>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}
