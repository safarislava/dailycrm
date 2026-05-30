use crate::common::BoxError;
use crate::mail::Mailer;
use crate::model::credential::contract::contentable::Contentable;
use crate::model::notification::burning_deadlines::BurningDeadlines;
use crate::model::notification::deadline_digest::DeadlineDigest;
use crate::model::notification::recipients::Recipients;
use crate::model::project::contract::list::List;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;

const SUBJECT: &str = "DailyCRM: Дедлайны";

pub struct DeadlineDigestNotification {
    pool: Arc<PgPool>,
    mailer: Arc<Mailer>,
}

impl DeadlineDigestNotification {
    pub fn new(pool: Arc<PgPool>, mailer: Arc<Mailer>) -> Self {
        Self { pool, mailer }
    }
}

#[async_trait::async_trait]
impl Task for DeadlineDigestNotification {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let digest = DeadlineDigest::new(BurningDeadlines::new(self.pool.clone()).items().await?);
        if digest.is_empty() {
            return Ok(());
        }
        let body = digest.content().await?;
        for email in Recipients::new(self.pool.clone()).items().await? {
            self.mailer.send(&email, SUBJECT, body.clone()).await?;
        }
        Ok(())
    }
}
