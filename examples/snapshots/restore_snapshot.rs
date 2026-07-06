use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let app = client.app("application_id");
    let snapshots = app.list_snapshots().await.unwrap();
    let snapshot = snapshots.first().unwrap();
    app.restore_snapshot(snapshot.name.clone(), snapshot.version_id().to_string())
        .await
        .unwrap();
}
