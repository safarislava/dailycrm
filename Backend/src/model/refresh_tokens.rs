use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct RefreshTokens {
    pool: PgPool,
}

impl RefreshTokens {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn store(
        &self,
        jti: Uuid,
        user_id: Uuid,
        expires_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO refresh_tokens (jti, user_id, expires_at) VALUES ($1, $2, $3)",
        )
        .bind(jti)
        .bind(user_id)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn is_valid(&self, jti: Uuid) -> Result<bool, sqlx::Error> {
        let row: Option<(bool,)> = sqlx::query_as(
            "SELECT revoked_at IS NULL FROM refresh_tokens WHERE jti = $1 AND expires_at > NOW()",
        )
        .bind(jti)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|(not_revoked,)| not_revoked).unwrap_or(false))
    }

    pub async fn revoke(&self, jti: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE refresh_tokens SET revoked_at = NOW() WHERE jti = $1 AND revoked_at IS NULL")
            .bind(jti)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}