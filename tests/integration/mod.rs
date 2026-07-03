use std::sync::{Once, OnceLock};

use squarecloud_rs::ApiClient;

mod account;
mod app;
pub mod helpers;

static ENV: Once = Once::new();
static APP_ID: OnceLock<String> = OnceLock::new();

pub fn setup() {
    ENV.call_once(|| {
        dotenvy::from_filename(".env.test")
            .expect(".env.test not found — copy .env.test.example");
    });
}

/// Returns the shared dummy app ID, uploading it on first call.
///
/// Uses a blocking tokio runtime so the upload runs exactly once across
/// all test threads, regardless of how many tests call this concurrently.
pub fn shared_app_id() -> &'static str {
    APP_ID.get_or_init(|| {
        setup();
        // Spawn a dedicated thread so we can create a fresh tokio runtime
        // without conflicting with the one already running inside each
        // #[tokio::test] context.
        std::thread::spawn(|| {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async {
                    ApiClient::new()
                        .upload_app(helpers::dummy_zip())
                        .await
                        .expect("failed to upload shared test app")
                        .id
                })
        })
        .join()
        .unwrap()
    })
}

/// Returns the shared app ID only if it was already initialized.
///
/// Used by the cleanup test to avoid uploading just to delete.
pub fn shared_app_id_if_initialized() -> Option<&'static str> {
    APP_ID.get().map(String::as_str)
}
