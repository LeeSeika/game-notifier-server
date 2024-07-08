mod setup;
mod handlers;
mod req_models;
mod resp_models;

fn main() {

    dotenvy::dotenv().ok();
    setup::start();
}
