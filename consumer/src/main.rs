pub mod consumer;
mod filter;

fn main() {
    dotenvy::dotenv().ok();
    consumer::main();
}