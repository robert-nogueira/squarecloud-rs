use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize};

/// The raw content of a file read from an application's filesystem.
///
/// Returned by
/// [`FileResource::read`](crate::resources::FileResource::read).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContent {
    /// The encoding the API used on the wire (`"base64"`). It was never a
    /// MIME type.
    #[deprecated(
        since = "0.5.0",
        note = "only names the wire encoding; read the bytes from `data`"
    )]
    #[serde(rename = "encoding", alias = "type", default)]
    pub data_type: String,
    /// The raw file bytes.
    #[serde(with = "base64_bytes")]
    pub data: Vec<u8>,
}

mod base64_bytes {
    use base64::{Engine, engine::general_purpose::STANDARD};
    use serde::{Deserialize, Deserializer, Serializer, de::Error};

    pub fn serialize<S: Serializer>(
        bytes: &[u8],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&STANDARD.encode(bytes))
    }

    /// Also accepts the deprecated byte array, the shape the API sends
    /// without `encoding=base64` and that 0.4 serialized.
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Wire {
        Base64(String),
        Bytes(Vec<u8>),
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<u8>, D::Error> {
        match Wire::deserialize(deserializer)? {
            Wire::Base64(encoded) => {
                STANDARD.decode(encoded).map_err(D::Error::custom)
            }
            Wire::Bytes(bytes) => Ok(bytes),
        }
    }
}

/// Distinguishes a regular file from a directory entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

    use super::{FileContent, FileInfo, FileType};

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
            "lastModified": 1_783_269_964_121.640_9_f64
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

    #[test]
    #[allow(deprecated)]
    fn content_accepts_the_legacy_buffer_shape() {
        let content: FileContent =
            serde_json::from_str(r#"{"type":"Buffer","data":[104,105]}"#)
                .expect("the 0.4 shape still deserializes");
        assert_eq!(content.data, b"hi");
        assert_eq!(content.data_type, "Buffer");
    }

    #[test]
    #[allow(deprecated)]
    fn content_without_encoding_field_deserializes() {
        let content: FileContent =
            serde_json::from_str(r#"{"data":"aGk="}"#).expect("deserializes");
        assert_eq!(content.data, b"hi");
        assert!(content.data_type.is_empty());
    }

    #[test]
    fn content_roundtrips_through_base64() {
        let content: FileContent =
            serde_json::from_str(r#"{"encoding":"base64","data":"AP8QgA=="}"#)
                .expect("deserializes");
        assert_eq!(content.data, [0x00, 0xff, 0x10, 0x80]);

        let json = serde_json::to_value(&content).expect("serializes");
        assert_eq!(json["data"], "AP8QgA==");
        assert_eq!(json["encoding"], "base64");
    }
}
