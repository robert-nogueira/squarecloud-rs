use serde_json::json;
use squarecloud_rs::{ApiError, ApiErrorCode};
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn app_info_matches_uploaded() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "id": "app-123",
                "name": "squarecloud-rs-test",
                "owner": "user-123",
                "cluster": "florida-1",
                "ram": 512,
                "language": "javascript",
                "domain": null,
                "custom": null
            }
        })))
        .mount(&server)
        .await;

    let info = client.app("app-123").info().await.unwrap();
    assert_eq!(info.id, "app-123");
    assert_eq!(info.name, "squarecloud-rs-test");
    assert_eq!(info.ram, 512);
    assert!(!info.language.is_empty());
}

#[tokio::test]
async fn app_status_returns_runtime_stats() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "cpu": "2.5%",
                "ram": "128/512MB",
                "status": "running",
                "running": true,
                "storage": "50MB",
                "network": { "total": "1MB", "now": "0KB" },
                "uptime": 1700000000000i64
            }
        })))
        .mount(&server)
        .await;

    let status = client.app("app-123").status().await.unwrap();
    assert!(!status.cpu.is_empty());
    assert!(!status.ram.is_empty());
    assert!(!status.status.is_empty());
    assert!(!status.storage.is_empty());
    assert!(status.uptime.is_some());
}

#[tokio::test]
async fn app_status_stopped() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "cpu": "0%",
                "ram": "0MB",
                "status": "exited",
                "running": false,
                "storage": "50MB",
                "network": { "total": "0MB", "now": "0KB" },
                "uptime": null
            }
        })))
        .mount(&server)
        .await;

    let status = client.app("app-123").status().await.unwrap();
    assert!(!status.running);
    assert!(status.uptime.is_none());
}

#[tokio::test]
async fn app_logs_returns_string() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/logs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": { "logs": "Server started on port 80\n" }
        })))
        .mount(&server)
        .await;

    let result = client.app("app-123").logs().await;
    assert!(result.is_ok(), "logs() failed: {:?}", result.err());
}

#[tokio::test]
async fn all_apps_status_includes_shared_app() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": [
                { "id": "app-123", "running": true,  "cpu": "1.2%", "ram": "100MB" },
                { "id": "app-456", "running": false, "cpu": null,   "ram": null }
            ]
        })))
        .mount(&server)
        .await;

    let statuses = client.all_apps_status().await.unwrap();
    assert!(
        statuses.iter().any(|s| s.id == "app-123"),
        "app-123 not found in all_apps_status"
    );
    assert!(statuses[1].cpu.is_none());
}

#[tokio::test]
async fn app_envs_crud() {
    let (client, server) = crate::mock_client().await;

    Mock::given(method("POST"))
        .and(path("/apps/app-123/envs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": { "TEST_KEY": "hello" }
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/apps/app-123/envs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": { "TEST_KEY": "hello" }
        })))
        .mount(&server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/apps/app-123/envs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": { "OTHER_KEY": "world" }
        })))
        .mount(&server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/apps/app-123/envs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {}
        })))
        .mount(&server)
        .await;

    let app = client.app("app-123");
    let envs = std::collections::HashMap::from([
        ("TEST_KEY".to_string(), "hello".to_string()),
    ]);

    let after_upsert = app.upsert_envs(&envs).await.unwrap();
    assert_eq!(
        after_upsert.get("TEST_KEY").map(String::as_str),
        Some("hello")
    );

    let listed = app.list_envs().await.unwrap();
    assert!(listed.contains_key("TEST_KEY"));

    let overwrite = std::collections::HashMap::from([
        ("OTHER_KEY".to_string(), "world".to_string()),
    ]);
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
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/apps/app-123/commit"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    assert!(client.app("app-123").commit(vec![]).await.unwrap());
}

#[tokio::test]
async fn app_analytics_returns_analytics() {
    let (client, server) = crate::mock_client().await;
    let item = json!({
        "type": "BR", "visits": 10, "requests": 20,
        "bytes": 1024, "date": "2024-01-01"
    });
    Mock::given(method("GET"))
        .and(path("/apps/app-123/network/analytics"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "visits": [item], "countries": [item], "devices": [item],
                "os": [item], "browsers": [item], "protocols": [item],
                "methods": [item], "paths": [item], "referers": [item],
                "providers": [item]
            }
        })))
        .mount(&server)
        .await;

    let analytics = client.app("app-123").analytics().await.unwrap();
    assert_eq!(analytics.countries[0].item_type, "BR");
}

#[tokio::test]
async fn app_analytics_invalid_time_range() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/network/analytics"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "error", "code": "INVALID_TIME_RANGE"
        })))
        .mount(&server)
        .await;

    match client.app("app-123").analytics().await {
        Err(ApiError::Api { code: ApiErrorCode::InvalidTimeRange }) => {}
        other => panic!("expected InvalidTimeRange, got {other:?}"),
    }
}

#[tokio::test]
async fn app_dns_record_returns_record() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/network/dns"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "type": "a",
                "name": "example.com",
                "value": "1.2.3.4",
                "status": "active"
            }
        })))
        .mount(&server)
        .await;

    let record = client.app("app-123").dns_record().await.unwrap();
    assert!(!record.name.is_empty());
    assert!(!record.value.is_empty());
}

#[tokio::test]
async fn app_dns_record_no_custom_domain() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/network/dns"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "error", "code": "NO_CUSTOM_DOMAIN"
        })))
        .mount(&server)
        .await;

    match client.app("app-123").dns_record().await {
        Err(ApiError::Api { code: ApiErrorCode::NoCustomDomain }) => {}
        other => panic!("expected NoCustomDomain, got {other:?}"),
    }
}

#[tokio::test]
async fn app_set_custom_domain_returns_true() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/apps/app-123/network/custom"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    assert!(
        client.app("app-123").set_custom_domain("example.com").await.unwrap()
    );
}

#[tokio::test]
async fn app_network_errors_returns_result() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/network/errors"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "summary": { "total": 5, "by_class": { "5xx": 5 } },
                "by_status": [{ "status": 502, "total": 5 }],
                "timeseries": [],
                "top_paths": [],
                "by_method": {}
            }
        })))
        .mount(&server)
        .await;

    let result = client.app("app-123").network_errors(false).await;
    assert!(result.is_ok(), "network_errors() failed: {:?}", result.err());
}

#[tokio::test]
async fn app_network_errors_invalid_time_range() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/network/errors"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "error", "code": "INVALID_TIME_RANGE"
        })))
        .mount(&server)
        .await;

    match client.app("app-123").network_errors(false).await {
        Err(ApiError::Api { code: ApiErrorCode::InvalidTimeRange }) => {}
        other => panic!("expected InvalidTimeRange, got {other:?}"),
    }
}

#[tokio::test]
async fn app_network_logs_returns_vec() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/network/logs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": [{
                "timestamp": "2024-01-01T00:00:00Z",
                "ip": "1.2.3.4",
                "country": "BR",
                "location": "São Paulo",
                "asn": "AS12345",
                "agent": "curl/8.0",
                "category": "human",
                "mitigated": false,
                "method": "GET",
                "host": "example.com",
                "path": "/",
                "query": null,
                "protocol": "HTTP/2",
                "referer": null,
                "status": 200,
                "contentType": "text/html",
                "cache": "HIT"
            }]
        })))
        .mount(&server)
        .await;

    let result = client.app("app-123").network_logs().await;
    assert!(result.is_ok(), "network_logs() failed: {:?}", result.err());
}

#[tokio::test]
async fn app_network_logs_invalid_time_range() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/network/logs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "error", "code": "INVALID_TIME_RANGE"
        })))
        .mount(&server)
        .await;

    match client.app("app-123").network_logs().await {
        Err(ApiError::Api { code: ApiErrorCode::InvalidTimeRange }) => {}
        other => panic!("expected InvalidTimeRange, got {other:?}"),
    }
}

#[tokio::test]
async fn app_network_performance_returns_result() {
    let (client, server) = crate::mock_client().await;
    let p = json!({ "p50": 10, "p95": 50, "p99": 100 });
    Mock::given(method("GET"))
        .and(path("/apps/app-123/network/performance"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "summary": { "edge": p, "origin": p, "requests": 1000 },
                "timeseries": [],
                "countries": [],
                "colos": [],
                "slowest_paths": []
            }
        })))
        .mount(&server)
        .await;

    let result = client.app("app-123").network_performance().await;
    assert!(result.is_ok(), "network_performance() failed: {:?}", result.err());
}

#[tokio::test]
async fn app_network_performance_invalid_time_range() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/network/performance"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "error", "code": "INVALID_TIME_RANGE"
        })))
        .mount(&server)
        .await;

    match client.app("app-123").network_performance().await {
        Err(ApiError::Api { code: ApiErrorCode::InvalidTimeRange }) => {}
        other => panic!("expected InvalidTimeRange, got {other:?}"),
    }
}

#[tokio::test]
async fn app_purge_cache_returns_true() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/apps/app-123/network/purge_cache"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    assert!(client.app("app-123").purge_cache().await.unwrap());
}

#[tokio::test]
async fn app_purge_cache_keep_calm() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/apps/app-123/network/purge_cache"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "error", "code": "KEEP_CALM"
        })))
        .mount(&server)
        .await;

    match client.app("app-123").purge_cache().await {
        Err(ApiError::Api { code: ApiErrorCode::KeepCalm }) => {}
        other => panic!("expected KeepCalm, got {other:?}"),
    }
}

#[tokio::test]
async fn app_metrics_returns_vec() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/metrics"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": [
                {
                    "date": "2024-01-01T00:00:00Z",
                    "cpu": 1.2,
                    "ram": 128.0,
                    "net": [1000, 2000]
                }
            ]
        })))
        .mount(&server)
        .await;

    let _ = client.app("app-123").metrics().await.unwrap();
}

#[tokio::test]
async fn app_restart_returns_true() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/apps/app-123/restart"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    assert!(client.app("app-123").restart().await.unwrap());
}

#[tokio::test]
async fn app_start_returns_true() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/apps/app-123/start"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    assert!(client.app("app-123").start().await.unwrap());
}

#[tokio::test]
async fn app_start_already_started() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/apps/app-123/start"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "error", "code": "CONTAINER_ALREADY_STARTED"
        })))
        .mount(&server)
        .await;

    match client.app("app-123").start().await {
        Err(ApiError::Api { code: ApiErrorCode::ContainerAlreadyStarted }) => {}
        other => panic!("expected ContainerAlreadyStarted, got {other:?}"),
    }
}

#[tokio::test]
async fn app_stop_returns_true() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/apps/app-123/stop"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    assert!(client.app("app-123").stop().await.unwrap());
}

#[tokio::test]
async fn app_delete_returns_true() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("DELETE"))
        .and(path("/apps/app-123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    assert!(client.app("app-123").delete().await.unwrap());
}

#[tokio::test]
async fn app_current_deploy_returns_deploy() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/deployments/current"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "id": "deploy-1",
                "state": "complete",
                "date": "2024-01-01T00:00:00Z",
                "source": "upload",
                "branch": null,
                "files": null
            }
        })))
        .mount(&server)
        .await;

    let deploy = client.app("app-123").current_deploy().await.unwrap();
    assert!(!deploy.id.is_empty());
    assert!(!deploy.state.is_empty());
}

#[tokio::test]
async fn app_list_deploys_returns_vec() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/deployments"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": [
                {
                    "id": "deploy-1",
                    "state": "complete",
                    "date": "2024-01-01T00:00:00Z",
                    "source": "upload",
                    "branch": null,
                    "files": null
                }
            ]
        })))
        .mount(&server)
        .await;

    let deploys = client.app("app-123").list_deploys().await.unwrap();
    assert!(!deploys.is_empty());
}

#[tokio::test]
async fn app_set_webhook_integration_returns_url() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/apps/app-123/deploy/webhook"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": { "webhook": "https://api.squarecloud.app/v2/deploy/webhook/abc123" }
        })))
        .mount(&server)
        .await;

    let url = client.app("app-123")
        .set_webhook_integration("gh_token".to_string())
        .await
        .unwrap();
    assert!(url.starts_with("https://"));
}

#[tokio::test]
async fn app_snapshot_lifecycle() {
    let (client, server) = crate::mock_client().await;

    Mock::given(method("POST"))
        .and(path("/apps/app-123/snapshots"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "url": "https://storage.example.com/snap-123.zip",
                "key": "snap/app-123/snap-123"
            }
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/apps/app-123/snapshots"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": [
                {
                    "name": "snapshot-1",
                    "size": 1024,
                    "modified": "2024-01-01T00:00:00Z",
                    "key": "snap/app-123/snap-1"
                }
            ]
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/apps/app-123/snapshots/restore"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    let app = client.app("app-123");

    let snap = app.create_snapshot().await.unwrap();
    assert!(!snap.url.is_empty());
    assert!(!snap.key.is_empty());

    let snapshots = app.list_snapshots().await.unwrap();
    assert!(!snapshots.is_empty());

    let first = &snapshots[0];
    assert!(
        app.restore_snapshot(first.name.clone(), first.key.clone())
            .await
            .unwrap()
    );
}

#[tokio::test]
async fn app_file_operations() {
    let (client, server) = crate::mock_client().await;

    Mock::given(method("GET"))
        .and(path("/apps/app-123/files"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": [
                {
                    "name": "index.js",
                    "type": "file",
                    "size": 512,
                    "lastModified": 1704067200000i64
                }
            ]
        })))
        .mount(&server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/apps/app-123/files"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/apps/app-123/files/content"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "type": "text/plain",
                "data": [104, 101, 108, 108, 111]
            }
        })))
        .mount(&server)
        .await;

    Mock::given(method("PATCH"))
        .and(path("/apps/app-123/files"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/apps/app-123/files"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    let app = client.app("app-123");

    let files = app.file("/").all_files("/").await.unwrap();
    assert!(!files.is_empty());

    let handle = app.file("/squarecloud_rs_test.txt");
    assert!(handle.write("hello from squarecloud-rs").await.unwrap());

    let content = handle.read("/squarecloud_rs_test.txt").await.unwrap();
    assert!(!content.data_type.is_empty());

    assert!(handle.move_to("/squarecloud_rs_test_moved.txt").await.unwrap());

    assert!(app.file("/squarecloud_rs_test_moved.txt").delete().await.unwrap());
}
