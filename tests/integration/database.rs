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
    let info = ApiClient::new().database(db_id).info().await.unwrap();
    assert_eq!(info.id, db_id);
    assert!(!info.name.is_empty());
}

#[tokio::test]
async fn database_status_returns_runtime_stats() {
    crate::setup();
    crate::throttle().await;
    let db_id = require_db!();
    let status = ApiClient::new().database(db_id).status().await.unwrap();
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
    assert!(result.unwrap());
    let result = ApiClient::new().database(db_id).edit(None, Some(256)).await;
    assert!(result.is_ok(), "edit(ram) failed: {:?}", result.err());
    assert!(result.unwrap());
}

#[tokio::test]
async fn database_edit_none_returns_false() {
    crate::setup();
    crate::throttle().await;
    let db_id = require_db!();
    let result = ApiClient::new().database(db_id).edit(None, None).await;
    assert_eq!(result.unwrap(), false);
}

#[tokio::test]
async fn database_certificate_returns_string() {
    crate::setup();
    crate::throttle().await;
    let db_id = require_db!();
    let result = ApiClient::new().database(db_id).certificate().await;
    assert!(result.is_ok(), "certificate() failed: {:?}", result.err());
    assert!(!result.unwrap().is_empty());
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
    assert!(!result.unwrap().is_empty());
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
    assert!(!result.unwrap().is_empty());
}

#[tokio::test]
async fn database_snapshot_lifecycle() {
    crate::setup();
    crate::throttle().await;
    let db_id = require_db!();
    let db = ApiClient::new().database(db_id);

    let snap = db.create_snapshot().await.unwrap();
    assert!(!snap.url.is_empty());
    assert!(!snap.key.is_empty());

    crate::throttle().await;
    let snapshots = db.list_snapshots().await.unwrap();
    assert!(!snapshots.is_empty());

    let first = &snapshots[0];
    crate::throttle().await;
    assert!(
        db.restore_snapshot(
            first.name.clone(),
            first.version_id().to_string(),
        )
        .await
        .unwrap()
    );
}

#[tokio::test]
async fn database_start_returns_true() {
    crate::setup();
    crate::throttle().await;
    let db_id = require_db!();
    let result = ApiClient::new().database(db_id).start().await;
    assert!(result.is_ok(), "start() failed: {:?}", result.err());
    assert!(result.unwrap());
}

#[tokio::test]
async fn database_stop_returns_true() {
    crate::setup();
    crate::throttle().await;
    let db_id = require_db!();
    let result = ApiClient::new().database(db_id).stop().await;
    assert!(result.is_ok(), "stop() failed: {:?}", result.err());
    assert!(result.unwrap());
}

/// Must stay last alphabetically so it runs after all other database tests.
#[tokio::test]
async fn z_cleanup_shared_database() {
    if let Some(id) = crate::shared_database_id_if_initialized() {
        ApiClient::new().database(id).delete().await.unwrap();
    }
}
