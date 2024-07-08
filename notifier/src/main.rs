
mod notifier;

fn main() {
    dotenvy::dotenv().ok();
    test_add_data();
    notifier::main();
}

#[tokio::main]
async fn test_add_data() {}
