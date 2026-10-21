use squarecloud::{ApiError, errors::WorkspaceErrorCode};

#[tokio::test]
async fn create_workspace_and_delete() {
    crate::setup();
    crate::throttle().await;
    let client = crate::client();
    let ws = match client
        .create_workspace("squarecloud-rs-test".to_string())
        .await
    {
        Ok(ws) => ws,
        Err(ApiError::Service {
            code:
                code @ (WorkspaceErrorCode::WorkspaceLimitReached
                | WorkspaceErrorCode::UpgradeRequired),
        }) => {
            eprintln!("create_workspace skipped: {code:?}");
            return;
        }
        Err(e) => panic!("create_workspace() failed: {e:?}"),
    };
    // Delete before asserting, so a failed assertion never leaves the
    // workspace behind.
    crate::throttle().await;
    let handle = client.workspace(&ws.id);
    let deleted = crate::delete_with_retry(|| handle.delete())
        .await
        .expect("cleanup: workspace delete() should succeed");
    assert!(deleted);
    assert!(!ws.id.is_empty());
    assert_eq!(ws.name, "squarecloud-rs-test");
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
    // Skip the workspace create_workspace_and_delete may be deleting.
    let Some(ws) = workspaces.iter().find(|w| w.name != "squarecloud-rs-test")
    else {
        return;
    };
    let info = crate::client()
        .workspace(&ws.id)
        .info()
        .await
        .expect("workspace info() should succeed for existing workspace");
    assert_eq!(info.id, ws.id);
    assert!(!info.name.is_empty());
}
