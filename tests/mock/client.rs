use serde_json::json;
use squarecloud::{ApiError, errors::ServiceStatusErrorCode};
use wiremock::matchers::{method, path, query_param};
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

    let status = client
        .service_status()
        .await
        .expect("service_status() should succeed with mocked 200");
    assert!(!status.status.is_empty());
    assert!(!status.message.is_empty());
}

#[tokio::test]
async fn service_status_error_envelope_maps_to_service_code() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/service/status"))
        .respond_with(ResponseTemplate::new(500).set_body_json(json!({
            "status": "error",
            "code": "INTERNAL_SERVER_ERROR"
        })))
        .mount(&server)
        .await;

    let err = client
        .service_status()
        .await
        .expect_err("mocked 500 should surface as an error");
    assert!(matches!(
        err,
        ApiError::Service {
            code: ServiceStatusErrorCode::InternalServerError
        }
    ));
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
    assert_eq!(result.expect("all_domains() should return vec").len(), 1);
}

#[tokio::test]
async fn load_balancers_returns_groups() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/apps/load-balancers"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "limit": 2,
                "balancers": [
                    {
                        "hostname": "example.com",
                        "apps": [
                            { "id": "abc123def456abc123def456", "name": "web-1", "cluster": "suki-cluster" },
                            { "id": "def456abc123def456abc123", "name": "web-2", "cluster": "nasa-cluster" }
                        ]
                    }
                ]
            }
        })))
        .mount(&server)
        .await;

    let result = client.load_balancers().await;
    assert!(
        result.is_ok(),
        "load_balancers() failed: {:?}",
        result.err()
    );
    let lb = result.expect("load_balancers() should return groups");
    assert_eq!(lb.limit, 2);
    assert_eq!(lb.balancers.len(), 1);
    assert_eq!(lb.balancers[0].hostname, "example.com");
    assert_eq!(lb.balancers[0].apps.len(), 2);
    assert_eq!(
        lb.balancers[0].apps[0].cluster.as_deref(),
        Some("suki-cluster")
    );
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
async fn all_snapshots_with_scope_returns_vec() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/users/snapshots"))
        .and(query_param("scope", "applications"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": []
        })))
        .mount(&server)
        .await;

    let result = client
        .all_snapshots(Some(squarecloud::types::SnapshotScope::Applications))
        .await;
    assert!(
        result.is_ok(),
        "all_snapshots(Some) failed: {:?}",
        result.err()
    );
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
    assert_eq!(
        result.expect("upload_app() should return uploaded app").id,
        "app-new"
    );
}

#[tokio::test]
async fn create_database_returns_database() {
    use squarecloud::types::DatabaseType;

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
    assert_eq!(
        result
            .expect("create_database() should return created database")
            .id,
        "db-new"
    );
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
    assert_eq!(
        result
            .expect("create_workspace() should return created workspace")
            .id,
        "ws-new"
    );
}
