use squarecloud::{Client, types::UploadOptions};

#[tokio::main]
async fn main() {
    let client =
        Client::new(std::env::var("API_TOKEN").expect("set API_TOKEN"));

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
