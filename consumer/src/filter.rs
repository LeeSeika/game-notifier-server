use std::collections::HashMap;
use entity::game::Game;

pub(crate) async fn r#do<'a>(games: &'a Vec<Game>, game_info_map: &HashMap<&str, &Game>) -> HashMap<i64, &'a str> {
    let mut player_in_game_map: HashMap<i64, &str> = HashMap::new();

    for game in games {
        for player in &game.players {
            let game_id = player_in_game_map.get(&player.account_id).unwrap_or(&"");

            if !game_id.is_empty() {
                let game_info_option = game_info_map.get(game_id);
                let game_info;
                match game_info_option {
                    Some(g) => game_info = g,
                    None => continue,
                }

                if game_info.activate_time > game.activate_time {
                    continue;
                }
            }

            player_in_game_map.insert(player.account_id, game.match_id.as_str());
        }
    }

    player_in_game_map
}