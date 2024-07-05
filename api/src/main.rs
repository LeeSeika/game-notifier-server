mod setup;
mod handlers;
mod req_models;
mod resp_models;

use std::env;
use sea_orm::Database;

fn main() {

    dotenvy::dotenv().ok();
    setup::start();
}
