use chrono::{Duration, Utc};
use futures_util::StreamExt;
use squarecloud::{
    ApiError,
    errors::{AppErrorCode, SnapshotErrorCode},
    types::RealtimeEvent,
};

#[tokio::test]
async fn app_info_matches_uploaded() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = crate::client();
    let app = client.app(app_id);

    let info = app.info().await.expect("info() should return app info");
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
    let client = crate::client();
    let status = client
        .app(app_id)
        .status()
        .await
        .expect("status() should return runtime stats");

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
    let client = crate::client();
    let result = client.app(app_id).logs().await;
    assert!(result.is_ok(), "logs() failed: {:?}", result.err());
}

#[tokio::test]
async fn all_apps_status_includes_shared_app() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = crate::client();
    let statuses = client
        .all_apps_status()
        .await
        .expect("all_apps_status() should return a vec");

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
    let client = crate::client();
    let app = client.app(app_id);

    let envs = std::collections::HashMap::from([(
        "TEST_KEY".to_string(),
        "hello".to_string(),
    )]);

    let after_upsert = app
        .upsert_envs(&envs)
        .await
        .expect("upsert_envs() should succeed");
    assert_eq!(
        after_upsert.get("TEST_KEY").map(String::as_str),
        Some("hello")
    );

    let listed = app
        .list_envs()
        .await
        .expect("list_envs() should return the env map");
    assert!(listed.contains_key("TEST_KEY"));

    let overwrite = std::collections::HashMap::from([(
        "OTHER_KEY".to_string(),
        "world".to_string(),
    )]);
    let after_overwrite = app
        .overwrite_envs(&overwrite)
        .await
        .expect("overwrite_envs() should succeed");
    assert!(!after_overwrite.contains_key("TEST_KEY"));
    assert_eq!(
        after_overwrite.get("OTHER_KEY").map(String::as_str),
        Some("world")
    );

    let after_delete = app
        .delete_envs(&["OTHER_KEY".to_string()])
        .await
        .expect("delete_envs() should succeed");
    assert!(!after_delete.contains_key("OTHER_KEY"));
}

#[tokio::test]
async fn app_commit_returns_true() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = crate::client();
    assert!(
        client
            .app(app_id)
            .commit(crate::helpers::dummy_zip())
            .await
            .expect("commit() should succeed")
    );
}

/// Verifies `commit_to` actually unpacks the archive at `path`, not just
/// that the API returned success: reads the committed file back from the
/// destination directory and checks its content matches what was sent.
#[tokio::test]
async fn app_commit_to_path_unpacks_at_destination() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = crate::client();
    let app = client.app(app_id);

    assert!(
        app.commit_to(
            crate::helpers::dummy_zip(),
            Some("squarecloud_rs_test_commit_to")
        )
        .await
        .expect("commit_to() should succeed")
    );

    let content = app
        .file("/")
        .read("squarecloud_rs_test_commit_to/index.js")
        .await
        .expect(
            "index.js should exist at the commit_to destination directory",
        );
    assert_eq!(content.data, crate::helpers::DUMMY_INDEX_JS);
}

#[tokio::test]
async fn app_analytics_returns_analytics() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = crate::client();
    let end = Utc::now();
    let start = end - Duration::days(7);
    client
        .app(app_id)
        .analytics(start, end)
        .await
        .expect("analytics() should return data for valid date range");
}

#[tokio::test]
#[ignore = "requires a custom domain configured on the test app"]
async fn app_dns_records_returns_records() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = crate::client();
    let records = client
        .app(app_id)
        .dns_records()
        .await
        .expect("dns_records() should return DNS records");
    assert!(!records.is_empty());
    for record in &records {
        assert!(!record.name.is_empty());
        assert!(!record.value.is_empty());
    }
}

#[tokio::test]
async fn app_network_errors_returns_result() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = crate::client();
    let end = Utc::now();
    let start = end - Duration::days(7);
    client
        .app(app_id)
        .network_errors(false, start, end)
        .await
        .expect("network_errors(5xx only) should succeed");
    client
        .app(app_id)
        .network_errors(true, start, end)
        .await
        .expect("network_errors(include 4xx) should succeed");
}

#[tokio::test]
#[ignore = "requires a Pro or Enterprise plan"]
async fn app_network_logs_returns_vec() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = crate::client();
    let end = Utc::now();
    let start = end - Duration::days(7);
    client
        .app(app_id)
        .network_logs(start, end)
        .await
        .expect("network_logs() should succeed");
}

#[tokio::test]
#[ignore = "requires a Pro or Enterprise plan"]
async fn app_network_performance_returns_result() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = crate::client();
    let end = Utc::now();
    let start = end - Duration::days(7);
    client
        .app(app_id)
        .network_performance(start, end)
        .await
        .expect("network_performance() should succeed");
}

#[tokio::test]
async fn app_purge_cache_returns_true() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = crate::client();
    assert!(
        client
            .app(app_id)
            .purge_cache()
            .await
            .expect("purge_cache() should return true")
    );
}

#[tokio::test]
async fn app_metrics_returns_vec() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = crate::client();
    let _ = client
        .app(app_id)
        .metrics()
        .await
        .expect("metrics() should return vec");
}

#[tokio::test]
async fn app_restart_returns_true() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = crate::client();
    assert!(
        client
            .app(app_id)
            .restart()
            .await
            .expect("restart() should return true")
    );
}

#[tokio::test]
async fn app_start_returns_true() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = crate::client();
    client
        .app(app_id)
        .stop()
        .await
        .expect("stop() should succeed before start");
    crate::throttle().await;
    assert!(
        client
            .app(app_id)
            .start()
            .await
            .expect("start() should return true")
    );
}

#[tokio::test]
async fn app_stop_returns_true() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = crate::client();
    assert!(
        client
            .app(app_id)
            .stop()
            .await
            .expect("stop() should return true")
    );
}

#[tokio::test]
async fn app_current_deploy_returns_deploy() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = crate::client();
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
    let client = crate::client();
    let result = client.app(app_id).list_deploys().await;
    assert!(result.is_ok(), "list_deploys() failed: {:?}", result.err());
}

#[tokio::test]
async fn app_snapshot_lifecycle() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = crate::client();
    let app = client.app(app_id);

    let snap = match app.create_snapshot().await {
        Ok(s) => s,
        Err(ApiError::Service {
            code: SnapshotErrorCode::DailySnapshotsLimitReached,
        }) => {
            eprintln!("Skipping app_snapshot_lifecycle: daily limit reached");
            return;
        }
        Err(e) => panic!("create_snapshot failed: {e:?}"),
    };
    assert!(!snap.url.is_empty());
    assert!(!snap.key.is_empty());

    crate::throttle().await;
    let snapshots = app
        .list_snapshots()
        .await
        .expect("list_snapshots() should return snapshots after create");
    assert!(!snapshots.is_empty());

    let first = &snapshots[0];
    crate::throttle().await;
    assert!(
        app.restore_snapshot(
            first.name.clone(),
            first.version_id().to_string(),
        )
        .await
        .expect("restore_snapshot() should succeed")
    );
}

#[tokio::test]
async fn app_file_operations() {
    crate::setup();
    crate::throttle().await;
    let app_id = crate::shared_app_id();
    let client = crate::client();
    let app = client.app(app_id);

    let files = app
        .file("/")
        .all_files("/")
        .await
        .expect("all_files() should return file list");
    assert!(!files.is_empty());

    let handle = app.file("/squarecloud_rs_test.txt");
    crate::throttle().await;
    assert!(
        handle
            .write("hello from squarecloud-rs")
            .await
            .expect("write() should succeed")
    );

    crate::throttle().await;
    let content = handle
        .read("/squarecloud_rs_test.txt")
        .await
        .expect("read() should return file content");
    assert!(!content.data_type.is_empty());

    crate::throttle().await;
    assert!(
        handle
            .move_to("/squarecloud_rs_test_moved.txt")
            .await
            .expect("move_to() should succeed")
    );

    crate::throttle().await;
    assert!(
        app.file("/squarecloud_rs_test_moved.txt")
            .delete()
            .await
            .expect("delete() should succeed")
    );
}

/// Must stay last alphabetically so it runs after all other app tests.
#[tokio::test]
async fn z_cleanup_shared_app() {
    if let Some(id) = crate::shared_app_id_if_initialized() {
        let app = crate::client().app(id);
        for attempt in 0..3_u32 {
            match app.delete().await {
                Ok(_) => return,
                Err(ApiError::Service {
                    code: AppErrorCode::RestoreInProgress,
                }) if attempt < 2 => {
                    tokio::time::sleep(std::time::Duration::from_secs(15))
                        .await;
                }
                Err(ApiError::Service {
                    code: AppErrorCode::Unknown(ref raw),
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
    let client = crate::client();

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
