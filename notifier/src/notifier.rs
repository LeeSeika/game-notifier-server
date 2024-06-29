use std::env;
use std::str::from_utf8;
use futures::StreamExt;
use entity::game::Game;
use bloomfilter::Bloom;

#[tokio::main]
pub(crate) async fn main() {
    let items_count = env::var("BLOOM.ITEMS_COUNT")
        .unwrap_or(String::from("1000"))
        .parse::<usize>()
        .unwrap_or(100_000);
    let fp_p = env::var("BLOOM.FALSE_POSITIVE_RATE")
        .unwrap_or("0.01".to_string())
        .parse::<f64>()
        .unwrap_or(0.01);

    let nats_url = env::var("NATS_URL").unwrap();
    let client = async_nats::connect(nats_url).await.unwrap();

    let mut bloom = Bloom::new_for_fp_rate(items_count, fp_p);
    let mut count = 0;

    loop {
        let mut res = consume_and_convert(&client).await;
        let games: Vec<Game>;
        match res {
            Ok(_games) => {
                games = _games;
            }
            Err(e) => {
                println!("Error: {:?}", e);
                continue;
            }
        }

        for game in games {
            if bloom.check(&game.match_id) {
                continue;
            }

            
            bloom.set(&game.match_id);
            count += 1;

            if count >= items_count {
                bloom.clear();
                count = 0;
            }
        }
    }
}

async fn consume_and_convert(client: &async_nats::Client) -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    let mut subscription = client.subscribe("games").await.unwrap().take(1);

    let payload_bytes = subscription.next()
        .await
        .ok_or("no message received")?
        .payload;

    let payload_str = from_utf8(&payload_bytes)?;

    let games: Vec<Game> = serde_json::from_str(payload_str)?;

    Ok(games)
}