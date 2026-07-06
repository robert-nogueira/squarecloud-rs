use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    client
        .database("database_id")
        .edit(Some("new-name"), None)
        .await
        .unwrap();
}
