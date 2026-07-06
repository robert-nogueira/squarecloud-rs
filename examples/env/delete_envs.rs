use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let keys = ["KEY".to_string()];
    let result = client
        .app("application_id")
        .delete_envs(&keys)
        .await
        .unwrap();
    println!("{result:#?}");
}
