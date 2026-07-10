use chrono::{Duration, Utc};
use futures_util::StreamExt;
use squarecloud::{ApiClient, ApiError, ApiErrorCode, types::RealtimeEvent};

#[tokio::test]
async fn app_info_matches_uploaded() {
    crate::setup();
    crate::throttle().await;
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
    crate::throttle().await;
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
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    let result = client.app(app_id).logs().await;
    assert!(result.is_ok(), "logs() failed: {:?}", result.err());
}

#[tokio::test]
async fn all_apps_status_includes_shared_app() {
    crate::setup();
    crate::throttle().await;
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
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    let app = client.app(app_id);

    let envs = std::collections::HashMap::from([(
        "TEST_KEY".to_string(),
        "hello".to_string(),
    )]);

    let after_upsert = app.upsert_envs(&envs).await.unwrap();
    assert_eq!(
        after_upsert.get("TEST_KEY").map(String::as_str),
        Some("hello")
    );

    let listed = app.list_envs().await.unwrap();
    assert!(listed.contains_key("TEST_KEY"));

    let overwrite = std::collections::HashMap::from([(
        "OTHER_KEY".to_string(),
        "world".to_string(),
    )]);
    let after_overwrite = app.overwrite_envs(&overwrite).await.unwrap();
    assert!(!after_overwrite.contains_key("TEST_KEY"));
    assert_eq!(
        after_overwrite.get("OTHER_KEY").map(String::as_str),
        Some("world")
    );

    let after_delete =
        app.delete_envs(&["OTHER_KEY".to_string()]).await.unwrap();
    assert!(!after_delete.contains_key("OTHER_KEY"));
}

#[tokio::test]
async fn app_commit_returns_true() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    assert!(
        client
            .app(app_id)
            .commit(crate::helpers::dummy_zip())
            .await
            .unwrap()
    );
}

#[tokio::test]
async fn app_analytics_returns_analytics() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    let end = Utc::now();
    let start = end - Duration::days(7);
    client.app(app_id).analytics(start, end).await.unwrap();
}

#[tokio::test]
#[ignore = "requires a custom domain configured on the test app"]
async fn app_dns_record_returns_record() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    let record = client.app(app_id).dns_record().await.unwrap();
    assert!(!record.name.is_empty());
    assert!(!record.value.is_empty());
}

#[tokio::test]
async fn app_network_errors_returns_result() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    let end = Utc::now();
    let start = end - Duration::days(7);
    client
        .app(app_id)
        .network_errors(false, start, end)
        .await
        .unwrap();
    client
        .app(app_id)
        .network_errors(true, start, end)
        .await
        .unwrap();
}

#[tokio::test]
#[ignore = "requires a Pro or Enterprise plan"]
async fn app_network_logs_returns_vec() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    let end = Utc::now();
    let start = end - Duration::days(7);
    client.app(app_id).network_logs(start, end).await.unwrap();
}

#[tokio::test]
#[ignore = "requires a Pro or Enterprise plan"]
async fn app_network_performance_returns_result() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    let end = Utc::now();
    let start = end - Duration::days(7);
    client
        .app(app_id)
        .network_performance(start, end)
        .await
        .unwrap();
}

#[tokio::test]
async fn app_purge_cache_returns_true() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    assert!(client.app(app_id).purge_cache().await.unwrap());
}

#[tokio::test]
async fn app_metrics_returns_vec() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    let _ = client.app(app_id).metrics().await.unwrap();
}

#[tokio::test]
async fn app_restart_returns_true() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    assert!(client.app(app_id).restart().await.unwrap());
}

#[tokio::test]
async fn app_start_returns_true() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    client.app(app_id).stop().await.unwrap();
    crate::throttle().await;
    assert!(client.app(app_id).start().await.unwrap());
}

#[tokio::test]
async fn app_stop_returns_true() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    assert!(client.app(app_id).stop().await.unwrap());
}

#[tokio::test]
async fn app_current_deploy_returns_deploy() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    let result = client.app(app_id).current_deploy().await;
    assert!(
        result.is_ok(),
        "current_deploy() failed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn app_list_deploys_returns_vec() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    let result = client.app(app_id).list_deploys().await;
    assert!(result.is_ok(), "list_deploys() failed: {:?}", result.err());
}

#[tokio::test]
async fn app_snapshot_lifecycle() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    let app = client.app(app_id);

    let snap = match app.create_snapshot().await {
        Ok(s) => s,
        Err(ApiError::Api {
            code: ApiErrorCode::DailySnapshotsLimitReached,
        }) => {
            eprintln!("Skipping app_snapshot_lifecycle: daily limit reached");
            return;
        }
        Err(e) => panic!("create_snapshot failed: {e:?}"),
    };
    assert!(!snap.url.is_empty());
    assert!(!snap.key.is_empty());

    crate::throttle().await;
    let snapshots = app.list_snapshots().await.unwrap();
    assert!(!snapshots.is_empty());

    let first = &snapshots[0];
    crate::throttle().await;
    assert!(
        app.restore_snapshot(
            first.name.clone(),
            first.version_id().to_string(),
        )
        .await
        .unwrap()
    );
}

#[tokio::test]
async fn app_file_operations() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();
    let app = client.app(app_id);

    let files = app.file("/").all_files("/").await.unwrap();
    assert!(!files.is_empty());

    let handle = app.file("/squarecloud_rs_test.txt");
    crate::throttle().await;
    assert!(handle.write("hello from squarecloud-rs").await.unwrap());

    crate::throttle().await;
    let content = handle.read("/squarecloud_rs_test.txt").await.unwrap();
    assert!(!content.data_type.is_empty());

    crate::throttle().await;
    assert!(
        handle
            .move_to("/squarecloud_rs_test_moved.txt")
            .await
            .unwrap()
    );

    crate::throttle().await;
    assert!(
        app.file("/squarecloud_rs_test_moved.txt")
            .delete()
            .await
            .unwrap()
    );
}

/// Must stay last alphabetically so it runs after all other app tests.
#[tokio::test]
async fn z_cleanup_shared_app() {
    if let Some(id) = crate::shared_app_id_if_initialized() {
        let app = ApiClient::new().app(id);
        for attempt in 0..3_u32 {
            match app.delete().await {
                Ok(_) => return,
                Err(ApiError::Api {
                    code: ApiErrorCode::RestoreInProgress,
                }) if attempt < 2 => {
                    tokio::time::sleep(std::time::Duration::from_secs(15))
                        .await;
                }
                Err(ApiError::Api {
                    code: ApiErrorCode::Unknown(ref raw),
                }) => {
                    eprintln!(
                        "cleanup: uncatalogued API code on delete: {raw:?}"
                    );
                    return;
                }
                Err(e) => {
                    eprintln!("cleanup: delete failed: {e:?}");
                    return;
                }
            }
        }
    }
}

#[tokio::test]
async fn app_realtime_receives_log_events() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = ApiClient::new();

    // The server emits System events (REALTIME_CONNECTING, cluster ID,
    // REALTIME_CONNECTED) before any log events. Filter directly for Log
    // events and wait up to 10s; the app logs every 1s.
    let app = client.app(app_id);
    let stream = app.realtime().filter(|e| {
        futures_util::future::ready(matches!(e, Ok(RealtimeEvent::Log(_))))
    });
    tokio::pin!(stream);

    let first_log = tokio::time::timeout(
        std::time::Duration::from_secs(10),
        stream.next(),
    )
    .await
    .expect("timed out waiting for a Log event");

    assert!(
        matches!(first_log, Some(Ok(RealtimeEvent::Log(_)))),
        "expected a Log event, got: {first_log:?}"
    );
}
