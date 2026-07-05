use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

fn workspace_json() -> serde_json::Value {
    json!({
        "id": "ws-123",
        "name": "My Workspace",
        "owner": "user-123",
        "members": [],
        "applications": [],
        "createdAt": "2024-01-01T00:00:00Z"
    })
}

#[tokio::test]
async fn all_workspaces_returns_vec() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/workspaces"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": [workspace_json()]
        })))
        .mount(&server)
        .await;

    let result = client.all_workspaces().await;
    assert!(result.is_ok(), "all_workspaces() failed: {:?}", result.err());
    assert_eq!(result.unwrap().len(), 1);
}

#[tokio::test]
async fn workspace_info_returns_info() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/workspaces/ws-123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": workspace_json()
        })))
        .mount(&server)
        .await;

    let info = client.workspace("ws-123").info().await.unwrap();
    assert_eq!(info.id, "ws-123");
    assert_eq!(info.name, "My Workspace");
}

#[tokio::test]
async fn workspace_get_invite_code() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/workspaces/members/code"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": { "code": "invite-abc123" }
        })))
        .mount(&server)
        .await;

    let result = client.workspace("ws-123").get_invite_code().await;
    assert!(
        result.is_ok(),
        "get_invite_code() failed: {:?}",
        result.err()
    );
    assert_eq!(result.unwrap(), "invite-abc123");
}

#[tokio::test]
async fn workspace_invite_member() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/workspaces/members"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    let result = client
        .workspace("ws-123")
        .invite_member("invite-abc123", "member")
        .await;
    assert!(
        result.is_ok(),
        "invite_member() failed: {:?}",
        result.err()
    );
    assert!(result.unwrap());
}

#[tokio::test]
async fn workspace_remove_member() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("DELETE"))
        .and(path("/workspaces/members"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    let result = client
        .workspace("ws-123")
        .remove_member("user-456")
        .await;
    assert!(
        result.is_ok(),
        "remove_member() failed: {:?}",
        result.err()
    );
    assert!(result.unwrap());
}

#[tokio::test]
async fn workspace_change_member_permissions() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("PATCH"))
        .and(path("/workspaces/members"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    let result = client
        .workspace("ws-123")
        .change_member_permissions("invite-abc123", "admin")
        .await;
    assert!(
        result.is_ok(),
        "change_member_permissions() failed: {:?}",
        result.err()
    );
    assert!(result.unwrap());
}

#[tokio::test]
async fn workspace_add_app() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/workspaces/applications"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    let result = client.workspace("ws-123").add_app("app-456").await;
    assert!(result.is_ok(), "add_app() failed: {:?}", result.err());
    assert!(result.unwrap());
}

#[tokio::test]
async fn workspace_remove_app() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("DELETE"))
        .and(path("/workspaces/applications"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    let result = client.workspace("ws-123").remove_app("app-456").await;
    assert!(result.is_ok(), "remove_app() failed: {:?}", result.err());
    assert!(result.unwrap());
}

#[tokio::test]
async fn workspace_leave() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("DELETE"))
        .and(path("/workspaces/leave"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    let result = client.workspace("ws-123").leave().await;
    assert!(result.is_ok(), "leave() failed: {:?}", result.err());
    assert!(result.unwrap());
}

#[tokio::test]
async fn workspace_delete() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("DELETE"))
        .and(path("/workspaces"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    let result = client.workspace("ws-123").delete().await;
    assert!(result.is_ok(), "delete() failed: {:?}", result.err());
    assert!(result.unwrap());
}
