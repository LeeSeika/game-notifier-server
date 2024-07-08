use std::env;
use std::str::from_utf8;
use futures::StreamExt;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use entity::dto::game::NotificationMessage;
use crate::mailer::Mailer;

#[tokio::main]
pub(crate) async fn main() {
    let nats_url = env::var("NATS_URL").unwrap();
    let sender = env::var("EMAIL.SENDER").unwrap();
    let smtp_username = env::var("EMAIL.SMTP_USERNAME").unwrap();
    let smtp_pwd = env::var("EMAIL.SMTP_PWD").unwrap();
    let smtp_host = env::var("EMAIL.SMTP_HOST").unwrap();

    let client = async_nats::connect(nats_url).await.unwrap();
    let mailer = Mailer::new(sender, smtp_username, smtp_pwd, smtp_host);

    let mut notification_subscriber = client
        .subscribe("notification")
        .await
        .unwrap();

    loop {
        println!("looping");
        let res = consume_and_convert(&mut notification_subscriber).await;
        let notification = match res {
            Ok(notification) => notification,
            Err(e) => {
                eprintln!("Error consuming message: {:?}", e);
                continue;
            }
        };

        mailer.send_mail(notification).await
    }
}

async fn consume_and_convert(notification_subscriber: &mut async_nats::Subscriber) -> Result<NotificationMessage, Box<dyn std::error::Error>> {

    let payload_bytes = notification_subscriber
        .next()
        .await
        .ok_or("no message received")?
        .payload;

    let payload_str = from_utf8(&payload_bytes)?;

    let notification: NotificationMessage = serde_json::from_str(payload_str)?;

    Ok(notification)
}