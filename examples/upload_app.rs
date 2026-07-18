use squarecloud::Client;

#[tokio::main]
async fn main() {
    let zip_path = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example upload_app -- <zip_path>");
    let client =
        Client::new(std::env::var("API_TOKEN").expect("set API_TOKEN"));
    let bytes = std::fs::read(zip_path).unwrap();
    let app = client.upload_app(bytes).await.unwrap();
    println!("{app:#?}");
}
