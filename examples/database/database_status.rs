use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let status = client.database("database_id").status().await.unwrap();
    println!("{status:#?}");
}
