use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Metadata for a previously taken snapshot.
///
/// Returned in list responses from
/// [`AppResource::list_snapshots`](crate::resources::AppResource::list_snapshots),
/// [`DatabaseResource::list_snapshots`](crate::resources::DatabaseResource::list_snapshots),
/// and [`ApiClient::all_snapshots`](crate::ApiClient::all_snapshots).
#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotReference {
    /// A pre-signed URL from which the snapshot archive can be downloaded.
    pub url: String,
    /// The storage key that identifies this snapshot in future restore calls.
    pub key: String,
}
