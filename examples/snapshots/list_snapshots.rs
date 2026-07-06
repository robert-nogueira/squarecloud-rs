use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let snapshots = client
        .app("application_id")
        .list_snapshots()
        .await
        .unwrap();
    println!("{snapshots:#?}");
}
