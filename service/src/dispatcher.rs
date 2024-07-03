use std::collections::{HashMap};
use std::env;
use std::os::unix::raw::time_t;
use tikv_client::RawClient;
use std::str;
use entity::game::{Game, NotificationMessage};
use uuid::uuid;

pub async fn dispatch(player_in_game_map: HashMap<i64, &str>, game_info_map: HashMap<&str, &Game>, nats_client: &async_nats::Client) -> Result<(), Box<dyn std::error::Error>>{
    let urls= env::var("TIKV.URLS")
        .unwrap()
        .split(",")
        .map(|s| s.to_string())
        .collect();
    let client = RawClient::new(urls).await?;

    for (player_id, game_id) in player_in_game_map {
        let get_op = client.get(player_id.to_string()).await.unwrap();
        let subscribers;
        match get_op {
            Some(v) => {
                subscribers = v;
                println!("{:?}", subscribers)
            },
            None => {
                continue
            },
        }

        let convert_res = str::from_utf8(&subscribers);
        let subscribers;
        match convert_res {
            Ok(v) =>{
                subscribers = v.to_string();
                println!("{:?}", subscribers);
            },
            Err(_) => {
                client.delete(player_id.to_string()).await.ok();
                //todo: log error
                eprintln!("error converting subscriber data to string");
                continue
            },
        }

        let subscribers: Vec<String> = subscribers.split(",").map(|s| s.to_string()).collect();
        println!("subs: {:?}", subscribers);

        let get_op = game_info_map.get(game_id);
        let game_info;
        match get_op {
            Some(g) => game_info = g,
            None => {
                println!("no game found for {}", game_id);
                continue;
            },
        }

        for subscriber in subscribers {

            let uuid = uuid::Uuid::new_v4();

            let msg = NotificationMessage {
                subscriber: uuid.to_string(),
                game: (*game_info).to_owned(),
            };
            println!("subscriber: {:?}", uuid.to_string());
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