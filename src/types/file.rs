use chrono::{DateTime, Utc, serde::ts_milliseconds};
use serde::{Deserialize, Serialize};

/// The raw content of a file read from an application's filesystem.
///
/// Returned by
/// [`FileResource::read`](crate::resources::FileResource::read).
#[derive(Debug, Serialize, Deserialize)]
pub struct FileContent {
    /// The MIME type of the file content (e.g. `"text/plain"`).
    #[serde(rename = "type")]
    pub data_type: String,
    /// The raw file bytes.
    pub data: Vec<u8>,
}

/// Distinguishes a regular file from a directory entry.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    /// A regular file.
    File,
    /// A directory.
    Directory,
}

/// Metadata for a single entry in an application's filesystem directory.
///
/// Returned as part of a [`Vec`] by
/// [`FileResource::all_files`](crate::resources::FileResource::all_files).
#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    /// The entry's name (not the full path).
    pub name: String,
    /// Whether this entry is a regular file or a directory.
    #[serde(rename = "type")]
    pub file_type: FileType,
    /// Size in bytes. `0` for directories.
    pub size: u64,
    /// The UTC timestamp of the last modification.
    #[serde(with = "ts_milliseconds")]
    #[serde(rename = "lastModified")]
    pub last_modified: DateTime<Utc>,
}
