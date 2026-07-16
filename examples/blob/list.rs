use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();

    let result = client.blob().list(None, None).await.unwrap();
    println!("Objects ({}):", result.objects.len());
    for obj in &result.objects {
        println!("  {} — {} bytes", obj.id, obj.size);
    }
    if let Some(token) = result.continuation_token {
        println!("Next page token: {token}");
    }
}
