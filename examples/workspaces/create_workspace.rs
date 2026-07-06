use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let workspace = client
        .create_workspace("my-workspace".to_string())
        .await
        .unwrap();
    println!("{workspace:#?}");
}
