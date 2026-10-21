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
            "my-file.png",
            bytes,
            UploadOptions {
                prefix: Some("images".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    match object.url {
        Some(url) => println!("Uploaded: {url}"),
        None => println!("Uploaded private object: {}", object.id),
    }
}
