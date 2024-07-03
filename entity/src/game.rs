use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Game {
    pub activate_time: i64,
    pub deactivate_time: i64,
    pub server_steam_id: String,
    pub lobby_id: String,
    pub league_id: i32,
    pub lobby_type: i32,
    pub game_time: i32,
    pub delay: i32,
    pub spectators: i32,
    pub game_mode: i32,
    pub average_mmr: i32,
    pub match_id: String,
    pub series_id: i32,
    pub team_name_radiant: String,
    pub team_name_dire: String,
    pub team_logo_radiant: String,
    pub team_logo_dire: String,
    pub team_id_radiant: i32,
    pub team_id_dire: i32,
    pub sort_score: i32,
    pub last_update_time: i64,
    pub radiant_lead: i32,
    pub radiant_score: i32,
    pub dire_score: i32,
    pub players: Vec<Player>,
    pub building_state: i32,
    pub weekend_tourney_tournament_id: i32,
    pub weekend_tourney_division: i32,
    pub weekend_tourney_skill_level: i32,
    pub weekend_tourney_bracket_round: i32,
    pub custom_game_difficulty: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Player {
    pub account_id: i64,
    pub hero_id: i32,
    pub team_slot: i32,
    pub team: i32,
    pub name: Option<String>,
    pub country_code: Option<String>,
    pub fantasy_role: Option<i32>,
    pub team_id: Option<i32>,
    pub team_name: Option<String>,
    pub team_tag: Option<String>,
    pub is_locked: Option<bool>,
    pub is_pro: Option<bool>,
    pub locked_until: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationMessage {
    pub subscriber: String,
    pub game: Game,
}