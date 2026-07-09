use chrono::{Duration, Utc};
use squarecloud::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example network_errors -- <app_id>");
    let end = Utc::now();
    let start = end - Duration::days(7);
    let errors = client
        .app(&app_id)
        .network_errors(false, start, end)
        .await
        .unwrap();
    println!("{errors:#?}");
}
