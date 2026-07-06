use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn service_status_returns_status() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/service/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "operational",
            "message": "All systems normal"
        })))
        .mount(&server)
        .await;

    let status = client.service_status().await.unwrap();
    assert!(!status.status.is_empty());
    assert!(!status.message.is_empty());
}

#[tokio::test]
async fn all_domains_returns_vec() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/domains"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": [
                { "app_id": "app-123", "hostname": "myapp.squareweb.app", "type": "subdomain" }
            ]
        })))
        .mount(&server)
        .await;

    let result = client.all_domains().await;
    assert!(result.is_ok(), "all_domains() failed: {:?}", result.err());
    assert_eq!(result.unwrap().len(), 1);
}

#[tokio::test]
async fn all_database_status_returns_vec() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/databases/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": [
                { "id": "db-123", "running": true, "cpu": "1%", "ram": "64MB" }
            ]
        })))
        .mount(&server)
        .await;

    let result = client.all_database_status().await;
    assert!(
        result.is_ok(),
        "all_database_status() failed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn all_snapshots_returns_vec() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/users/snapshots"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": [
                {
                    "name": "my-app",
                    "size": 1024,
                    "modified": "2024-01-01T00:00:00Z",
                    "key": "abc123"
                }
            ]
        })))
        .mount(&server)
        .await;

    let result = client.all_snapshots(None).await;
    assert!(result.is_ok(), "all_snapshots() failed: {:?}", result.err());
}

#[tokio::test]
async fn all_workspaces_returns_vec() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/workspaces"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": [
                {
                    "id": "ws-123",
                    "name": "My Workspace",
                    "owner": "user-123",
                    "members": [],
                    "applications": [],
                    "createdAt": "2024-01-01T00:00:00Z"
                }
            ]
        })))
        .mount(&server)
        .await;

    let result = client.all_workspaces().await;
    assert!(
        result.is_ok(),
        "all_workspaces() failed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn upload_app_returns_uploaded_app() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/apps"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "id": "app-new",
                "name": "my-app",
                "description": null,
                "subdomain": null,
                "ram": 512,
                "cpu": 0.5,
                "language": { "name": "rust", "version": "1.80" }
            }
        })))
        .mount(&server)
        .await;

    let result = client.upload_app(vec![0u8; 4]).await;
    assert!(result.is_ok(), "upload_app() failed: {:?}", result.err());
    assert_eq!(result.unwrap().id, "app-new");
}

#[tokio::test]
async fn create_database_returns_database() {
    use squarecloud_rs::types::DatabaseType;

    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/databases"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "id": "db-new",
                "name": "my-db",
                "memory": 256,
                "cpu": 1,
                "type": "postgres",
                "password": "secret",
                "certificate": "cert-pem",
                "connection_url": "postgres://localhost/mydb"
            }
        })))
        .mount(&server)
        .await;

    let result = client
        .create_database(
            "my-db".to_string(),
            256,
            DatabaseType::Postgres,
            "16".to_string(),
        )
        .await;
    assert!(
        result.is_ok(),
        "create_database() failed: {:?}",
        result.err()
    );
    assert_eq!(result.unwrap().id, "db-new");
}

#[tokio::test]
async fn create_workspace_returns_workspace_info() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/workspaces"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "id": "ws-new",
                "name": "my-workspace",
                "owner": "user-123",
                "members": [],
                "applications": [],
                "createdAt": "2024-01-01T00:00:00Z"
            }
        })))
        .mount(&server)
        .await;

    let result = client.create_workspace("my-workspace".to_string()).await;
    assert!(
        result.is_ok(),
        "create_workspace() failed: {:?}",
        result.err()
    );
    assert_eq!(result.unwrap().id, "ws-new");
}
