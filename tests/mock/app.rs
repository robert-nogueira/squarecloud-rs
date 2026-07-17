use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use serde_json::json;
use squarecloud::{
    ApiError,
    errors::{AppErrorCode, NetworkErrorCode},
    types::AnalyticsFilters,
    types::DnsRecordType,
    types::RealtimeEvent,
};
use wiremock::matchers::{method, path, query_param};
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

    let info = client
        .app("app-123")
        .info()
        .await
        .expect("info() should succeed with mocked 200");
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

    let status = client
        .app("app-123")
        .status()
        .await
        .expect("status() should succeed with mocked running state");
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

    let status = client
        .app("app-123")
        .status()
        .await
        .expect("status() should succeed with mocked stopped state");
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

    let statuses = client
        .all_apps_status()
        .await
        .expect("all_apps_status() should succeed with mocked vec");
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
    let envs = std::collections::HashMap::from([(
        "TEST_KEY".to_string(),
        "hello".to_string(),
    )]);

    let after_upsert = app
        .upsert_envs(&envs)
        .await
        .expect("upsert_envs() should succeed with mocked 200");
    assert_eq!(
        after_upsert.get("TEST_KEY").map(String::as_str),
        Some("hello")
    );

    let listed = app
        .list_envs()
        .await
        .expect("list_envs() should succeed with mocked 200");
    assert!(listed.contains_key("TEST_KEY"));

    let overwrite = std::collections::HashMap::from([(
        "OTHER_KEY".to_string(),
        "world".to_string(),
    )]);
    let after_overwrite = app
        .overwrite_envs(&overwrite)
        .await
        .expect("overwrite_envs() should succeed with mocked 200");
    assert!(!after_overwrite.contains_key("TEST_KEY"));
    assert_eq!(
        after_overwrite.get("OTHER_KEY").map(String::as_str),
        Some("world")
    );

    let after_delete = app
        .delete_envs(&["OTHER_KEY".to_string()])
        .await
        .expect("delete_envs() should succeed with mocked 200");
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

    assert!(
        client
            .app("app-123")
            .commit(vec![])
            .await
            .expect("commit() should succeed with mocked 200")
    );
}

#[tokio::test]
async fn app_commit_to_sends_path_query_param() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/apps/app-123/commit"))
        .and(query_param("path", "src/routes"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    assert!(
        client
            .app("app-123")
            .commit_to(vec![], Some("src/routes"))
            .await
            .expect("commit_to() should succeed with mocked 200")
    );
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
        .and(query_param("start", "2026-07-08T00:00:00Z"))
        .and(query_param("end", "2026-07-09T00:00:00Z"))
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

    let start: DateTime<Utc> = "2026-07-08T00:00:00Z"
        .parse()
        .expect("hardcoded RFC3339 timestamp");
    let end: DateTime<Utc> = "2026-07-09T00:00:00Z"
        .parse()
        .expect("hardcoded RFC3339 timestamp");
    let analytics = client
        .app("app-123")
        .analytics(start, end)
        .await
        .expect("analytics() should succeed with mocked 200");
    assert_eq!(analytics.countries[0].item_type, "BR");
}

#[tokio::test]
async fn app_analytics_filtered_sends_filters_and_parses_new_breakdowns() {
    let (client, server) = crate::mock_client().await;
    let item = json!({
        "type": "404", "visits": 1, "requests": 2,
        "bytes": 512, "date": "2026-07-16"
    });
    Mock::given(method("GET"))
        .and(path("/apps/app-123/network/analytics"))
        .and(query_param("start", "2026-07-08T00:00:00Z"))
        .and(query_param("end", "2026-07-09T00:00:00Z"))
        .and(query_param("country", "BR"))
        .and(query_param("status", "404"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "visits": [item],
                "ips": [item],
                "status_codes": [item],
                "bots": [item],
                "content_types": [item]
            }
        })))
        .mount(&server)
        .await;

    let start: DateTime<Utc> = "2026-07-08T00:00:00Z"
        .parse()
        .expect("hardcoded RFC3339 timestamp");
    let end: DateTime<Utc> = "2026-07-09T00:00:00Z"
        .parse()
        .expect("hardcoded RFC3339 timestamp");
    let filters = AnalyticsFilters::new().country("BR").status("404");
    let analytics = client
        .app("app-123")
        .analytics_filtered(start, end, filters)
        .await
        .expect("analytics_filtered() should succeed with mocked 200");
    assert_eq!(analytics.ips.len(), 1);
    assert_eq!(analytics.status_codes[0].item_type, "404");
    assert_eq!(analytics.bots.len(), 1);
    assert_eq!(analytics.content_types[0].bytes, 512);
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

    let start: DateTime<Utc> = "2026-07-09T00:00:00Z"
        .parse()
        .expect("hardcoded RFC3339 timestamp");
    let end: DateTime<Utc> = "2026-07-08T00:00:00Z"
        .parse()
        .expect("hardcoded RFC3339 timestamp");
    match client.app("app-123").analytics(start, end).await {
        Err(ApiError::Service {
            code: NetworkErrorCode::InvalidTimeRange,
        }) => {}
        other => panic!("expected InvalidTimeRange, got {other:?}"),
    }
}

#[tokio::test]
async fn app_dns_records_returns_all_records() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/network/dns"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": [
                {
                    "type": "txt",
                    "name": "_cf-custom-hostname.example.com",
                    "value": "abc123-4567-89ab-cdef-0123456789ab",
                    "status": "pending"
                },
                {
                    "type": "txt",
                    "name": "_acme-challenge.example.com",
                    "value": "AbCdEfGhIjKlMnOpQrStUvWxYz",
                    "status": "pending_validation"
                },
                {
                    "type": "cname",
                    "name": "example.com",
                    "value": "cname.squareweb.app",
                    "status": "active"
                }
            ]
        })))
        .mount(&server)
        .await;

    let records = client
        .app("app-123")
        .dns_records()
        .await
        .expect("dns_records() should succeed with mocked 200");
    assert_eq!(records.len(), 3);
    assert_eq!(records[0].record_type, DnsRecordType::Txt);
    assert_eq!(records[2].record_type, DnsRecordType::Cname);
    assert_eq!(records[2].value, "cname.squareweb.app");
}

#[tokio::test]
async fn app_dns_records_empty_when_hostname_not_registered() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/network/dns"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": []
        })))
        .mount(&server)
        .await;

    let records = client
        .app("app-123")
        .dns_records()
        .await
        .expect("dns_records() should succeed with mocked 200");
    assert!(records.is_empty());
}

#[tokio::test]
async fn app_dns_records_no_custom_domain() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/network/dns"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "error", "code": "NO_CUSTOM_DOMAIN"
        })))
        .mount(&server)
        .await;

    match client.app("app-123").dns_records().await {
        Err(ApiError::Service {
            code: NetworkErrorCode::NoCustomDomain,
        }) => {}
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
        client
            .app("app-123")
            .set_custom_domain("example.com")
            .await
            .expect("set_custom_domain() should succeed with mocked 200")
    );
}

#[tokio::test]
async fn app_network_errors_returns_result() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/network/errors"))
        .and(query_param("start", "2026-07-08T00:00:00Z"))
        .and(query_param("end", "2026-07-09T00:00:00Z"))
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

    let start: DateTime<Utc> = "2026-07-08T00:00:00Z"
        .parse()
        .expect("hardcoded RFC3339 timestamp");
    let end: DateTime<Utc> = "2026-07-09T00:00:00Z"
        .parse()
        .expect("hardcoded RFC3339 timestamp");
    let result = client
        .app("app-123")
        .network_errors(false, start, end)
        .await;
    assert!(
        result.is_ok(),
        "network_errors() failed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn app_network_errors_include_4xx() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/network/errors"))
        .and(query_param("start", "2026-07-08T00:00:00Z"))
        .and(query_param("end", "2026-07-09T00:00:00Z"))
        .and(query_param("include_4xx", "true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "summary": { "total": 2, "by_class": { "4xx": 2, "5xx": 0 } },
                "by_status": [],
                "timeseries": [],
                "top_paths": [],
                "by_method": {}
            }
        })))
        .mount(&server)
        .await;

    let start: DateTime<Utc> = "2026-07-08T00:00:00Z"
        .parse()
        .expect("hardcoded RFC3339 timestamp");
    let end: DateTime<Utc> = "2026-07-09T00:00:00Z"
        .parse()
        .expect("hardcoded RFC3339 timestamp");
    let result = client.app("app-123").network_errors(true, start, end).await;
    assert!(
        result.is_ok(),
        "network_errors(true) failed: {:?}",
        result.err()
    );
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

    let start: DateTime<Utc> = "2026-07-09T00:00:00Z"
        .parse()
        .expect("hardcoded RFC3339 timestamp");
    let end: DateTime<Utc> = "2026-07-08T00:00:00Z"
        .parse()
        .expect("hardcoded RFC3339 timestamp");
    match client
        .app("app-123")
        .network_errors(false, start, end)
        .await
    {
        Err(ApiError::Service {
            code: NetworkErrorCode::InvalidTimeRange,
        }) => {}
        other => panic!("expected InvalidTimeRange, got {other:?}"),
    }
}

#[tokio::test]
async fn app_network_logs_returns_vec() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/network/logs"))
        .and(query_param("start", "2026-07-08T00:00:00Z"))
        .and(query_param("end", "2026-07-09T00:00:00Z"))
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

    let start: DateTime<Utc> = "2026-07-08T00:00:00Z"
        .parse()
        .expect("hardcoded RFC3339 timestamp");
    let end: DateTime<Utc> = "2026-07-09T00:00:00Z"
        .parse()
        .expect("hardcoded RFC3339 timestamp");
    let result = client.app("app-123").network_logs(start, end).await;
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

    let start: DateTime<Utc> = "2026-07-09T00:00:00Z"
        .parse()
        .expect("hardcoded RFC3339 timestamp");
    let end: DateTime<Utc> = "2026-07-08T00:00:00Z"
        .parse()
        .expect("hardcoded RFC3339 timestamp");
    match client.app("app-123").network_logs(start, end).await {
        Err(ApiError::Service {
            code: NetworkErrorCode::InvalidTimeRange,
        }) => {}
        other => panic!("expected InvalidTimeRange, got {other:?}"),
    }
}

#[tokio::test]
async fn app_network_performance_returns_result() {
    let (client, server) = crate::mock_client().await;
    let p = json!({ "p50": 10, "p95": 50, "p99": 100 });
    Mock::given(method("GET"))
        .and(path("/apps/app-123/network/performance"))
        .and(query_param("start", "2026-07-08T00:00:00Z"))
        .and(query_param("end", "2026-07-09T00:00:00Z"))
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

    let start: DateTime<Utc> = "2026-07-08T00:00:00Z"
        .parse()
        .expect("hardcoded RFC3339 timestamp");
    let end: DateTime<Utc> = "2026-07-09T00:00:00Z"
        .parse()
        .expect("hardcoded RFC3339 timestamp");
    let result = client.app("app-123").network_performance(start, end).await;
    assert!(
        result.is_ok(),
        "network_performance() failed: {:?}",
        result.err()
    );
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

    let start: DateTime<Utc> = "2026-07-09T00:00:00Z"
        .parse()
        .expect("hardcoded RFC3339 timestamp");
    let end: DateTime<Utc> = "2026-07-08T00:00:00Z"
        .parse()
        .expect("hardcoded RFC3339 timestamp");
    match client.app("app-123").network_performance(start, end).await {
        Err(ApiError::Service {
            code: NetworkErrorCode::InvalidTimeRange,
        }) => {}
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

    assert!(
        client
            .app("app-123")
            .purge_cache()
            .await
            .expect("purge_cache() should succeed with mocked 200")
    );
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
        Err(ApiError::Service {
            code: NetworkErrorCode::KeepCalm,
        }) => {}
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

    let _ = client
        .app("app-123")
        .metrics()
        .await
        .expect("metrics() should succeed with mocked 200");
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

    assert!(
        client
            .app("app-123")
            .restart()
            .await
            .expect("restart() should succeed with mocked 200")
    );
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

    assert!(
        client
            .app("app-123")
            .start()
            .await
            .expect("start() should succeed with mocked 200")
    );
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
        Err(ApiError::Service {
            code: AppErrorCode::ContainerAlreadyStarted,
        }) => {}
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

    assert!(
        client
            .app("app-123")
            .stop()
            .await
            .expect("stop() should succeed with mocked 200")
    );
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

    assert!(
        client
            .app("app-123")
            .delete()
            .await
            .expect("delete() should succeed with mocked 200")
    );
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

    let deploy = client
        .app("app-123")
        .current_deploy()
        .await
        .expect("current_deploy() should succeed with mocked 200");
    assert_eq!(deploy.id.as_deref(), Some("deploy-1"));
    assert_eq!(deploy.state.as_deref(), Some("complete"));
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

    let deploys = client
        .app("app-123")
        .list_deploys()
        .await
        .expect("list_deploys() should succeed with mocked 200");
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

    let url = client
        .app("app-123")
        .set_webhook_integration("gh_token".to_string())
        .await
        .expect("set_webhook_integration() should succeed with mocked 200");
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
                    "key": "AWSAccessKeyId=FAKE&Expires=9999&Signature=sig&versionId=dead-beef-0000"
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

    let snap = app
        .create_snapshot()
        .await
        .expect("create_snapshot() should succeed with mocked 200");
    assert!(!snap.url.is_empty());
    assert!(!snap.key.is_empty());

    let snapshots = app
        .list_snapshots()
        .await
        .expect("list_snapshots() should succeed with mocked 200");
    assert!(!snapshots.is_empty());

    let first = &snapshots[0];
    assert!(
        app.restore_snapshot(
            first.name.clone(),
            first.version_id().to_string()
        )
        .await
        .expect("restore_snapshot() should succeed with mocked 200")
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

    let files = app
        .file("/")
        .all_files("/")
        .await
        .expect("all_files() should succeed with mocked 200");
    assert!(!files.is_empty());

    let handle = app.file("/squarecloud_rs_test.txt");
    assert!(
        handle
            .write("hello from squarecloud-rs")
            .await
            .expect("write() should succeed with mocked 200")
    );

    let content = handle
        .read("/squarecloud_rs_test.txt")
        .await
        .expect("read() should succeed with mocked 200");
    assert!(!content.data_type.is_empty());

    assert!(
        handle
            .move_to("/squarecloud_rs_test_moved.txt")
            .await
            .expect("move_to() should succeed with mocked 200")
    );

    assert!(
        app.file("/squarecloud_rs_test_moved.txt")
            .delete()
            .await
            .expect("delete() should succeed with mocked 200")
    );
}

#[tokio::test]
async fn realtime_parses_log_event() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/realtime"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "text/event-stream")
                .set_body_bytes(
                    b"event: system\ndata: REALTIME_CONNECTED\n\n\
                      event: logs\ndata: hello from app\n\n"
                        as &[u8],
                ),
        )
        .mount(&server)
        .await;

    let events: Vec<_> = client.app("app-123").realtime().collect().await;

    assert_eq!(events.len(), 2);
    assert!(matches!(events[0], Ok(RealtimeEvent::System(_))));
    assert!(matches!(events[1], Ok(RealtimeEvent::Log(_))));
    if let Ok(RealtimeEvent::Log(msg)) = &events[1] {
        assert_eq!(msg, "hello from app");
    }
}

#[tokio::test]
async fn realtime_multiline_data_is_concatenated() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/realtime"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "text/event-stream")
                .set_body_bytes(
                    b"event: logs\ndata: line one\ndata: line two\n\n"
                        as &[u8],
                ),
        )
        .mount(&server)
        .await;

    let events: Vec<_> = client.app("app-123").realtime().collect().await;

    assert_eq!(events.len(), 1);
    if let Ok(RealtimeEvent::Log(msg)) = &events[0] {
        assert_eq!(msg, "line one\nline two");
    }
}

#[tokio::test]
async fn realtime_comments_are_ignored() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/realtime"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "text/event-stream")
                .set_body_bytes(
                    b": ping\n\nevent: logs\ndata: hello\n\n" as &[u8],
                ),
        )
        .mount(&server)
        .await;

    let events: Vec<_> = client.app("app-123").realtime().collect().await;

    assert_eq!(events.len(), 1);
    assert!(matches!(events[0], Ok(RealtimeEvent::Log(_))));
}
