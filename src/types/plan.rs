use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Memory quota details for an account plan.
#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
    /// The plan's human-readable name (e.g. `"Starter"`, `"Pro"`).
    pub name: String,
    /// Memory quota details for this plan.
    pub memory: PlanMemory,
    /// The UTC timestamp when the plan expires or renews (`null` for free
    /// plans or plans without an explicit expiry).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "chrono::serde::ts_milliseconds_option"
    )]
    pub duration: Option<DateTime<Utc>>,
}
