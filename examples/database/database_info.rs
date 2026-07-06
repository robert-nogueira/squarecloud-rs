use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let info = client.database("database_id").info().await.unwrap();
    println!("{info:#?}");
}
