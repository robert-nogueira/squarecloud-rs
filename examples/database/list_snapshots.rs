use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let snapshots = client
        .database("database_id")
        .list_snapshots()
        .await
        .unwrap();
    println!("{snapshots:#?}");
}
