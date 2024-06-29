use std::env;
use std::mem::take;
use std::str::from_utf8;
use async_nats::Subscriber;
use futures::StreamExt;
use entity::game::{ NotificationMessage};

#[tokio::main]
pub(crate) async fn main() {
    let nats_url = env::var("NATS_URL").unwrap();
    let client = async_nats::connect(nats_url).await.unwrap();

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

        println!("{:?}", notification);
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