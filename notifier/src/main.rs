
mod notifier;
mod mailer;

fn main() {
    dotenvy::dotenv().ok();
    notifier::main();
}

