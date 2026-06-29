use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Metadata for a previously taken snapshot.
///
/// Returned in list responses from
/// [`AppResource::list_snapshots`](crate::resources::AppResource::list_snapshots),
/// [`DatabaseResource::list_snapshots`](crate::resources::DatabaseResource::list_snapshots),
/// and [`ApiClient::all_snapshots`](crate::ApiClient::all_snapshots).
#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    /// The snapshot's display name.
    pub name: String,
    /// Size of the snapshot archive in bytes.
    pub size: u64,
    /// The UTC timestamp of the last modification to this snapshot.
    pub modified: DateTime<Utc>,
    /// The storage key used when referencing this snapshot in restore calls.
    pub key: String,
}

/// A reference to a newly created snapshot, including its download location.
///
/// Returned by
/// [`AppResource::create_snapshot`](crate::resources::AppResource::create_snapshot)
/// and
/// [`DatabaseResource::create_snapshot`](crate::resources::DatabaseResource::create_snapshot).
#[derive(Serialize, Deserialize)]
pub struct SnapshotReference {
    /// A pre-signed URL from which the snapshot archive can be downloaded.
    pub url: String,
    /// The storage key that identifies this snapshot in future restore calls.
    pub key: String,
}
