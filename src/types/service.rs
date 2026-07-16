use serde::{Deserialize, Serialize};

/// The operational status of the SquareCloud platform.
///
/// Returned by
/// [`Client::service_status`](crate::Client::service_status).
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceStatus {
    /// A machine-readable status indicator (e.g. `"operational"`).
    pub status: String,
    /// A human-readable description of the current platform state.
    pub message: String,
}
