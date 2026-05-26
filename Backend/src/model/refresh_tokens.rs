use crate::model::refresh_token::RefreshToken;
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

    pub async fn store(&self, token: &RefreshToken) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO refresh_tokens (jti, user_id, expires_at) VALUES ($1, $2, $3)")
            .bind(token.jti)
            .bind(token.user_id)
            .bind(token.expires_at)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn user_id_with_jti_revocation(&self, jti: Uuid) -> Result<Option<Uuid>, sqlx::Error> {
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

    pub async fn revoke(&self, jti: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE refresh_tokens SET revoked_at = NOW() WHERE jti = $1 AND revoked_at IS NULL",
        )
        .bind(jti)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}