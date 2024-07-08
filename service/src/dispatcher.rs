use std::collections::{HashMap};
use std::str;
use entity::dto::game::{Game, NotificationMessage};
use crate::Context;
use crate::subscription::subscription::{SubscriptionTrait};

pub async fn dispatch(ctx: &Context, player_in_game_map: HashMap<i64, &str>, game_info_map: HashMap<&str, &Game>, nats_client: &async_nats::Client) -> Result<(), Box<dyn std::error::Error>>{

    for (player_id, game_id) in player_in_game_map {
        let get_subscribers_res = ctx.subscription_service.get_subscribers(player_id)
            .await;
        let subscribers;
        match get_subscribers_res {
            Ok(s) => subscribers = s,
            Err(e) => {eprintln!("error getting subscribers: {}", e); continue}
        }

        let get_op = game_info_map.get(game_id);
        let game_info;
        match get_op {
            Some(g) => game_info = g,
            None => {
                println!("no game found for game_id: {}", game_id);
                continue;
            },
        }

        for subscriber in subscribers {

            let msg = NotificationMessage {
                subscriber: subscriber.to_string(),
                game: (*game_info).to_owned(),
            };
            let json_res = serde_json::to_string(&msg);
            let json;
            match json_res {
                Ok(v) => json = v,
                Err(_) => continue,
            }
            let publish_res = nats_client.publish("notification", json.into())
                .await;
            match publish_res {
                Ok(_) => println!("published message"),
                Err(e) => println!("error publishing message: {}", e),
            }
        }
    }

    Ok(())
}

