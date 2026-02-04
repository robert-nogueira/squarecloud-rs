use chrono::{DateTime, Utc, serde::ts_milliseconds};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FileContent {
    #[serde(rename = "type")]
    pub data_type: String,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub enum FileType {
    File,
    Directory,
}

#[derive(Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub file_type: FileType,
    #[serde(with = "ts_milliseconds")]
    #[serde(rename = "lastModified")]
    pub last_modified: DateTime<Utc>,
}
