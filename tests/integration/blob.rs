use squarecloud::types::UploadOptions;

#[tokio::test]
async fn blob_upload_and_list_and_delete() {
    crate::setup();
    let client = crate::client();

    let bytes: Vec<u8> = vec![b'a'; 1024];
    let object = client
        .blob()
        .upload("hello", "hello.txt", bytes, UploadOptions::default())
        .await
        .expect("blob upload() should succeed");
    // The stored extension comes from the multipart filename.
    assert!(object.id.ends_with(".txt"), "{}", object.id);
    // A prefix rule on the account can make the upload private, so the
    // URL is only checked when the API returns one.
    if let Some(url) = &object.url {
        assert!(!url.is_empty());
    }

    let listing = client
        .blob()
        .list(None, None)
        .await
        .expect("blob list() should succeed");
    assert!(listing.objects.iter().any(|o| o.id == object.id));

    let deleted = client
        .blob()
        .delete(&object.id)
        .await
        .expect("blob delete() should succeed");
    assert!(deleted);
}

#[tokio::test]
async fn blob_list_with_prefix() {
    crate::setup();
    let client = crate::client();

    // prefix must be [a-zA-Z0-9_], 3-32 chars
    let listing = client
        .blob()
        .list(Some("nonexistent"), None)
        .await
        .expect("blob list(prefix) should succeed");
    assert!(listing.objects.is_empty());
}

#[tokio::test]
async fn blob_stats_returns_data() {
    crate::setup();
    let client = crate::client();

    let stats = client
        .blob()
        .stats()
        .await
        .expect("blob stats() should succeed");
    assert!(stats.plan.included > 0);
}
