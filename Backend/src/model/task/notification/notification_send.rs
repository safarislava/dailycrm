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
            return Ok(());
        };
        let emails = RoleRecipients::new(self.pool.clone(), role.clone()).items().await?;
        for email in emails {
            match self.mailer.send(&email, subject, body.clone()).await {
                Ok(_) => {},
                Err(err) => eprintln!("Failed to send notification email to {}: {:?}", email, err),
            }
        }
        Ok(())
    }
}
