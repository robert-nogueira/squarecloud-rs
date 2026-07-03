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
