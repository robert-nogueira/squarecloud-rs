use squarecloud::{
    ApiError, Client, errors::SnapshotErrorCode, types::RestoreState,
};

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

    match db
        .restore_snapshot(
            snapshot.name.clone(),
            snapshot.version_id().to_string(),
        )
        .await
    {
        Ok(_) => println!("Restore finished"),
        // Large snapshots keep restoring in the background.
        Err(ApiError::Service {
            code: SnapshotErrorCode::SnapshotRestoreProcessing,
        }) => {
            let status = db.restore_status().await.unwrap();
            if status.state == RestoreState::Running {
                println!("Restore running, step: {:?}", status.step);
            } else {
                println!("Restore state: {:?}", status.state);
            }
        }
        Err(e) => panic!("restore failed: {e}"),
    }
}
