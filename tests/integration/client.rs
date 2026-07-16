use squarecloud::{Client, types::SnapshotScope};

#[tokio::test]
async fn service_status_returns_status() {
    crate::setup();
    crate::throttle().await;
    let client = Client::new();
    let status = client
        .service_status()
        .await
        .expect("service_status() should return status");
    assert!(!status.status.is_empty());
    assert!(!status.message.is_empty());
}

#[tokio::test]
async fn all_domains_returns_vec() {
    crate::setup();
    crate::throttle().await;
    let client = Client::new();
    let result = client.all_domains().await;
    assert!(result.is_ok(), "all_domains() failed: {:?}", result.err());
}

#[tokio::test]
async fn all_database_status_returns_vec() {
    crate::setup();
    crate::throttle().await;
    let client = Client::new();
    let result = client.all_database_status().await;
    assert!(
        result.is_ok(),
        "all_database_status() failed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn all_snapshots_returns_vec() {
    crate::setup();
    crate::throttle().await;
    let client = Client::new();
    let result = client.all_snapshots(None).await;
    assert!(result.is_ok(), "all_snapshots() failed: {:?}", result.err());
}

#[tokio::test]
async fn all_snapshots_with_scope_returns_vec() {
    crate::setup();
    crate::throttle().await;
    let client = Client::new();
    let result = client
        .all_snapshots(Some(SnapshotScope::Applications))
        .await;
    assert!(
        result.is_ok(),
        "all_snapshots(scope=applications) failed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn all_workspaces_returns_vec() {
    crate::setup();
    crate::throttle().await;
    let client = Client::new();
    let result = client.all_workspaces().await;
    assert!(
        result.is_ok(),
        "all_workspaces() failed: {:?}",
        result.err()
    );
}
