use squarecloud::{ApiClient, types::CredentialType};

macro_rules! require_db {
    () => {
        match crate::shared_database_id() {
            Some(id) => id,
            None => {
                eprintln!("Skipping: database not available on this plan");
                return;
            }
        }
    };
}

#[tokio::test]
async fn database_info_returns_info() {
    crate::setup();
    crate::throttle().await;
    let db_id = require_db!();
    let info = ApiClient::new()
        .database(db_id)
        .info()
        .await
        .expect("database info() should succeed");
    assert_eq!(info.id, db_id);
    assert!(!info.name.is_empty());
}

#[tokio::test]
async fn database_status_returns_runtime_stats() {
    crate::setup();
    crate::throttle().await;
    let db_id = require_db!();
    let status = ApiClient::new()
        .database(db_id)
        .status()
        .await
        .expect("database status() should succeed");
    assert!(!status.cpu.is_empty());
    assert!(!status.ram.is_empty());
    assert!(!status.status.is_empty());
}

#[tokio::test]
async fn database_metrics_returns_vec() {
    crate::setup();
    crate::throttle().await;
    let db_id = require_db!();
    let result = ApiClient::new().database(db_id).metrics().await;
    assert!(result.is_ok(), "metrics() failed: {:?}", result.err());
}

#[tokio::test]
async fn database_edit_name() {
    crate::setup();
    crate::throttle().await;
    let db_id = require_db!();
    let result = ApiClient::new()
        .database(db_id)
        .edit(Some("squarecloud-rs-test"), None)
        .await;
    assert!(result.is_ok(), "edit(name) failed: {:?}", result.err());
    assert!(result.expect("edit(name) should return true"));
    let result = ApiClient::new().database(db_id).edit(None, Some(256)).await;
    assert!(result.is_ok(), "edit(ram) failed: {:?}", result.err());
    assert!(result.expect("edit(ram) should return true"));
}

#[tokio::test]
async fn database_edit_none_returns_false() {
    crate::setup();
    crate::throttle().await;
    let db_id = require_db!();
    let result = ApiClient::new().database(db_id).edit(None, None).await;
    assert_eq!(
        result.expect("edit(None, None) should return Ok(false)"),
        false
    );
}

#[tokio::test]
async fn database_certificate_returns_string() {
    crate::setup();
    crate::throttle().await;
    let db_id = require_db!();
    let result = ApiClient::new().database(db_id).certificate().await;
    assert!(result.is_ok(), "certificate() failed: {:?}", result.err());
    assert!(
        !result
            .expect("certificate() should return non-empty string")
            .is_empty()
    );
}

#[tokio::test]
async fn database_redefine_credential_password() {
    crate::setup();
    crate::throttle().await;
    let db_id = require_db!();
    let result = ApiClient::new()
        .database(db_id)
        .redefine_credential(CredentialType::Password)
        .await;
    assert!(
        result.is_ok(),
        "redefine_credential(Password) failed: {:?}",
        result.err()
    );
    assert!(
        !result
            .expect(
                "redefine_credential(Password) should return new credential"
            )
            .is_empty()
    );
}

#[tokio::test]
async fn database_redefine_credential_certificate() {
    crate::setup();
    crate::throttle().await;
    let db_id = require_db!();
    let result = ApiClient::new()
        .database(db_id)
        .redefine_credential(CredentialType::Certificate)
        .await;
    assert!(
        result.is_ok(),
        "redefine_credential(Certificate) failed: {:?}",
        result.err()
    );
    assert!(
        !result
            .expect(
                "redefine_credential(Certificate) should return certificate"
            )
            .is_empty()
    );
}

#[tokio::test]
async fn database_snapshot_lifecycle() {
    crate::setup();
    crate::throttle().await;
    let db_id = require_db!();
    let db = ApiClient::new().database(db_id);

    let snap = db
        .create_snapshot()
        .await
        .expect("create_snapshot() should succeed");
    assert!(!snap.url.is_empty());
    assert!(!snap.key.is_empty());

    crate::throttle().await;
    let snapshots = db
        .list_snapshots()
        .await
        .expect("list_snapshots() should return snapshots after create");
    assert!(!snapshots.is_empty());

    let first = &snapshots[0];
    crate::throttle().await;
    assert!(
        db.restore_snapshot(
            first.name.clone(),
            first.version_id().to_string(),
        )
        .await
        .expect("restore_snapshot() should succeed")
    );
}

#[tokio::test]
async fn database_start_returns_true() {
    crate::setup();
    crate::throttle().await;
    let db_id = require_db!();
    let result = ApiClient::new().database(db_id).start().await;
    assert!(result.is_ok(), "start() failed: {:?}", result.err());
    assert!(result.expect("database start() should return true"));
}

#[tokio::test]
async fn database_stop_returns_true() {
    crate::setup();
    crate::throttle().await;
    let db_id = require_db!();
    let result = ApiClient::new().database(db_id).stop().await;
    assert!(result.is_ok(), "stop() failed: {:?}", result.err());
    assert!(result.expect("database stop() should return true"));
}

/// Must stay last alphabetically so it runs after all other database tests.
#[tokio::test]
async fn z_cleanup_shared_database() {
    if let Some(id) = crate::shared_database_id_if_initialized() {
        ApiClient::new()
            .database(id)
            .delete()
            .await
            .expect("database delete() should succeed on cleanup");
    }
}
