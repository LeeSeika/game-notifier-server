use std::env;
use std::str::from_utf8;
use futures::future::err;
use futures::StreamExt;
use entity::game::Game;

#[tokio::main]
pub(crate) async fn main() {
    let nats_url = env::var("NATS_URL").unwrap();
    let client = async_nats::connect(nats_url).await.unwrap();

    loop {
        let mut  res = subscribe_and_convert(&client).await;
        let games: Vec<Game>;
        match res {
            Ok(_games) => {
                games = _games;
            },
            Err(e) => {
                println!("Error: {:?}", e);
                continue
            }
        }

        println!("{:?}", games)
    }
}

async fn subscribe_and_convert(client: &async_nats::Client) -> Result<Vec<Game>, Box<dyn std::error::Error>>{
    let mut subscription = client.subscribe("games").await.unwrap().take(1);

    let payload_bytes = subscription.next()
        .await
        .ok_or("No message received")?
        .payload;

    let payload_str = from_utf8(&payload_bytes)?;

    let games: Vec<Game> = serde_json::from_str(payload_str)?;

   Ok(games)
}