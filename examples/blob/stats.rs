use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client =
        Client::new(std::env::var("API_TOKEN").expect("set API_TOKEN"));

    let stats = client.blob().stats().await.unwrap();
    println!("Objects: {}", stats.usage.objects);
    println!("Storage: {} bytes", stats.usage.storage);
    println!("Plan included: {} bytes", stats.plan.included);
    println!("Estimated cost: ${:.4}", stats.billing.total_estimate);
}
