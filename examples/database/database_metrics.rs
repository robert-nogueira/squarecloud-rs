use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let metrics = client.database("database_id").metrics().await.unwrap();
    println!("{metrics:#?}");
}
