use crate::common::BoxError;
use crate::model::task::task::Task;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct RefreshTokenSubmission {
    pool: PgPool,
    user_id: Uuid,
    jti: Uuid,
    expires_at: DateTime<Utc>,
}

impl RefreshTokenSubmission {
    pub fn new(pool: PgPool, user_id: Uuid, jti: Uuid, expires_at: DateTime<Utc>) -> Self {
        Self {
            pool,
            user_id,
            jti,
            expires_at,
        }
    }
}

#[async_trait::async_trait]
impl Task for RefreshTokenSubmission {
    type Output = ();

    async fn output(&self) -> Result<Self::Output, BoxError> {
        sqlx::query("INSERT INTO refresh_tokens (jti, user_id, expires_at) VALUES ($1, $2, $3)")
            .bind(self.jti)
            .bind(self.user_id)
            .bind(self.expires_at)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
