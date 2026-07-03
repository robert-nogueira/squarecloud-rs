use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// How a deployment was triggered.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeploySource {
    /// Triggered by a Git webhook or GitHub App push.
    Git,
    /// Triggered by a manual ZIP upload.
    Upload,
}

/// Files changed in a `commit`-state deployment event.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeployFiles {
    /// Files added in this deploy.
    pub added: Vec<String>,
    /// Files removed in this deploy.
    pub removed: Vec<String>,
    /// Files modified in this deploy.
    pub modified: Vec<String>,
}

/// Metadata for a single deployment event.
///
/// Returned by
/// [`AppResource::current_deploy`](crate::resources::AppResource::current_deploy)
/// and as part of the list from
/// [`AppResource::list_deploys`](crate::resources::AppResource::list_deploys).
#[derive(Debug, Serialize, Deserialize)]
pub struct Deploy {
    /// The deployment's unique identifier.
    pub id: String,
    /// The deployment lifecycle stage.
    pub state: String,
    /// The UTC timestamp when this deployment was recorded.
    pub date: DateTime<Utc>,
    /// How the deployment was triggered.
    pub source: DeploySource,
    /// The Git branch checked out. Present only on `clone`-state events.
    pub branch: Option<String>,
    /// Files changed. Present only on `commit`-state events.
    pub files: Option<DeployFiles>,
}
