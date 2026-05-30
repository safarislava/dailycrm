use crate::common::BoxError;
use lettre::message::Mailbox;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use std::env;

pub struct Mailer {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from: Mailbox,
}

impl Mailer {
    pub fn new(transport: AsyncSmtpTransport<Tokio1Executor>, from: Mailbox) -> Self {
        Self { transport, from }
    }

    pub fn from_env() -> Self {
        let host = env::var("SMTP_HOST").expect("SMTP_HOST must be set");
        let username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
        let password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
        let port: u16 = env::var("SMTP_PORT")
            .unwrap_or_else(|_| "465".to_string())
            .parse()
            .expect("SMTP_PORT must be a number");
        let from = env::var("MAIL_FROM")
            .expect("MAIL_FROM must be set")
            .parse::<Mailbox>()
            .expect("MAIL_FROM must be a valid address");
        let transport = if port == 465 {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&host)
                .expect("valid SMTP relay")
                .credentials(Credentials::new(username, password))
                .build()
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&host)
                .port(port)
                .credentials(Credentials::new(username, password))
                .build()
        };
        Self::new(transport, from)
    }

    pub async fn send(&self, to: &str, subject: &str, body: String) -> Result<(), BoxError> {
        let message = Message::builder()
            .from(self.from.clone())
            .to(to.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)?;
        self.transport.send(message).await?;
        Ok(())
    }
}
