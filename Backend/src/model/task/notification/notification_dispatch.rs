use crate::common::BoxError;
use crate::mail::Mailer;
use crate::model::task::contract::task::Task;
use crate::model::task::notification::notification_dequeue::NotificationDequeue;
use crate::model::task::notification::notification_send::NotificationSend;
use sqlx::PgPool;
use std::sync::Arc;

pub struct NotificationDispatch {
    pool: Arc<PgPool>,
    mailer: Arc<Mailer>,
}

impl NotificationDispatch {
    pub fn new(pool: Arc<PgPool>, mailer: Arc<Mailer>) -> Self {
        Self { pool, mailer }
    }
}

#[async_trait::async_trait]
impl Task for NotificationDispatch {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let notifications = NotificationDequeue::new(self.pool.clone()).done().await?;
        if !notifications.is_empty() {
            println!("Queue: Dequeued {} notification(s) for dispatching.", notifications.len());
        }
        for notification in notifications {
            NotificationSend::new(self.pool.clone(), self.mailer.clone(), notification)
                .done()
                .await?;
        }
        Ok(())
    }
}
