use crate::common::BoxError;
use lettre::message::Mailbox;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use std::env;
use std::time::Duration;

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
        let username = env::var("SMTP_USERNAME").ok().unwrap_or_default();
        let password = env::var("SMTP_PASSWORD").ok().unwrap_or_default();
        let port: u16 = env::var("SMTP_PORT")
            .unwrap_or_else(|_| "465".to_string())
            .parse()
            .expect("SMTP_PORT must be a number");
        let from = env::var("MAIL_FROM")
            .expect("MAIL_FROM must be set")
            .parse::<Mailbox>()
            .expect("MAIL_FROM must be a valid address");

        println!("Mailer: Initializing with SMTP_HOST={}, SMTP_PORT={}, MAIL_FROM={:?}", host, port, from);

        let credentials = if !username.is_empty() && !password.is_empty() {
            Some(Credentials::new(username, password))
        } else {
            None
        };

        let transport = if port == 465 {
            let mut builder = AsyncSmtpTransport::<Tokio1Executor>::relay(&host)
                .expect("valid SMTP relay")
                .timeout(Some(Duration::from_secs(10)));
            if let Some(creds) = credentials {
                builder = builder.credentials(creds);
            }
            builder.build()
        } else {
            let mut builder = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&host)
                .port(port)
                .timeout(Some(Duration::from_secs(10)));
            if let Some(creds) = credentials {
                builder = builder.credentials(creds);
            }
            builder.build()
        };
        Self::new(transport, from)
    }

    pub async fn send(&self, to: &str, subject: &str, body: String) -> Result<(), BoxError> {
        println!("Mailer: Sending email to '{}' with subject '{}'...", to, subject);
        let message = Message::builder()
            .from(self.from.clone())
            .to(to.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)?;
        match self.transport.send(message).await {
            Ok(_) => {
                println!("Mailer: Email to '{}' successfully sent.", to);
                Ok(())
            }
            Err(err) => {
                eprintln!("Mailer ERROR: Failed to send email to '{}': {:?}", to, err);
                Err(err.into())
            }
        }
    }
}
