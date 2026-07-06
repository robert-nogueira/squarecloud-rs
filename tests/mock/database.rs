use serde_json::json;
use squarecloud_rs::types::CredentialType;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn database_info_returns_info() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/databases/db-123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "id": "db-123",
                "name": "my-db",
                "owner": "user-123",
                "cluster": "florida-1",
                "ram": 256,
                "type": "postgresql",
                "port": 5432,
                "created_at": "2024-01-01T00:00:00Z"
            }
        })))
        .mount(&server)
        .await;

    let info = client.database("db-123").info().await.unwrap();
    assert_eq!(info.id, "db-123");
    assert_eq!(info.name, "my-db");
    assert_eq!(info.ram, 256);
}

#[tokio::test]
async fn database_status_returns_runtime_stats() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/databases/db-123/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "cpu": "1.5%",
                "ram": "64/256MB",
                "status": "running",
                "running": true,
                "storage": "10MB",
                "network": { "total": "1MB", "now": "0KB" },
                "uptime": null
            }
        })))
        .mount(&server)
        .await;

    let status = client.database("db-123").status().await.unwrap();
    assert!(!status.cpu.is_empty());
    assert!(!status.ram.is_empty());
    assert!(!status.status.is_empty());
}

#[tokio::test]
async fn database_metrics_returns_vec() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/databases/db-123/metrics"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": [
                {
                    "date": "2024-01-01T00:00:00Z",
                    "cpu": 1.5,
                    "ram": 25.0,
                    "net": [1000, 2000]
                }
            ]
        })))
        .mount(&server)
        .await;

    let result = client.database("db-123").metrics().await;
    assert!(result.is_ok(), "metrics() failed: {:?}", result.err());
    assert_eq!(result.unwrap().len(), 1);
}

#[tokio::test]
async fn database_edit_name() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("PATCH"))
        .and(path("/databases/db-123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    let result = client.database("db-123").edit(Some("new-name"), None).await;
    assert!(result.is_ok(), "edit() failed: {:?}", result.err());
    assert!(result.unwrap());
}

#[tokio::test]
async fn database_edit_none_returns_false() {
    let (client, _server) = crate::mock_client().await;

    let result = client.database("db-123").edit(None, None).await;
    assert_eq!(result.unwrap(), false);
}

#[tokio::test]
async fn database_certificate_returns_string() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/databases/db-123/credentials/certificate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "certificate": "-----BEGIN CERTIFICATE-----\nMIIB...\n-----END CERTIFICATE-----"
            }
        })))
        .mount(&server)
        .await;

    let result = client.database("db-123").certificate().await;
    assert!(result.is_ok(), "certificate() failed: {:?}", result.err());
    assert!(!result.unwrap().is_empty());
}

#[tokio::test]
async fn database_redefine_credential_password() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/databases/db-123/credentials/reset"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "password": "new-secret-password"
            }
        })))
        .mount(&server)
        .await;

    let result = client
        .database("db-123")
        .redefine_credential(CredentialType::Password)
        .await;
    assert!(
        result.is_ok(),
        "redefine_credential(Password) failed: {:?}",
        result.err()
    );
    assert_eq!(result.unwrap(), "new-secret-password");
}

#[tokio::test]
async fn database_redefine_credential_certificate() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/databases/db-123/credentials/reset"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "certificate": "-----BEGIN CERTIFICATE-----\nMIIB...\n-----END CERTIFICATE-----"
            }
        })))
        .mount(&server)
        .await;

    let result = client
        .database("db-123")
        .redefine_credential(CredentialType::Certificate)
        .await;
    assert!(
        result.is_ok(),
        "redefine_credential(Certificate) failed: {:?}",
        result.err()
    );
    assert!(!result.unwrap().is_empty());
}

#[tokio::test]
async fn database_snapshot_lifecycle() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/databases/db-123/snapshots"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "url": "https://storage.example.com/snap.tar.gz",
                "key": "AWSAccessKeyId=FAKE&Expires=9999&Signature=sig&versionId=dead-beef-0000"
            }
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/databases/db-123/snapshots"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": [
                {
                    "name": "db-123",
                    "size": 2048,
                    "modified": "2024-01-01T00:00:00Z",
                    "key": "AWSAccessKeyId=FAKE&Expires=9999&Signature=sig&versionId=dead-beef-0000"
                }
            ]
        })))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/databases/db-123/snapshots/restore"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    let db = client.database("db-123");

    let snap = db.create_snapshot().await.unwrap();
    assert_eq!(snap.url, "https://storage.example.com/snap.tar.gz");
    assert!(!snap.key.is_empty());

    let snapshots = db.list_snapshots().await.unwrap();
    assert_eq!(snapshots.len(), 1);

    let first = &snapshots[0];
    assert!(
        db.restore_snapshot(
            first.name.clone(),
            first.version_id().to_string()
        )
        .await
        .unwrap()
    );
}

#[tokio::test]
async fn database_start_returns_true() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/databases/db-123/start"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    let result = client.database("db-123").start().await;
    assert!(result.is_ok(), "start() failed: {:?}", result.err());
    assert!(result.unwrap());
}

#[tokio::test]
async fn database_stop_returns_true() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/databases/db-123/stop"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    let result = client.database("db-123").stop().await;
    assert!(result.is_ok(), "stop() failed: {:?}", result.err());
    assert!(result.unwrap());
}

#[tokio::test]
async fn database_delete_returns_true() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("DELETE"))
        .and(path("/databases/db-123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": true
        })))
        .mount(&server)
        .await;

    let result = client.database("db-123").delete().await;
    assert!(result.is_ok(), "delete() failed: {:?}", result.err());
    assert!(result.unwrap());
}
