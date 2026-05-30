use crate::common::BoxError;
use crate::model::task::task::Task;
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserIdReceipt {
    pool: PgPool,
    jti: Uuid,
}

impl UserIdReceipt {
    pub fn new(pool: PgPool, jti: Uuid) -> Self {
        Self { pool, jti }
    }
}

#[async_trait::async_trait]
impl Task for UserIdReceipt {
    type Output = Option<Uuid>;

    async fn output(&self) -> Result<Self::Output, BoxError> {
        let row: Option<(Uuid,)> = sqlx::query_as(
            "UPDATE refresh_tokens SET revoked_at = NOW() \
             WHERE jti = $1 AND revoked_at IS NULL AND expires_at > NOW() \
             RETURNING user_id",
        )
        .bind(self.jti)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|(user_id,)| user_id))
    }
}
