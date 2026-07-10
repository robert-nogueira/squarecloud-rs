use squarecloud::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();

    let stats = client.blob().stats().await.unwrap();
    println!("Objects: {}", stats.usage.objects);
    println!("Storage: {} bytes", stats.usage.storage);
    println!("Plan included: {} bytes", stats.plan.included);
    println!("Estimated cost: ${:.4}", stats.billing.total_estimate);
}
