use std::collections::HashMap;

use squarecloud::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example NAME -- <app_id>");
    let mut envs = HashMap::new();
    envs.insert("KEY".to_string(), "value".to_string());
    let result = client.app(&app_id).overwrite_envs(&envs).await.unwrap();
    println!("{result:#?}");
}
