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
        let username = env::var("SMTP_USERNAME").ok();
        let password = env::var("SMTP_PASSWORD").ok();
        let port: u16 = env::var("SMTP_PORT")
            .unwrap_or_else(|_| "465".to_string())
            .parse()
            .expect("SMTP_PORT must be a number");
        let from = env::var("MAIL_FROM")
            .expect("MAIL_FROM must be set")
            .parse::<Mailbox>()
            .expect("MAIL_FROM must be a valid address");

        println!("Mailer: Initializing with SMTP_HOST={}, SMTP_PORT={}, MAIL_FROM={:?}", host, port, from);

        let settings = MailSettings::new(host, port, username, password);
        Self::new(settings.transport(), from)
    }

    pub async fn send(&self, to: &str, subject: &str, body: String) -> Result<(), BoxError> {
        println!("Mailer: Sending email to '{}' with subject '{}'...", to, subject);
        let message = Message::builder()
            .from(self.from.clone())
            .to(to.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)?;
        match actix_web::rt::time::timeout(Duration::from_secs(10), self.transport.send(message)).await {
            Ok(Ok(_)) => {
                println!("Mailer: Email to '{}' successfully sent.", to);
                Ok(())
            }
            Ok(Err(err)) => {
                eprintln!("Mailer ERROR: Failed to send email to '{}': {:?}", to, err);
                Err(err.into())
            }
            Err(_) => {
                eprintln!("Mailer ERROR: Sending email to '{}' timed out after 10 seconds.", to);
                Err("SMTP send timed out".into())
            }
        }
    }
}

struct MailSettings {
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
}

impl MailSettings {
    fn new(
        host: String,
        port: u16,
        username: Option<String>,
        password: Option<String>,
    ) -> Self {
        Self { host, port, username, password }
    }

    fn transport(&self) -> AsyncSmtpTransport<Tokio1Executor> {
        let mut builder = match self.port {
            465 => AsyncSmtpTransport::<Tokio1Executor>::relay(&self.host)
                .expect("valid SMTP relay"),
            587 => AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&self.host)
                .expect("valid SMTP relay"),
            _ => AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&self.host)
                .port(self.port),
        };

        builder = builder.timeout(Some(Duration::from_secs(10)));
        if let Some(ref creds) = self.credentials() {
            builder = builder.credentials(creds.clone());
        }
        builder.build()
    }

    fn credentials(&self) -> Option<Credentials> {
        match (&self.username, &self.password) {
            (Some(u), Some(p)) if !u.is_empty() && !p.is_empty() => {
                Some(Credentials::new(u.clone(), p.clone()))
            }
            _ => None,
        }
    }
}
