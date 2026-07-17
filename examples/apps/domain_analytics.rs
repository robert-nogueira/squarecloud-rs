use chrono::{Duration, Utc};
use squarecloud::{Client, types::AnalyticsFilters};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example domain_analytics -- <app_id>");
    let end = Utc::now();
    let start = end - Duration::days(7);
    let app = client.app(&app_id);

    let analytics = app.analytics(start, end).await.unwrap();
    println!("== full traffic ==\n{analytics:#?}");

    let filters = AnalyticsFilters::new().country("BR").protocol("HTTP/2");
    let filtered = app.analytics_filtered(start, end, filters).await.unwrap();
    println!("== BR traffic over HTTP/2 ==");
    println!("status codes: {:#?}", filtered.status_codes);
    println!("bots:         {:#?}", filtered.bots);
}
