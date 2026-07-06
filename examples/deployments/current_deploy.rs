use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let deploy = client
        .app("application_id")
        .current_deploy()
        .await
        .unwrap();
    println!("{deploy:#?}");
}
