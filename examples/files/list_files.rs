use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let files = client
        .app("application_id")
        .file("/")
        .all_files("/")
        .await
        .unwrap();
    for f in &files {
        println!("{} ({:?}) {} bytes", f.name, f.file_type, f.size);
    }
}
