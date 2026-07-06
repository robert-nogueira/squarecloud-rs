use squarecloud::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <app_id>");
    let keys = ["KEY".to_string()];
    let result = client.app(&app_id).delete_envs(&keys).await.unwrap();
    println!("{result:#?}");
}
