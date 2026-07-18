use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client =
        Client::new(std::env::var("API_TOKEN").expect("set API_TOKEN"));
    let lb = client.load_balancers().await.unwrap();
    println!("plan limit: {} apps per domain", lb.limit);
    for group in lb.balancers {
        println!("{} ({} apps)", group.hostname, group.apps.len());
        for app in group.apps {
            println!("  {} [{}]", app.name, app.id);
        }
    }
}
