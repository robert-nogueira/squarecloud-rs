use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

/// Deserializes a field that the API returns as either a string or a number.
///
/// Used for `cpu`, `ram`, and `storage` in [`RuntimeStats`], which the API
/// returns as formatted strings by default (e.g. `"3.2%"`) but as raw
/// numbers when called with `?rawData=true`.
fn deserialize_as_string<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{Error, Visitor};

    struct V;

    impl<'de> Visitor<'de> for V {
        type Value = String;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "a string or number")
        }

        fn visit_str<E: Error>(self, v: &str) -> Result<String, E> {
            Ok(v.to_owned())
        }

        fn visit_string<E: Error>(self, v: String) -> Result<String, E> {
            Ok(v)
        }

        fn visit_f64<E: Error>(self, v: f64) -> Result<String, E> {
            Ok(v.to_string())
        }

        fn visit_i64<E: Error>(self, v: i64) -> Result<String, E> {
            Ok(v.to_string())
        }

        fn visit_u64<E: Error>(self, v: u64) -> Result<String, E> {
            Ok(v.to_string())
        }
    }

    d.deserialize_any(V)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{AppNetwork, NetworkCounter, RuntimeStats};

    #[test]
    fn network_counter_formatted_variant() {
        let v: NetworkCounter = serde_json::from_value(json!("1 MB")).unwrap();
        assert!(matches!(v, NetworkCounter::Formatted(_)));
    }

    #[test]
    fn network_counter_raw_variant() {
        let v: NetworkCounter =
            serde_json::from_value(json!([1000, 2000])).unwrap();
        assert!(matches!(v, NetworkCounter::Raw(_)));
    }

    #[test]
    fn deserialize_as_string_accepts_string() {
        let stats: RuntimeStats = serde_json::from_value(json!({
            "cpu": "3.2%", "ram": "128/512MB", "status": "running",
            "running": true, "storage": "50MB",
            "network": { "total": "1MB", "now": "0KB" },
            "uptime": null
        }))
        .unwrap();
        assert_eq!(stats.cpu, "3.2%");
    }

    #[test]
    fn deserialize_as_string_accepts_number() {
        let stats: RuntimeStats = serde_json::from_value(json!({
            "cpu": 3.2, "ram": 128, "status": "running",
            "running": true, "storage": 52428800,
            "network": { "total": [1000, 2000], "now": [0, 0] },
            "uptime": null
        }))
        .unwrap();
        assert_eq!(stats.cpu, "3.2");
        assert_eq!(stats.ram, "128");
    }

    #[test]
    fn app_network_accepts_mixed_counter_types() {
        let n: AppNetwork =
            serde_json::from_value(json!({ "total": "1 MB", "now": [0, 0] }))
                .unwrap();
        assert!(matches!(n.total, NetworkCounter::Formatted(_)));
        assert!(matches!(n.now, NetworkCounter::Raw(_)));
    }

    #[test]
    fn deserialize_as_string_accepts_negative_integer() {
        let stats: RuntimeStats = serde_json::from_value(json!({
            "cpu": -1, "ram": -256, "status": "exited",
            "running": false, "storage": -0,
            "network": { "total": "0KB", "now": "0KB" },
            "uptime": null
        }))
        .unwrap();
        assert_eq!(stats.cpu, "-1");
        assert_eq!(stats.ram, "-256");
    }

    #[test]
    fn deserialize_as_string_rejects_invalid_type() {
        let result: Result<RuntimeStats, _> = serde_json::from_value(json!({
            "cpu": true, "ram": "128/512MB", "status": "running",
            "running": true, "storage": "50MB",
            "network": { "total": "1MB", "now": "0KB" },
            "uptime": null
        }));
        assert!(result.is_err());
    }
}

/// A network throughput counter that the API returns as either a formatted
/// string or a raw `[bytes_in, bytes_out]` array when called with
/// `?rawData=true`.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkCounter {
    /// Human-readable summary (e.g. `"1 MB ↑ 500 KB ↓"`).
    Formatted(String),
    /// Raw byte counts as `[bytes_in, bytes_out]`.
    Raw(Vec<u64>),
}

/// Network throughput figures for a running resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct AppNetwork {
    /// Cumulative bytes transferred since the resource started.
    pub total: NetworkCounter,
    /// Bytes transferred in the current measurement interval.
    pub now: NetworkCounter,
}

/// Condensed runtime status for a single entry in the all-apps or
/// all-databases list response.
///
/// Returned as part of a [`Vec`] by
/// [`Client::all_apps_status`](crate::Client::all_apps_status) and
/// [`Client::all_database_status`](crate::Client::all_database_status).
/// `cpu` and `ram` are only present when `running` is `true`.
#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeStatsListItem {
    /// The application or database identifier.
    pub id: String,
    /// Whether the resource is currently running.
    pub running: bool,
    /// CPU usage as a percentage string. Present only when running.
    pub cpu: Option<String>,
    /// RAM usage string. Present only when running.
    pub ram: Option<String>,
}

/// Full runtime status for a single application or database.
///
/// Returned by [`AppResource::status`](crate::resources::AppResource::status)
/// and [`DatabaseResource::status`](crate::resources::DatabaseResource::status).
/// The `cpu`, `ram`, and `storage` fields accept both the formatted string
/// mode (default) and the raw numeric mode (`?rawData=true`).
#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeStats {
    /// CPU usage (e.g. `"3.2%"` or `3.2` in raw mode).
    #[serde(deserialize_with = "deserialize_as_string")]
    pub cpu: String,
    /// RAM usage (e.g. `"128/512MB"` or `128` in raw mode).
    #[serde(deserialize_with = "deserialize_as_string")]
    pub ram: String,
    /// Resource lifecycle state (e.g. `"running"`, `"exited"`).
    pub status: String,
    /// Whether the application process is currently running.
    pub running: bool,
    /// Disk usage (e.g. `"50MB"` or `52428800` in raw mode).
    #[serde(deserialize_with = "deserialize_as_string")]
    pub storage: String,
    /// Network throughput statistics.
    pub network: AppNetwork,
    /// The UTC timestamp when the process last started. `null` when not running.
    #[serde(with = "chrono::serde::ts_milliseconds_option")]
    pub uptime: Option<DateTime<Utc>>,
}
