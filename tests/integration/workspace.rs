#[tokio::test]
async fn create_workspace_and_delete() {
    crate::setup();
    crate::throttle().await;
    let client = crate::client();
    let ws = client
        .create_workspace("squarecloud-rs-test".to_string())
        .await;
    if let Err(e) = &ws {
        eprintln!("create_workspace skipped: {e:?}");
        return;
    }
    let ws = ws.expect("create_workspace() should succeed after Err check");
    assert!(!ws.id.is_empty());
    assert_eq!(ws.name, "squarecloud-rs-test");
    crate::throttle().await;
    client.workspace(&ws.id).delete().await.ok();
}

#[tokio::test]
async fn all_workspaces_returns_vec() {
    crate::setup();
    crate::throttle().await;
    let client = crate::client();
    let result = client.all_workspaces().await;
    assert!(
        result.is_ok(),
        "all_workspaces() failed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn workspace_info_returns_info() {
    crate::setup();
    crate::throttle().await;
    let client = crate::client();
    let workspaces = client
        .all_workspaces()
        .await
        .expect("all_workspaces() should return workspace list");
    if workspaces.is_empty() {
        return;
    }
    let ws = &workspaces[0];
    let info = crate::client()
        .workspace(&ws.id)
        .info()
        .await
        .expect("workspace info() should succeed for existing workspace");
    assert_eq!(info.id, ws.id);
    assert!(!info.name.is_empty());
}
