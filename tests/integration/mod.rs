use std::sync::{Once, OnceLock};

use squarecloud_rs::ApiClient;

mod account;
mod app;
pub mod helpers;

static ENV: Once = Once::new();
static APP_ID: OnceLock<Result<String, String>> = OnceLock::new();

pub fn setup() {
    ENV.call_once(|| {
        dotenvy::from_filename(".env.test")
            .expect(".env.test not found — copy .env.test.example");
    });
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
                tokio::runtime::Runtime::new().unwrap().block_on(async {
                    ApiClient::new()
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

/// Waits briefly between tests to avoid hitting the SquareCloud rate limit.
pub async fn throttle() {
    tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
}
