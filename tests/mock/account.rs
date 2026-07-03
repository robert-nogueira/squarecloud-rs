use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn me_deserializes_success_response() {
    let (client, server) = crate::mock_client().await;
    Mock::given(method("GET"))
        .and(path("/users/me"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": "success",
            "response": {
                "user": {
                    "id": "user-123",
                    "name": "Test User",
                    "email": "test@example.com",
                    "created_at": "2024-01-01T00:00:00Z",
                    "plan": {
                        "name": "Starter",
                        "memory": { "limit": 512, "available": 512, "used": 0 },
                        "duration": null
                    }
                },
                "applications": [],
                "databases": []
            }
        })))
        .mount(&server)
        .await;

    let me = client.me().await.unwrap();
    assert_eq!(me.user.id, "user-123");
}
