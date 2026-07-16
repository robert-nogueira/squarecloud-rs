use chrono::{Duration, Utc};
use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example network_performance -- <app_id>");
    let end = Utc::now();
    let start = end - Duration::days(7);
    let perf = client
        .app(&app_id)
        .network_performance(start, end)
        .await
        .unwrap();
    println!("{perf:#?}");
}
