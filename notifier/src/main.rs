mod notifier;

fn main() {

    dotenvy::dotenv().ok();
    notifier::main();
}
