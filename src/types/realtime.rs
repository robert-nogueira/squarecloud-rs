/// An event received from the application's realtime log stream.
///
/// Returned by
/// [`AppResource::realtime`](crate::resources::AppResource::realtime).
#[derive(Debug, Clone)]
pub enum RealtimeEvent {
    /// A log line emitted by the application.
    Log(String),
    /// A lifecycle or keepalive message from the SSE server
    /// (e.g. `REALTIME_CONNECTED`, `PING`).
    System(String),
}
