use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Memory quota details for an account plan.
#[derive(Serialize, Deserialize)]
pub struct PlanMemory {
    /// Total RAM limit granted by the plan, in megabytes.
    pub limit: u64,
    /// RAM still available for new allocations, in megabytes.
    pub available: u64,
    /// RAM currently consumed by running resources, in megabytes.
    pub used: u64,
}

/// The active subscription plan for an account.
///
/// Included in [`AccountInfo`](crate::types::AccountInfo).
#[derive(Serialize, Deserialize)]
pub struct Plan {
    /// The plan's human-readable name (e.g. `"Starter"`, `"Pro"`).
    pub name: String,
    /// Memory quota details for this plan.
    pub memory: PlanMemory,
    /// The UTC timestamp when the plan expires or renews.
    #[serde(with = "ts_milliseconds")]
    pub duration: DateTime<Utc>,
}
