use chrono::{DateTime, Utc};
use serde_json::json;
use squarecloud::errors::{ApiError, DatabaseErrorCode, SnapshotErrorCode};
use squarecloud::types::{CredentialType, RestoreState, SnapshotOrigin};
use wiremock::matchers::{body_json, method, path};
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

    let info = client
        .database("db-123")
        .info()
        .await
        .expect("database info() should succeed with mocked 200");
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

    let status = client
        .database("db-123")
        .status()
        .await
        .expect("database status() should succeed with mocked 200");
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
    assert_eq!(result.expect("metrics() should return vec").len(), 1);
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
    assert!(result.expect("edit(name) should return true"));
}

#[tokio::test]
async fn database_edit_ram() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("PATCH"))
        .and(path("/databases/db-123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    let result = client.database("db-123").edit(None, Some(512)).await;
    assert!(result.is_ok(), "edit(ram) failed: {:?}", result.err());
    assert!(result.expect("edit(ram) should return true"));
}

#[tokio::test]
async fn database_edit_none_returns_false() {
    let (client, _server) = crate::mock_client().await;

    let result = client.database("db-123").edit(None, None).await;
    assert_eq!(
        result.expect("edit(None, None) should return Ok(false)"),
        false
    );
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
    assert!(
        !result
            .expect("certificate() should return non-empty string")
            .is_empty()
    );
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
    assert_eq!(
        result.expect(
            "redefine_credential(Password) should return new password"
        ),
        "new-secret-password"
    );
}

#[tokio::test]
async fn database_redefine_certificate_reports_failed_fetch_after_reset() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/databases/db-123/credentials/reset"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/databases/db-123/credentials/certificate"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "status": "error",
            "code": "DATABASE_NOT_RUNNING"
        })))
        .mount(&server)
        .await;

    match client
        .database("db-123")
        .redefine_credential(CredentialType::Certificate)
        .await
    {
        Err(ApiError::Service {
            code: DatabaseErrorCode::DatabaseNotRunning,
        }) => {}
        other => panic!("expected DatabaseNotRunning, got {other:?}"),
    }
}

#[tokio::test]
async fn database_redefine_credential_certificate() {
    let (client, server) = crate::mock_client().await;
    // The reset answers without the certificate, which is then fetched.
    Mock::given(method("POST"))
        .and(path("/databases/db-123/credentials/reset"))
        .and(body_json(json!({ "reset": "certificate" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/databases/db-123/credentials/certificate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "certificate": "-----BEGIN CERTIFICATE-----\nNEW\n-----END CERTIFICATE-----"
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let certificate = client
        .database("db-123")
        .redefine_credential(CredentialType::Certificate)
        .await
        .expect("redefine_credential(Certificate) should succeed");
    assert!(certificate.contains("NEW"));
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
                    "key": "AWSAccessKeyId=FAKE&Expires=9999&Signature=sig",
                    "version_id": "dead-beef-0000",
                    "url": "https://snapshots.squarecloud.app/db-123.zip?sig",
                    "runtime": "postgres",
                    "origin": "manual"
                }
            ]
        })))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/databases/db-123/snapshots/restore"))
        .and(body_json(json!({
            "snapshotId": "db-123",
            "versionId": "dead-beef-0000"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    let db = client.database("db-123");

    let snap = db
        .create_snapshot()
        .await
        .expect("create_snapshot() should succeed with mocked 200");
    assert_eq!(snap.url, "https://storage.example.com/snap.tar.gz");
    assert!(!snap.key.is_empty());

    let snapshots = db
        .list_snapshots()
        .await
        .expect("list_snapshots() should succeed with mocked 200");
    assert_eq!(snapshots.len(), 1);

    let first = &snapshots[0];
    assert_eq!(first.version_id, "dead-beef-0000");
    assert_eq!(first.runtime.as_deref(), Some("postgres"));
    assert_eq!(first.origin, Some(SnapshotOrigin::Manual));
    assert!(
        db.restore_snapshot(first.name.clone(), first.version_id.clone())
            .await
            .expect("restore_snapshot() should succeed with mocked 200")
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
    assert!(result.expect("database start() should return true"));
}

#[tokio::test]
async fn database_restore_status_reports_failure_code() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/databases/db-123/snapshots/restore"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "state": "error",
                "step": "unzip",
                "code": "SNAPSHOT_CORRUPT",
                "started_at": 1760900000000_i64,
                "finished_at": 1760900060000_i64
            }
        })))
        .mount(&server)
        .await;

    let status = client
        .database("db-123")
        .restore_status()
        .await
        .expect("restore_status() should succeed with mocked 200");
    assert_eq!(status.state, RestoreState::Error);
    assert_eq!(status.code, Some(SnapshotErrorCode::SnapshotCorrupt));
    assert_eq!(
        status.started_at,
        DateTime::<Utc>::from_timestamp_millis(1_760_900_000_000)
    );
    assert_eq!(
        status.finished_at,
        DateTime::<Utc>::from_timestamp_millis(1_760_900_060_000)
    );
}

#[tokio::test]
async fn database_restore_status_without_recent_restore() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/databases/db-123/snapshots/restore"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": { "state": "none" }
        })))
        .mount(&server)
        .await;

    let status = client
        .database("db-123")
        .restore_status()
        .await
        .expect("restore_status() should accept a bare state");
    assert_eq!(status.state, RestoreState::None);
    assert_eq!(status.step, None);
    assert_eq!(status.started_at, None);
}

#[tokio::test]
async fn database_start_already_started_returns_service_error() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/databases/db-123/start"))
        .respond_with(ResponseTemplate::new(409).set_body_json(json!({
            "status": "error",
            "code": "CONTAINER_ALREADY_STARTED"
        })))
        .mount(&server)
        .await;

    match client.database("db-123").start().await {
        Err(ApiError::Service {
            code: DatabaseErrorCode::ContainerAlreadyStarted,
        }) => {}
        other => {
            panic!("expected ContainerAlreadyStarted, got {other:?}")
        }
    }
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
    assert!(result.expect("database stop() should return true"));
}

#[tokio::test]
async fn database_delete_returns_true() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("DELETE"))
        .and(path("/databases/db-123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    let result = client.database("db-123").delete().await;
    assert!(result.is_ok(), "delete() failed: {:?}", result.err());
    assert!(result.expect("database delete() should return true"));
}
