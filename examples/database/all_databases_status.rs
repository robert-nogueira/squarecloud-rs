use squarecloud::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let statuses = client.all_database_status().await.unwrap();
    println!("{statuses:#?}");
}
