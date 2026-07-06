use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let db = client.database("database_id");
    let snapshots = db.list_snapshots().await.unwrap();
    let snapshot = snapshots.first().unwrap();
    db.restore_snapshot(snapshot.name.clone(), snapshot.version_id().to_string())
        .await
        .unwrap();
}
