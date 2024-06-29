

mod fetcher;

fn main() {

    dotenvy::dotenv().ok();
    fetcher::start_fetch();
}