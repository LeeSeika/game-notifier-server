use std::collections::HashMap;
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

    let mut games_subscriber = client
        .subscribe("games")
        .await
        .unwrap();

    let mut bloom = Bloom::new_for_fp_rate(items_count, fp_p);
    let mut count = 0;

    loop {
        println!("looping");
        let res = consume_and_convert(&mut games_subscriber).await;
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

        let mut game_info_map: HashMap<&str, &Game> = HashMap::new();

        let mut game: &Game;

        for i in 0..games.len() {
            game = &games[i];
            if bloom.check(&game.match_id) {
                continue;
            }

            game_info_map.insert(game.match_id.as_str(), &game);

            bloom.set(&game.match_id);
            count += 1;
        }

        if count > items_count * 3 {
            bloom.clear();
            count = 0;
        }

        let player_in_game_map = crate::filter::r#do(&games, &game_info_map).await;
        let _ = service::dispatcher::dispatch(player_in_game_map,  game_info_map, &client)
            .await
            .unwrap();
    }
}

async fn consume_and_convert(games_subscriber: &mut async_nats::Subscriber) -> Result<Vec<Game>, Box<dyn std::error::Error>> {

    let payload_bytes = games_subscriber
        .next()
        .await
        .ok_or("no message received")?
        .payload;

    let payload_str = from_utf8(&payload_bytes)?;

    let games: Vec<Game> = serde_json::from_str(payload_str)?;

    Ok(games)
}