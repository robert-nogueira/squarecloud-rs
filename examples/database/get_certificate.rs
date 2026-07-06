use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let cert = client.database("database_id").certificate().await.unwrap();
    println!("{cert}");
}
