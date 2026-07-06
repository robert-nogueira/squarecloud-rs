use squarecloud::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <app_id>");
    let snapshot = client.app(&app_id).create_snapshot().await.unwrap();
    println!("url: {}", snapshot.url);
}
