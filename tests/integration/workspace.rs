use squarecloud_rs::ApiClient;

#[tokio::test]
async fn all_workspaces_returns_vec() {
    crate::setup();
    crate::throttle().await;
    let client = ApiClient::new();
    let result = client.all_workspaces().await;
    assert!(result.is_ok(), "all_workspaces() failed: {:?}", result.err());
}

#[tokio::test]
async fn workspace_info_returns_info() {
    crate::setup();
    crate::throttle().await;
    let client = ApiClient::new();
    let workspaces = client.all_workspaces().await.unwrap();
    if workspaces.is_empty() {
        return;
    }
    let ws = &workspaces[0];
    let info = ApiClient::new().workspace(&ws.id).info().await.unwrap();
    assert_eq!(info.id, ws.id);
    assert!(!info.name.is_empty());
}
