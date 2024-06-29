use std::env;
use tikv_client::RawClient;

mod notifier;

fn main() {
    dotenvy::dotenv().ok();
    test_add_data();
    notifier::main();
}

#[tokio::main]
async fn test_add_data() {
    let urls= env::var("TIKV.URLS")
        .unwrap()
        .split(",")
        .map(|s| s.to_string())
        .collect();
    let client = RawClient::new(urls).await.unwrap();
    client.put("301339401".to_string(), "Hello,TiKV!".to_string()).await.unwrap();
}
