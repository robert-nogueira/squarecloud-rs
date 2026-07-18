use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};

/// An event received from the application's realtime log stream.
///
/// Returned by
/// [`AppResource::realtime`](crate::resources::AppResource::realtime).
#[derive(Debug, Clone)]
pub enum RealtimeEvent {
    /// A log line emitted by the application.
    Log {
        /// Which output stream the line came from.
        stream: LogStream,
        /// The log line's text, with the stream-indicator byte already
        /// stripped.
        line: String,
    },
    /// A lifecycle or keepalive message from the SSE server
    /// (e.g. `REALTIME_CONNECTED`, `PING`).
    System(String),
    /// Live container metrics, merged from the server's complete and lean
    /// frames. See [`RealtimeStatus`] for how the merge works.
    Status(RealtimeStatus),
}

/// Which output stream a [`RealtimeEvent::Log`] line came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogStream {
    /// Standard output.
    Stdout,
    /// Standard error.
    Stderr,
}

/// Live container metrics from a [`RealtimeEvent::Status`] event.
///
/// The server sends one COMPLETE frame right after connecting, then lean
/// frames roughly once a second carrying only [`cpu`](Self::cpu),
/// [`ram`](Self::ram), [`net_io`](Self::net_io), and
/// [`block_io`](Self::block_io).
/// [`AppResource::realtime`](crate::resources::AppResource::realtime) merges
/// each lean frame onto the last known complete frame internally, so every
/// `RealtimeStatus` you receive already has every field filled in with the
/// most recently seen value.
#[derive(Debug, Clone, PartialEq)]
pub struct RealtimeStatus {
    /// CPU usage as a percentage (e.g. `12.5` for 12.5%). Can exceed 100 on
    /// multi-core allocations.
    pub cpu: f64,
    /// The CPU percentage ceiling for this application's plan allocation.
    /// `None` only if no complete frame has been received yet, which should
    /// not happen in practice: the server always sends one right after
    /// connecting.
    pub cpu_limit: Option<f64>,
    /// RAM usage.
    pub ram: RealtimeRam,
    /// Container lifecycle status (e.g. `"running"`). `None` only if no
    /// complete frame has been received yet (see
    /// [`cpu_limit`](Self::cpu_limit)).
    pub status: Option<String>,
    /// Cumulative and instantaneous network throughput.
    pub net_io: RealtimeNetIo,
    /// Cumulative disk I/O.
    pub block_io: RealtimeBlockIo,
    /// When the container process started. `None` only if no complete frame
    /// has been received yet (see [`cpu_limit`](Self::cpu_limit)).
    pub uptime: Option<DateTime<Utc>>,
}

/// RAM usage in megabytes, from a [`RealtimeStatus`] frame.
///
/// Both fields are fractional on the wire (e.g. `13.83`), not whole
/// megabytes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RealtimeRam {
    /// Megabytes currently in use.
    pub used_mb: f64,
    /// The application's RAM allocation ceiling, in megabytes.
    pub limit_mb: f64,
}

impl<'de> Deserialize<'de> for RealtimeRam {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let [used_mb, limit_mb] = <[f64; 2]>::deserialize(deserializer)?;
        Ok(Self { used_mb, limit_mb })
    }
}

/// Network throughput for a [`RealtimeStatus`] frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct RealtimeNetIo {
    /// Cumulative bytes received since the container started.
    #[serde(rename = "i")]
    pub bytes_in: u64,
    /// Cumulative bytes sent since the container started.
    #[serde(rename = "o")]
    pub bytes_out: u64,
    /// Instantaneous throughput, in bytes per second, at the time of this
    /// frame.
    pub new: RealtimeNetRate,
}

/// Instantaneous network throughput, in bytes per second.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct RealtimeNetRate {
    /// Bytes received per second.
    #[serde(rename = "i")]
    pub bytes_in: u64,
    /// Bytes sent per second.
    #[serde(rename = "o")]
    pub bytes_out: u64,
}

/// Cumulative disk I/O for a [`RealtimeStatus`] frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct RealtimeBlockIo {
    /// Cumulative bytes read from disk since the container started.
    #[serde(rename = "i")]
    pub bytes_read: u64,
    /// Cumulative bytes written to disk since the container started.
    #[serde(rename = "o")]
    pub bytes_written: u64,
}

/// Raw shape of a single `event: status` SSE frame, before merging.
///
/// `cpu_limit`, `status`, and `uptime` are only present in the first frame
/// of each connection; later frames omit them. See [`StatusMerger`].
#[derive(Debug, Deserialize)]
pub(crate) struct StatusFrame {
    cpu: f64,
    #[serde(rename = "cpuLimit")]
    cpu_limit: Option<f64>,
    ram: RealtimeRam,
    status: Option<String>,
    #[serde(rename = "netIO")]
    net_io: RealtimeNetIo,
    #[serde(rename = "bIO")]
    block_io: RealtimeBlockIo,
    #[serde(default, with = "chrono::serde::ts_milliseconds_option")]
    uptime: Option<DateTime<Utc>>,
}

/// Accumulates the constant fields carried by the first complete `status`
/// frame of a connection and fills them into every later, lean frame.
#[derive(Debug, Default)]
pub(crate) struct StatusMerger {
    cpu_limit: Option<f64>,
    status: Option<String>,
    uptime: Option<DateTime<Utc>>,
}

impl StatusMerger {
    pub(crate) fn merge(&mut self, frame: StatusFrame) -> RealtimeStatus {
        if let Some(cpu_limit) = frame.cpu_limit {
            self.cpu_limit = Some(cpu_limit);
        }
        if let Some(status) = frame.status {
            self.status = Some(status);
        }
        if let Some(uptime) = frame.uptime {
            self.uptime = Some(uptime);
        }
        RealtimeStatus {
            cpu: frame.cpu,
            cpu_limit: self.cpu_limit,
            ram: frame.ram,
            status: self.status.clone(),
            net_io: frame.net_io,
            block_io: frame.block_io,
            uptime: self.uptime,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{RealtimeRam, StatusFrame, StatusMerger};

    #[test]
    fn ram_deserializes_from_two_element_array() {
        // Real API responses report used_mb as a fraction (e.g. 13.83), not
        // a whole number; this is the case that broke the earlier u64-typed
        // version of this struct.
        let ram: RealtimeRam =
            serde_json::from_value(json!([13.83, 512])).unwrap();
        assert_eq!(ram.used_mb, 13.83);
        assert_eq!(ram.limit_mb, 512.0);
    }

    fn full_frame() -> serde_json::Value {
        json!({
            "cpu": 12.5,
            "cpuLimit": 100,
            "ram": [12.98, 512],
            "status": "running",
            "netIO": { "i": 2048, "o": 4096, "new": { "i": 64, "o": 128 } },
            "bIO": { "i": 0, "o": 0 },
            "uptime": 1_716_000_000_000i64
        })
    }

    fn lean_frame() -> serde_json::Value {
        json!({
            "cpu": 13.1,
            "ram": [13.14, 512],
            "netIO": { "i": 2176, "o": 4288, "new": { "i": 72, "o": 140 } },
            "bIO": { "i": 0, "o": 0 }
        })
    }

    #[test]
    fn first_full_frame_populates_every_field() {
        let frame: StatusFrame = serde_json::from_value(full_frame()).unwrap();
        let status = StatusMerger::default().merge(frame);
        assert_eq!(status.cpu, 12.5);
        assert_eq!(status.cpu_limit, Some(100.0));
        assert_eq!(status.status.as_deref(), Some("running"));
        assert!(status.uptime.is_some());
    }

    #[test]
    fn lean_frame_inherits_constant_fields_from_last_full_frame() {
        let mut merger = StatusMerger::default();
        let full: StatusFrame = serde_json::from_value(full_frame()).unwrap();
        merger.merge(full);

        let lean: StatusFrame = serde_json::from_value(lean_frame()).unwrap();
        let status = merger.merge(lean);

        assert_eq!(status.cpu, 13.1);
        assert_eq!(status.cpu_limit, Some(100.0));
        assert_eq!(status.status.as_deref(), Some("running"));
        assert!(status.uptime.is_some());
        assert_eq!(status.ram.used_mb, 13.14);
    }

    #[test]
    fn lean_frame_before_any_full_frame_leaves_constant_fields_none() {
        let lean: StatusFrame = serde_json::from_value(lean_frame()).unwrap();
        let status = StatusMerger::default().merge(lean);
        assert_eq!(status.cpu_limit, None);
        assert_eq!(status.status, None);
        assert_eq!(status.uptime, None);
    }
}
