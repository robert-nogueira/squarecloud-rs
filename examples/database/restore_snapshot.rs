use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client =
        Client::new(std::env::var("API_TOKEN").expect("set API_TOKEN"));
    let db_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <db_id>");
    let db = client.database(&db_id);
    let snapshots = db.list_snapshots().await.unwrap();
    let snapshot = snapshots.first().unwrap();
    db.restore_snapshot(
        snapshot.name.clone(),
        snapshot.version_id().to_string(),
    )
    .await
    .unwrap();
}
