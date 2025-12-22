use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct WorkspaceMember {
    id: String,
    name: String,
    group: String,
    #[serde(rename = "joinedAt")]
    joined_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct WorkspaceApp {
    id: String,
    name: String,
    ram: u32,
    language: String,
}

#[derive(Serialize)]
pub struct Workspace {
    id: String,
    name: String,
    owner: String,
    members: Vec<WorkspaceMember>,
    applications: Vec<WorkspaceApp>,
    #[serde(rename = "createdAt")]
    created_at: DateTime<Utc>,
}
