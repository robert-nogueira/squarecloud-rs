use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize};

/// Deserializes a millisecond timestamp that the API may send as either an
/// integer or a float (e.g. `1783269964121.6409`).
fn deserialize_ms_option<'de, D>(
    d: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{Error, Visitor};

    struct V;

    impl<'de> Visitor<'de> for V {
        type Value = Option<DateTime<Utc>>;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "a unix timestamp in milliseconds or null")
        }

        fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
            Ok(Utc.timestamp_millis_opt(v).single())
        }

        fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
            Ok(Utc.timestamp_millis_opt(v as i64).single())
        }

        fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> {
            Ok(Utc.timestamp_millis_opt(v as i64).single())
        }

        fn visit_some<D2: Deserializer<'de>>(
            self,
            d2: D2,
        ) -> Result<Self::Value, D2::Error> {
            d2.deserialize_any(V)
        }
    }

    d.deserialize_option(V)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{FileInfo, FileType};

    #[test]
    fn file_type_deserializes_file_and_directory() {
        let f: FileType = serde_json::from_str(r#""file""#).unwrap();
        assert!(matches!(f, FileType::File));
        let d: FileType = serde_json::from_str(r#""directory""#).unwrap();
        assert!(matches!(d, FileType::Directory));
    }

    fn parse(json: serde_json::Value) -> FileInfo {
        serde_json::from_value(json).unwrap()
    }

    #[test]
    fn last_modified_null_becomes_none() {
        let fi = parse(json!({
            "name": "main.py", "type": "file", "size": 0,
            "lastModified": null
        }));
        assert!(fi.last_modified.is_none());
    }

    #[test]
    fn last_modified_absent_becomes_none() {
        let fi = parse(json!({ "name": "main.py", "type": "file" }));
        assert!(fi.last_modified.is_none());
    }

    #[test]
    fn last_modified_integer_ms() {
        let fi = parse(json!({
            "name": "f", "type": "file",
            "lastModified": 1_700_000_000_000_i64
        }));
        assert!(fi.last_modified.is_some());
    }

    #[test]
    fn last_modified_float_ms() {
        let fi = parse(json!({
            "name": "f", "type": "file",
            "lastModified": 1_783_269_964_121.6409_f64
        }));
        assert!(fi.last_modified.is_some());
    }

    #[test]
    fn last_modified_negative_ms_is_pre_epoch() {
        let fi = parse(json!({
            "name": "f", "type": "file",
            "lastModified": -1000_i64
        }));
        assert!(fi.last_modified.is_some());
    }

    #[test]
    fn last_modified_wrong_type_returns_error() {
        let result: Result<FileInfo, _> = serde_json::from_value(json!({
            "name": "f", "type": "file",
            "lastModified": "not-a-timestamp"
        }));
        assert!(result.is_err());
    }
}

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
    /// Size in bytes. `0` for directories or when absent in the response.
    #[serde(default)]
    pub size: u64,
    /// The UTC timestamp of the last modification. The API may return this as
    /// an integer or float millisecond timestamp, or omit it entirely.
    #[serde(
        deserialize_with = "deserialize_ms_option",
        rename = "lastModified",
        default
    )]
    pub last_modified: Option<DateTime<Utc>>,
}
