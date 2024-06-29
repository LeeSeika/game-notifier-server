use std::env;
use reqwest::{Client, IntoUrl, Response, Version};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

#[tokio::main]
pub async fn start_fetch() {
    let nats_url = env::var("NATS_URL").unwrap();
    let client: async_nats::Client = async_nats::connect(nats_url).await.unwrap();

    loop {
        let res = get("https://api.opendota.com/api/live")
            .await;

        match res {
            Ok(resp) => handle_resp(resp, &client).await,
            Err(e) => println!("error fetching data: {}", e),
        };

        sleep(std::time::Duration::from_secs(120)).await;
    }
}

async fn handle_resp(resp: Response, client: &async_nats::Client) {
    if !resp.status().is_success() {
        println!("failed fetching data, status code: {}", resp.status());
        return;
    }

    let body = resp.text().await.unwrap_or_default();
    if body.is_empty() {
        println!("empty response");
        return;
    }

    println!("{:?}", body);

    let publish_res = client.publish("games", body.into())
        .await;
    match publish_res {
        Ok(_) => println!("published message"),
        Err(e) => println!("error publishing message: {}", e),
    }
}

async fn get<T: IntoUrl + Clone>(url: T) -> reqwest::Result<Response> {
    Client::builder()
        .build()?
        .get(url)
        .version(Version::HTTP_11)
        .send()
        .await
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}