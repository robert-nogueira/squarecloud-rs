use squarecloud_rs::{ApiClient, ApiError, ApiErrorCode};

#[tokio::test]
async fn app_info_matches_uploaded() {
    crate::setup();
    let app_id = crate::shared_app_id();
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
    let app_id = crate::shared_app_id();
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
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    let logs = client.app(app_id).logs().await.unwrap();

    assert!(!logs.is_empty());
}

#[tokio::test]
async fn all_apps_status_includes_shared_app() {
    crate::setup();
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    let statuses = client.all_apps_status().await.unwrap();

    assert!(
        statuses.iter().any(|s| s.id == app_id),
        "shared app not found in all_apps_status"
    );
}

#[tokio::test]
async fn app_envs_crud() {
    crate::setup();
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    let app = client.app(app_id);

    let envs = std::collections::HashMap::from([
        ("TEST_KEY".to_string(), "hello".to_string()),
    ]);

    let after_upsert = app.upsert_envs(&envs).await.unwrap();
    assert_eq!(after_upsert.get("TEST_KEY").map(String::as_str), Some("hello"));

    let listed = app.list_envs().await.unwrap();
    assert!(listed.contains_key("TEST_KEY"));

    let overwrite = std::collections::HashMap::from([
        ("OTHER_KEY".to_string(), "world".to_string()),
    ]);
    let after_overwrite = app.overwrite_envs(&overwrite).await.unwrap();
    assert!(!after_overwrite.contains_key("TEST_KEY"));
    assert_eq!(after_overwrite.get("OTHER_KEY").map(String::as_str), Some("world"));

    let after_delete = app.delete_envs(&["OTHER_KEY".to_string()]).await.unwrap();
    assert!(!after_delete.contains_key("OTHER_KEY"));
}

#[tokio::test]
async fn app_commit_returns_true() {
    crate::setup();
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    assert!(client.app(app_id).commit(crate::helpers::dummy_zip()).await.unwrap());
}

#[tokio::test]
async fn app_metrics_returns_vec() {
    crate::setup();
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    let _ = client.app(app_id).metrics().await.unwrap();
}

#[tokio::test]
async fn app_restart_returns_true() {
    crate::setup();
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    assert!(client.app(app_id).restart().await.unwrap());
}

#[tokio::test]
async fn app_start_returns_true_or_already_started() {
    crate::setup();
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    match client.app(app_id).start().await {
        Ok(v) => assert!(v),
        Err(ApiError::Api { code: ApiErrorCode::ContainerAlreadyStarted }) => {}
        Err(e) => panic!("unexpected error: {e:?}"),
    }
}

#[tokio::test]
async fn app_stop_returns_true() {
    crate::setup();
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    assert!(client.app(app_id).stop().await.unwrap());
}

/// Must stay last alphabetically so it runs after all other app tests.
#[tokio::test]
async fn z_cleanup_shared_app() {
    if let Some(id) = crate::shared_app_id_if_initialized() {
        let client = ApiClient::new();
        client.app(id).delete().await.unwrap();
    }
}
