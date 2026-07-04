use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn app_info_deserializes_success_response() {
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
    assert_eq!(info.language, "javascript");
}

#[tokio::test]
async fn app_status_deserializes_success_response() {
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
    assert_eq!(status.cpu, "2.5%");
    assert_eq!(status.ram, "128/512MB");
    assert_eq!(status.status, "running");
    assert!(status.running);
    assert!(status.uptime.is_some());
}

#[tokio::test]
async fn app_logs_deserializes_success_response() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/logs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "logs": "Server listening on :3000\nRequest received\n"
            }
        })))
        .mount(&server)
        .await;

    let logs = client.app("app-123").logs().await.unwrap();
    assert!(logs.contains("Server listening"));
}

#[tokio::test]
async fn all_apps_status_deserializes_success_response() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": [
                { "id": "app-123", "running": true, "cpu": "1.2%", "ram": "100MB" },
                { "id": "app-456", "running": false, "cpu": null, "ram": null }
            ]
        })))
        .mount(&server)
        .await;

    let statuses = client.all_apps_status().await.unwrap();
    assert_eq!(statuses.len(), 2);
    assert_eq!(statuses[0].id, "app-123");
    assert!(statuses[0].running);
    assert_eq!(statuses[1].id, "app-456");
    assert!(!statuses[1].running);
    assert!(statuses[1].cpu.is_none());
}

#[tokio::test]
async fn app_list_envs_deserializes_success_response() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/envs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": { "KEY": "value" }
        })))
        .mount(&server)
        .await;

    let envs = client.app("app-123").list_envs().await.unwrap();
    assert_eq!(envs.get("KEY").map(String::as_str), Some("value"));
}

#[tokio::test]
async fn app_upsert_envs_deserializes_success_response() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/apps/app-123/envs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": { "KEY": "value" }
        })))
        .mount(&server)
        .await;

    let envs = std::collections::HashMap::from([("KEY".to_string(), "value".to_string())]);
    let result = client.app("app-123").upsert_envs(&envs).await.unwrap();
    assert_eq!(result.get("KEY").map(String::as_str), Some("value"));
}

#[tokio::test]
async fn app_overwrite_envs_deserializes_success_response() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("PUT"))
        .and(path("/apps/app-123/envs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": { "NEW_KEY": "new_value" }
        })))
        .mount(&server)
        .await;

    let envs = std::collections::HashMap::from([("NEW_KEY".to_string(), "new_value".to_string())]);
    let result = client.app("app-123").overwrite_envs(&envs).await.unwrap();
    assert!(!result.contains_key("KEY"));
    assert_eq!(result.get("NEW_KEY").map(String::as_str), Some("new_value"));
}

#[tokio::test]
async fn app_delete_envs_deserializes_success_response() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("DELETE"))
        .and(path("/apps/app-123/envs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {}
        })))
        .mount(&server)
        .await;

    let result = client.app("app-123").delete_envs(&["KEY".to_string()]).await.unwrap();
    assert!(result.is_empty());
}

#[tokio::test]
async fn app_commit_deserializes_success_response() {
    use wiremock::matchers::method;
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
async fn app_metrics_deserializes_success_response() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/metrics"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": [
                { "date": "2024-01-01T00:00:00Z", "cpu": 1.2, "ram": 128.0, "net": [1000, 2000] }
            ]
        })))
        .mount(&server)
        .await;

    let metrics = client.app("app-123").metrics().await.unwrap();
    assert_eq!(metrics.len(), 1);
    assert_eq!(metrics[0].net, [1000, 2000]);
}

#[tokio::test]
async fn app_restart_deserializes_success_response() {
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
async fn app_start_deserializes_success_response() {
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
async fn app_stop_deserializes_success_response() {
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
async fn app_analytics_deserializes_success_response() {
    let (client, server) = crate::mock_client().await;
    let item = json!({
        "type": "BR", "visits": 10, "requests": 20, "bytes": 1024,
        "date": "2024-01-01"
    });
    Mock::given(method("GET"))
        .and(path("/apps/app-123/network/analytics"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "visits":    [item],
                "countries": [item],
                "devices":   [item],
                "os":        [item],
                "browsers":  [item],
                "protocols": [item],
                "methods":   [item],
                "paths":     [item],
                "referers":  [item],
                "providers": [item]
            }
        })))
        .mount(&server)
        .await;

    let analytics = client.app("app-123").analytics().await.unwrap();
    assert_eq!(analytics.countries[0].item_type, "BR");
    assert_eq!(analytics.visits[0].requests, 20);
}

#[tokio::test]
async fn app_dns_record_deserializes_success_response() {
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
    assert_eq!(record.name, "example.com");
    assert_eq!(record.value, "1.2.3.4");
}

#[tokio::test]
async fn app_set_custom_domain_deserializes_success_response() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/apps/app-123/network/custom"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    assert!(client.app("app-123").set_custom_domain("example.com").await.unwrap());
}

#[tokio::test]
async fn app_network_errors_deserializes_success_response() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-123/network/errors"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "summary": {
                    "total": 5,
                    "by_class": { "5xx": 5 }
                },
                "by_status": [{ "status": 502, "total": 5 }],
                "timeseries": [],
                "top_paths": [],
                "by_method": {}
            }
        })))
        .mount(&server)
        .await;

    let errors = client.app("app-123").network_errors(false).await.unwrap();
    assert_eq!(errors.summary.total, 5);
    assert_eq!(errors.summary.by_class.server_errors, 5);
    assert!(errors.summary.by_class.client_errors.is_none());
}

#[tokio::test]
async fn app_network_logs_deserializes_success_response() {
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

    let logs = client.app("app-123").network_logs().await.unwrap();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].method, "GET");
    assert_eq!(logs[0].status, 200);
    assert!(!logs[0].mitigated);
}

#[tokio::test]
async fn app_network_performance_deserializes_success_response() {
    let (client, server) = crate::mock_client().await;
    let percentiles = json!({ "p50": 10, "p95": 50, "p99": 100 });
    Mock::given(method("GET"))
        .and(path("/apps/app-123/network/performance"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "summary": {
                    "edge": percentiles,
                    "origin": percentiles,
                    "requests": 1000
                },
                "timeseries": [],
                "countries": [],
                "colos": [],
                "slowest_paths": []
            }
        })))
        .mount(&server)
        .await;

    let perf = client.app("app-123").network_performance().await.unwrap();
    assert_eq!(perf.summary.requests, 1000);
    assert_eq!(perf.summary.edge.p50, 10);
    assert_eq!(perf.summary.origin.p99, 100);
}

#[tokio::test]
async fn app_purge_cache_deserializes_success_response() {
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
async fn app_delete_deserializes_success_response() {
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
async fn app_status_deserializes_stopped_response() {
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
