use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client =
        Client::new(std::env::var("API_TOKEN").expect("set API_TOKEN"));
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <app_id>");
    let app = client.app(&app_id);
    let snapshots = app.list_snapshots().await.unwrap();
    let snapshot = snapshots.first().unwrap();
    app.restore_snapshot(
        snapshot.name.clone(),
        snapshot.version_id().to_string(),
    )
    .await
    .unwrap();
}
