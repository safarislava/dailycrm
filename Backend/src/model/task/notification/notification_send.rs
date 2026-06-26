use crate::common::BoxError;
use crate::mail::Mailer;
use crate::model::notification::queued_notification::QueuedNotification;
use crate::model::notification::role_recipients::RoleRecipients;
use crate::model::project::contract::list::List;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;

pub struct NotificationSend {
    pool: Arc<PgPool>,
    mailer: Arc<Mailer>,
    notification: QueuedNotification,
}

impl NotificationSend {
    pub fn new(pool: Arc<PgPool>, mailer: Arc<Mailer>, notification: QueuedNotification) -> Self {
        Self {
            pool,
            mailer,
            notification,
        }
    }
}

#[async_trait::async_trait]
impl Task for NotificationSend {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let (Some(role), Some(subject), Some(body)) = (
            self.notification.role(),
            self.notification.subject(),
            self.notification.body(),
        ) else {
            println!("Queue: Notification has missing fields (role, subject, or body); skipped.");
            return Ok(());
        };
        let emails = RoleRecipients::new(self.pool.clone(), role.clone()).items().await?;
        println!("Queue: Processing notification for role '{:?}'. Found {} recipient email(s) in DB.", role, emails.len());
        for email in emails {
            match self.mailer.send(&email, subject, body.clone()).await {
                Ok(_) => {
                    println!("Queue: Successfully dispatched email to '{}'.", email);
                },
                Err(err) => {
                    eprintln!("Queue ERROR: Failed to send notification email to {}: {:?}", email, err);
                }
            }
        }
        Ok(())
    }
}
