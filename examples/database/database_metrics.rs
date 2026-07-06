use squarecloud::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let db_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <db_id>");
    let metrics = client.database(&db_id).metrics().await.unwrap();
    println!("{metrics:#?}");
}
