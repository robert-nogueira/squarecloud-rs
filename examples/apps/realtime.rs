use futures_util::StreamExt;
use squarecloud::{Client, RealtimeEvent};

#[tokio::main]
async fn main() {
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example realtime -- <app_id>");
    let client = Client::new();
    let mut stream = client.app(&app_id).realtime();
    while let Some(event) = stream.next().await {
        match event.expect("stream error") {
            RealtimeEvent::Log(msg) => println!("[log]    {msg}"),
            RealtimeEvent::System(msg) => println!("[system] {msg}"),
        }
    }
}
