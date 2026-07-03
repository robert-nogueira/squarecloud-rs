use squarecloud_rs::ApiClient;

#[tokio::test]
async fn app_info_matches_uploaded() {
    crate::setup();
    let app_id = crate::shared_app_id().await;
    let client = ApiClient::new();
    let app = client.app(app_id);

    let info = app.info().await.unwrap();
    assert_eq!(info.id, app_id);
    assert_eq!(info.name, "squarecloud-rs-test");
    assert_eq!(info.ram, 512);
    assert!(!info.language.is_empty());
}

#[tokio::test]
async fn app_status_returns_runtime_stats() {
    crate::setup();
    let app_id = crate::shared_app_id().await;
    let client = ApiClient::new();
    let status = client.app(app_id).status().await.unwrap();

    assert!(!status.cpu.is_empty());
    assert!(!status.ram.is_empty());
    assert!(!status.status.is_empty());
    assert!(!status.storage.is_empty());
}

#[tokio::test]
async fn app_logs_returns_string() {
    crate::setup();
    let app_id = crate::shared_app_id().await;
    let client = ApiClient::new();
    let logs = client.app(app_id).logs().await.unwrap();

    assert!(!logs.is_empty());
}

/// Must stay last alphabetically so it runs after all other app tests.
#[tokio::test]
async fn z_cleanup_shared_app() {
    if let Some(id) = crate::shared_app_id_if_initialized() {
        let client = ApiClient::new();
        client.app(id).delete().await.unwrap();
    }
}
