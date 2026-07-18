use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client =
        Client::new(std::env::var("API_TOKEN").expect("set API_TOKEN"));
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <app_id>");
    let files = client.app(&app_id).file("/").all_files("/").await.unwrap();
    for f in &files {
        println!("{} ({:?}) {} bytes", f.name, f.file_type, f.size);
    }
}
