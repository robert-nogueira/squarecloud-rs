use std::sync::Once;

use squarecloud_rs::ApiClient;
use tokio::sync::OnceCell;

mod account;
mod app;
pub mod helpers;

static ENV: Once = Once::new();
static APP_ID: OnceCell<String> = OnceCell::const_new();

pub fn setup() {
    ENV.call_once(|| {
        dotenvy::from_filename(".env.test")
            .expect(".env.test not found — copy .env.test.example");
    });
}

/// Returns the shared dummy app ID, uploading it on first call.
pub async fn shared_app_id() -> &'static str {
    APP_ID
        .get_or_init(|| async {
            setup();
            let client = ApiClient::new();
            client
                .upload_app(helpers::dummy_zip())
                .await
                .expect("failed to upload shared test app")
                .id
        })
        .await
}

/// Returns the shared app ID only if it was already initialized.
///
/// Used by the cleanup test to avoid uploading just to delete.
pub fn shared_app_id_if_initialized() -> Option<&'static str> {
    APP_ID.get().map(String::as_str)
}
