use squarecloud_rs::ApiClient;

#[tokio::test]
async fn me_returns_account_info() {
    crate::setup();
    let client = ApiClient::new();
    let me = client.me().await.unwrap();
    assert!(!me.id.is_empty());
}
