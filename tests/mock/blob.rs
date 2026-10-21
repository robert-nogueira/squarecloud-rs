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
                "id": "pub/123/avatar.png",
                "name": "avatar.png",
                "prefix": "images",
                "size": 4096,
                "private": false,
                "url": "https://blob.squarecloud.app/avatar.png",
                "expires_at": "2026-12-01T00:00:00Z",
                "sha256": "5f70bf18a086007016e948b04aed3b82103a36bea41755b6cddfaf10ace3c6ef",
                "replaced": true
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

    assert_eq!(result.id, "pub/123/avatar.png");
    assert_eq!(result.name, "avatar.png");
    assert_eq!(result.prefix.as_deref(), Some("images"));
    assert_eq!(result.size, 4096);
    assert!(!result.private);
    assert!(result.expires_at.is_some());
    assert_eq!(result.sha256.len(), 64);
    assert!(result.replaced);
    assert_eq!(
        result.url.as_deref(),
        Some("https://blob.squarecloud.app/avatar.png")
    );
}

#[tokio::test]
async fn blob_upload_private_object_has_no_url() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("POST"))
        .and(path("/objects"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "id": "prv/123/draft.png",
                "name": "draft.png",
                "size": 4096,
                "private": true,
                "url": null,
                "sha256": "2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae",
                "replaced": false
            }
        })))
        .mount(&server)
        .await;

    let result = client
        .blob()
        .upload(
            "draft",
            "image/png",
            vec![0u8; 4096],
            UploadOptions::default(),
        )
        .await
        .expect("upload() should accept a null url");

    assert_eq!(result.id, "prv/123/draft.png");
    assert_eq!(result.name, "draft.png");
    assert!(result.private);
    assert_eq!(result.url, None);
    assert_eq!(result.prefix, None);
    assert_eq!(result.expires_at, None);
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
                        "id": "pub/123/images/a.png",
                        "size": 4096,
                        "created_at": "2026-07-08T00:00:00Z",
                        "private": false,
                        "url": "https://blob.squarecloud.dev/pub/123/images/a.png",
                        "etag": "\"9bb58f26192e4ba00f01e2e7b136bbd8\""
                    },
                    {
                        "id": "prv/123/images/b.png",
                        "size": 512,
                        "created_at": "2026-07-08T00:00:00Z",
                        "expires_at": "2026-08-08T00:00:00Z",
                        "private": true,
                        "url": null,
                        "etag": "\"4a2f0b1c9d8e7f6a5b4c3d2e1f0a9b8c\""
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

    assert_eq!(result.objects.len(), 2);
    let (public, private) = (&result.objects[0], &result.objects[1]);
    assert_eq!(public.id, "pub/123/images/a.png");
    assert!(!public.private);
    assert!(public.url.is_some());
    assert_eq!(public.expires_at, None);
    assert!(private.private);
    assert_eq!(private.url, None);
    assert!(private.expires_at.is_some());
    assert!(!private.etag.is_empty());
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
                "continuationToken": "next-page-token"
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
                },
                "month": { "days": 24, "average_storage": 3006477107u64 }
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
    let month = stats.month.expect("month is present in the fixture");
    assert_eq!(month.days, 24);
    assert_eq!(month.average_storage, 3006477107.0);
}

#[tokio::test]
async fn blob_upload_with_invalid_mime_type_is_an_error() {
    let (client, _server) = crate::mock_client().await;
    let result = client
        .blob()
        .upload(
            "draft",
            "not a mime",
            vec![0u8; 512],
            UploadOptions::default(),
        )
        .await;
    match result {
        Err(squarecloud::ApiError::Transport(e)) => {
            assert!(e.is_builder(), "{e}")
        }
        other => panic!("expected a builder error, got {other:?}"),
    }
}
