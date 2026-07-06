use squarecloud::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let db_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <db_id>");
    let snapshot = client.database(&db_id).create_snapshot().await.unwrap();
    println!("url: {}", snapshot.url);
}
