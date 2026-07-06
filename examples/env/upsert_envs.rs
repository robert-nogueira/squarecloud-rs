use std::collections::HashMap;

use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let mut envs = HashMap::new();
    envs.insert("KEY".to_string(), "value".to_string());
    let result = client
        .app("application_id")
        .upsert_envs(&envs)
        .await
        .unwrap();
    println!("{result:#?}");
}
