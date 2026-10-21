use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::http::errors::SnapshotErrorCode;

/// Filters the resource type returned by
/// [`Client::all_snapshots`](crate::Client::all_snapshots).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotScope {
    /// Return only application snapshots.
    Applications,
    /// Return only database snapshots.
    Databases,
}

impl SnapshotScope {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Applications => "applications",
            Self::Databases => "databases",
        }
    }
}

/// Metadata for a previously taken snapshot.
///
/// Returned in list responses from
/// [`AppResource::list_snapshots`](crate::resources::AppResource::list_snapshots),
/// [`DatabaseResource::list_snapshots`](crate::resources::DatabaseResource::list_snapshots),
/// and [`Client::all_snapshots`](crate::Client::all_snapshots).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// The snapshot's display name.
    pub name: String,
    /// Size of the snapshot archive in bytes.
    pub size: u64,
    /// The UTC timestamp of the last modification to this snapshot.
    pub modified: DateTime<Utc>,
    /// The raw AWS pre-signed storage key for this snapshot.
    pub key: String,
}

impl Snapshot {
    /// Extracts the `versionId` UUID from the storage key.
    ///
    /// Pass the result as the `version_id` argument to
    /// [`AppResource::restore_snapshot`](crate::resources::AppResource::restore_snapshot)
    /// or
    /// [`DatabaseResource::restore_snapshot`](crate::resources::DatabaseResource::restore_snapshot).
    pub fn version_id(&self) -> &str {
        self.key
            .split('&')
            .find(|s| s.starts_with("versionId="))
            .and_then(|s| s.strip_prefix("versionId="))
            .unwrap_or(&self.key)
    }
}

/// A reference to a newly created snapshot, including its download location.
///
/// Returned by
/// [`AppResource::create_snapshot`](crate::resources::AppResource::create_snapshot)
/// and
/// [`DatabaseResource::create_snapshot`](crate::resources::DatabaseResource::create_snapshot).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotReference {
    /// A pre-signed URL from which the snapshot archive can be downloaded.
    pub url: String,
    /// The storage key that identifies this snapshot in future restore calls.
    pub key: String,
}

/// Where the last snapshot restore of an application or database stands.
///
/// Returned by
/// [`AppResource::restore_status`](crate::resources::AppResource::restore_status)
/// and
/// [`DatabaseResource::restore_status`](crate::resources::DatabaseResource::restore_status).
/// The API keeps a finished restore's outcome for one hour.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreStatus {
    /// The restore's current state.
    pub state: RestoreState,
    /// The step running now, for example `"download"` or `"unzip"`.
    /// `"interrupted"` means the node restarted mid-restore.
    #[serde(default)]
    pub step: Option<String>,
    /// Why the restore failed, when `state` is [`RestoreState::Error`].
    #[serde(default)]
    pub code: Option<SnapshotErrorCode>,
    /// When the restore started.
    #[serde(default, with = "chrono::serde::ts_milliseconds_option")]
    pub started_at: Option<DateTime<Utc>>,
    /// When the restore ended, once it succeeded or failed.
    #[serde(default, with = "chrono::serde::ts_milliseconds_option")]
    pub finished_at: Option<DateTime<Utc>>,
}

/// The state of a snapshot restore, as reported by
/// [`RestoreStatus::state`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum RestoreState {
    /// No restore ran on this resource in the last hour.
    None,
    /// The restore is waiting for the node to pick it up.
    Queued,
    /// The restore is running; see [`RestoreStatus::step`].
    Running,
    /// The restore finished successfully.
    Success,
    /// The restore failed; see [`RestoreStatus::code`].
    Error,
    /// A state this client does not recognise yet.
    #[serde(other)]
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::{RestoreState, RestoreStatus};

    #[test]
    fn unrecognised_restore_state_falls_back_to_unknown() {
        let status: RestoreStatus =
            serde_json::from_str(r#"{"state":"paused"}"#)
                .expect("an unknown state should not fail the whole status");
        assert_eq!(status.state, RestoreState::Unknown);
    }
}
