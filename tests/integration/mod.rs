use std::sync::{Once, OnceLock};

use squarecloud::Client;

mod account;
mod app;
mod blob;
mod client;
mod database;
pub mod helpers;
mod workspace;

static ENV: Once = Once::new();
static APP_ID: OnceLock<Result<String, String>> = OnceLock::new();
static DATABASE_ID: OnceLock<Result<String, String>> = OnceLock::new();

pub fn setup() {
    ENV.call_once(|| {
        dotenvy::from_filename(".env.test")
            .expect(".env.test not found — copy .env.test.example");
    });
}

/// Builds a `Client` authenticated with `API_TOKEN` from `.env.test`.
pub fn client() -> Client {
    setup();
    Client::new(std::env::var("API_TOKEN").expect(
        ".env.test not found or API_TOKEN unset — copy .env.test.example to .env.test",
    ))
}

/// Returns the shared dummy app ID, uploading it on first call.
///
/// Stores `Result` so a failed upload is cached: subsequent tests fail
/// immediately instead of retrying the upload and compounding rate limits.
pub fn shared_app_id() -> &'static str {
    APP_ID
        .get_or_init(|| {
            setup();
            std::thread::spawn(|| {
                tokio::runtime::Runtime::new()
                    .expect("failed to create tokio runtime for app upload")
                    .block_on(async {
                        crate::client()
                            .upload_app(helpers::dummy_zip())
                            .await
                            .map(|a| a.id)
                            .map_err(|e| format!("{e:?}"))
                    })
            })
            .join()
            .unwrap_or_else(|_| Err("upload thread panicked".to_string()))
        })
        .as_deref()
        .expect("shared app upload failed — check API token and rate limit")
}

/// Returns the shared app ID only if it was already initialized.
///
/// Used by the cleanup test to avoid uploading just to delete.
pub fn shared_app_id_if_initialized() -> Option<&'static str> {
    APP_ID.get().and_then(|r| r.as_deref().ok())
}

/// Returns the shared database ID, or `None` if creation failed.
///
/// Skips the test rather than panicking, since the plan may not support
/// database creation.
pub fn shared_database_id() -> Option<&'static str> {
    DATABASE_ID
        .get_or_init(|| {
            setup();
            std::thread::spawn(|| {
                tokio::runtime::Runtime::new()
                    .expect(
                        "failed to create tokio runtime for database create",
                    )
                    .block_on(async {
                        crate::client()
                            .create_database(
                                "squarecloud-rs-test".to_string(),
                                256,
                                squarecloud::DatabaseType::Postgres,
                                "16".to_string(),
                            )
                            .await
                            .map(|d| d.id)
                            .map_err(|e| {
                                eprintln!(
                                    "[database] create_database failed: {e:?}"
                                );
                                format!("{e:?}")
                            })
                    })
            })
            .join()
            .unwrap_or_else(|_| Err("database thread panicked".to_string()))
        })
        .as_deref()
        .ok()
}

pub fn shared_database_id_if_initialized() -> Option<&'static str> {
    DATABASE_ID.get().and_then(|r| r.as_deref().ok())
}

/// Waits briefly between tests to avoid hitting the SquareCloud rate limit.
pub async fn throttle() {
    tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
}
