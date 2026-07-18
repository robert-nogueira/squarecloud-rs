use futures_util::StreamExt;
use squarecloud::{Client, LogStream, RealtimeEvent};

#[tokio::main]
async fn main() {
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example realtime_logs -- <app_id>");
    let client =
        Client::new(std::env::var("API_TOKEN").expect("set API_TOKEN"));
    let stream = client.app(&app_id).realtime().filter_map(|e| async {
        match e.expect("stream error") {
            RealtimeEvent::Log { stream, line } => {
                let prefix = match stream {
                    LogStream::Stdout => "",
                    LogStream::Stderr => "[stderr] ",
                };
                Some(format!("{prefix}{line}"))
            }
            _ => None,
        }
    });
    tokio::pin!(stream);
    while let Some(msg) = stream.next().await {
        println!("{msg}");
    }
}
