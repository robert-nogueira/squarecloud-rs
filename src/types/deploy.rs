use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Metadata for a single deployment event.
///
/// Returned by
/// [`AppResource::current_deploy`](crate::resources::AppResource::current_deploy)
/// and as part of the list from
/// [`AppResource::list_deploys`](crate::resources::AppResource::list_deploys).
#[derive(Serialize, Deserialize)]
pub struct Deploy {
    /// The deployment's unique identifier.
    pub id: String,
    /// The deployment state (e.g. `"success"`, `"failed"`, `"pending"`).
    pub state: String,
    /// The UTC timestamp when this deployment was recorded.
    pub date: DateTime<Utc>,
}
