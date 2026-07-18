use futures_util::StreamExt;
use squarecloud::{Client, RealtimeEvent};

#[tokio::main]
async fn main() {
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example realtime -- <app_id>");
    let client =
        Client::new(std::env::var("API_TOKEN").expect("set API_TOKEN"));
    let mut stream = client.app(&app_id).realtime();
    while let Some(event) = stream.next().await {
        match event.expect("stream error") {
            RealtimeEvent::Log { stream, line } => {
                println!("[log:{stream:?}] {line}")
            }
            RealtimeEvent::System(msg) => println!("[system] {msg}"),
            RealtimeEvent::Status(status) => {
                println!(
                    "[status] cpu={:.1}% ram={}/{}MB",
                    status.cpu, status.ram.used_mb, status.ram.limit_mb
                )
            }
        }
    }
}
