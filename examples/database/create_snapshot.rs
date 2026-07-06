use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let snapshot = client
        .database("database_id")
        .create_snapshot()
        .await
        .unwrap();
    println!("url: {}", snapshot.url);
}
