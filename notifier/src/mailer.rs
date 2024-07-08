use lettre::error::Error;
use lettre::{Message, SmtpTransport, Transport};
use lettre::address::AddressError;
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use entity::dto::game::NotificationMessage;

pub(crate) struct Mailer {
    sender: String,
    smtp_username: String,
    smtp_pwd: String,
    smtp_host: String,
}

impl Mailer {
    pub(crate) fn new(sender: String, smtp_username: String, smtp_pwd: String, smtp_host: String) -> Self {
        Mailer {
            sender,
            smtp_username,
            smtp_pwd,
            smtp_host,
        }
    }
}

impl Mailer {
    pub(crate) async fn send_mail(&self, notification_message: NotificationMessage) {
        let res: Result<Mailbox, AddressError> = notification_message.subscriber.parse();
        let recipient;
        match res {
            Ok(m) => recipient = m,
            Err(e) => {
                eprintln!("Could not parse email: {e:?}");
                return;
            }
        }

        let email = Message::builder()
            .from(self.sender.as_str().parse().unwrap())
            .to(recipient)
            .subject(notification_message.game.match_id)
            .header(ContentType::TEXT_PLAIN)
            .body(String::from("Be happy!"))
            .unwrap();

        let creds = Credentials::new(self.smtp_username.to_owned(), self.smtp_pwd.to_owned());

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay(self.smtp_host.as_str())
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => {
                eprintln!("Could not send email: {e:?}");
                return;
            }
        }
    }
}