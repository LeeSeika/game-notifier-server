use reqwest::{Client, IntoUrl, Response, Version};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use entity::game::Game;
use serde_json::Result;

#[tokio::main]
pub async fn start_fetch() {
    loop {
        let res = get("https://api.opendota.com/api/live")
            .await;

        match res {
            Ok(resp) => handle_resp(resp).await,
            Err(e) => println!("error fetching data: {}", e),
        };

        sleep(std::time::Duration::from_secs(20)).await;
    }
}

async fn handle_resp(resp: Response) {
    let body = resp.text().await.unwrap_or_default();
    if body.is_empty() {
        println!("empty response");
        return;
    }

    let res: Result<Vec<Game>> = serde_json::from_str(body.as_str());
    let games: Vec<Game>;
    match res {
        Ok(gs) => games = gs,
        Err(e) => {println!("error parsing game: {}", e); return},
    }
    println!("game: {:?}", games);

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