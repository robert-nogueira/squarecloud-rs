use chrono::{DateTime, Utc, serde::ts_milliseconds};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum FileType {
    File,
    Directory,
}

#[derive(Serialize, Deserialize)]
pub struct File {
    name: String,
    #[serde(rename = "type")]
    file_type: FileType,
    #[serde(with = "ts_milliseconds")]
    #[serde(rename = "lastModified")]
    last_modified: DateTime<Utc>,
}
