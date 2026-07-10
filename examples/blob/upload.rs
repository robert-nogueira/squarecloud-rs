use squarecloud::{ApiClient, types::UploadOptions};

#[tokio::main]
async fn main() {
    let client = ApiClient::new();

    let bytes = std::fs::read("my-file.png").unwrap();
    let object = client
        .blob()
        .upload(
            "my_image",
            "image/png",
            bytes,
            UploadOptions {
                prefix: Some("images".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    println!("Uploaded: {}", object.url);
}
