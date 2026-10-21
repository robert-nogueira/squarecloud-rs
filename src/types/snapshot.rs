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
    /// The snapshot's identifier: the resource id, optionally suffixed
    /// with `_<runtime>` and `_<origin>`. Pass it unchanged as
    /// `snapshot_id` to the restore methods.
    pub name: String,
    /// Size of the snapshot archive in bytes.
    pub size: u64,
    /// The UTC timestamp of the last modification to this snapshot.
    pub modified: DateTime<Utc>,
    /// The signed query string used to compose the download URL.
    pub key: String,
    /// The snapshot's storage version. Pass it as `version_id` to
    /// [`AppResource::restore_snapshot`](crate::resources::AppResource::restore_snapshot)
    /// or
    /// [`DatabaseResource::restore_snapshot`](crate::resources::DatabaseResource::restore_snapshot).
    pub version_id: String,
    /// Signed download URL, valid for 30 days.
    pub url: String,
    /// What the archive holds: the application's language or the
    /// database engine. `None` on snapshots taken before 2026-09-17.
    #[serde(default)]
    pub runtime: Option<String>,
    /// Whether the snapshot was scheduled or requested. `None` on
    /// snapshots taken before 2026-09-17.
    #[serde(default)]
    pub origin: Option<SnapshotOrigin>,
}

impl Snapshot {
    /// Returns the snapshot's storage version.
    #[deprecated(since = "0.5.0", note = "read the `version_id` field")]
    pub fn version_id(&self) -> &str {
        &self.version_id
    }
}

/// How a snapshot came to exist, as reported by [`Snapshot::origin`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum SnapshotOrigin {
    /// Taken by the daily schedule.
    Automatic,
    /// Requested through the API, the dashboard, the CLI or the AI agent.
    Manual,
    /// An origin this client does not recognise yet.
    #[serde(other)]
    Unknown,
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
    /// The signed query string of the download URL. It is not the
    /// `version_id` the restore methods take; read that from
    /// [`Snapshot::version_id`](field@Snapshot::version_id) in the listing.
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
    use super::{RestoreState, RestoreStatus, Snapshot};

    #[test]
    fn unrecognised_restore_state_falls_back_to_unknown() {
        let status: RestoreStatus =
            serde_json::from_str(r#"{"state":"paused"}"#)
                .expect("an unknown state should not fail the whole status");
        assert_eq!(status.state, RestoreState::Unknown);
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_version_id_method_returns_the_field() {
        // The key carries no versionId, so only the field can supply it.
        let snapshot: Snapshot = serde_json::from_value(serde_json::json!({
            "name": "app-123",
            "size": 1,
            "modified": "2026-10-01T00:00:00Z",
            "key": "Expires=9999&Signature=sig",
            "version_id": "3HL4kqtJlcpX",
            "url": "https://snapshots.squarecloud.app/app-123.zip"
        }))
        .expect("deserializes");
        assert_eq!(snapshot.version_id(), "3HL4kqtJlcpX");
    }
}
