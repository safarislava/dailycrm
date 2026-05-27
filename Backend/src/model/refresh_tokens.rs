use crate::contract::RefreshTokens;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgRefreshTokens {
    pool: PgPool,
}

impl PgRefreshTokens {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RefreshTokens for PgRefreshTokens {
    async fn user_id_with_jti_revocation(&self, jti: Uuid) -> Result<Option<Uuid>, sqlx::Error> {
        let row: Option<(Uuid,)> = sqlx::query_as(
            "UPDATE refresh_tokens SET revoked_at = NOW() \
             WHERE jti = $1 AND revoked_at IS NULL AND expires_at > NOW() \
             RETURNING user_id",
        )
        .bind(jti)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|(user_id,)| user_id))
    }

    async fn revoke(&self, jti: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE refresh_tokens SET revoked_at = NOW()
             WHERE jti = $1 AND revoked_at IS NULL",
        )
        .bind(jti)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}