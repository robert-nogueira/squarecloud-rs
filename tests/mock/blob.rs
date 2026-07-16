use serde_json::json;
use squarecloud::{errors::BlobErrorCode, types::UploadOptions};
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn blob_upload_returns_object() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/objects"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "id": "obj-abc123",
                "name": "avatar.png",
                "size": 4096,
                "url": "https://blob.squarecloud.app/avatar.png"
            }
        })))
        .mount(&server)
        .await;

    let result = client
        .blob()
        .upload(
            "avatar_png",
            "image/png",
            vec![0u8; 4096],
            UploadOptions::default(),
        )
        .await
        .expect("upload() should succeed with mocked 200");

    assert_eq!(result.id, "obj-abc123");
    assert_eq!(result.name, "avatar.png");
    assert_eq!(result.size, 4096);
    assert!(!result.url.is_empty());
}

#[tokio::test]
async fn blob_list_returns_objects() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/objects"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "objects": [
                    {
                        "id": "obj-abc123",
                        "size": 4096,
                        "created_at": "2026-07-08T00:00:00Z",
                        "expires_at": null
                    }
                ]
            }
        })))
        .mount(&server)
        .await;

    let result = client
        .blob()
        .list(None, None)
        .await
        .expect("list() should succeed with mocked 200");

    assert_eq!(result.objects.len(), 1);
    assert_eq!(result.objects[0].id, "obj-abc123");
    assert!(result.continuation_token.is_none());
}

#[tokio::test]
async fn blob_list_with_continuation_token() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/objects"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "objects": [],
                "continuation_token": "next-page-token"
            }
        })))
        .mount(&server)
        .await;

    let result = client
        .blob()
        .list(Some("images"), None)
        .await
        .expect("list(prefix) should succeed with mocked 200");

    assert!(result.objects.is_empty());
    assert_eq!(
        result.continuation_token.as_deref(),
        Some("next-page-token")
    );
}

#[tokio::test]
async fn blob_delete_returns_true() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("DELETE"))
        .and(path("/objects"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success"
        })))
        .mount(&server)
        .await;

    let result = client
        .blob()
        .delete("obj-abc123")
        .await
        .expect("delete() should return true with mocked 200");

    assert!(result);
}

#[tokio::test]
async fn blob_delete_object_not_found_returns_error() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("DELETE"))
        .and(path("/objects"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "status": "error",
            "code": "OBJECT_NOT_FOUND"
        })))
        .mount(&server)
        .await;

    let err = client
        .blob()
        .delete("nonexistent-object")
        .await
        .expect_err("delete() should fail with OBJECT_NOT_FOUND");

    assert!(
        matches!(
            err,
            squarecloud::ApiError::Service {
                code: BlobErrorCode::ObjectNotFound
            }
        ),
        "unexpected error: {err:?}"
    );
}

#[tokio::test]
async fn blob_stats_returns_stats() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/account/stats"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "usage": { "objects": 42, "storage": 1048576 },
                "plan": { "included": 10737418240u64 },
                "billing": {
                    "extraStorage": 0,
                    "storagePrice": 0.0,
                    "objectsPrice": 0.0,
                    "totalEstimate": 0.0
                }
            }
        })))
        .mount(&server)
        .await;

    let stats = client
        .blob()
        .stats()
        .await
        .expect("stats() should succeed with mocked 200");

    assert_eq!(stats.usage.objects, 42);
    assert_eq!(stats.usage.storage, 1048576);
    assert_eq!(stats.plan.included, 10737418240);
    assert_eq!(stats.billing.total_estimate, 0.0);
}
