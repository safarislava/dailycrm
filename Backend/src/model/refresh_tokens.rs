use sqlx::PgPool;
use uuid::Uuid;

pub struct RefreshTokens;

impl RefreshTokens {
    pub async fn user_id_with_jti_revocation(&self, jti: Uuid, pool: &PgPool) -> Result<Option<Uuid>, sqlx::Error> {
        let row: Option<(Uuid,)> = sqlx::query_as(
            "UPDATE refresh_tokens SET revoked_at = NOW() \
             WHERE jti = $1 AND revoked_at IS NULL AND expires_at > NOW() \
             RETURNING user_id",
        )
        .bind(jti)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|(user_id,)| user_id))
    }

    pub async fn revoke(&self, jti: Uuid, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE refresh_tokens SET revoked_at = NOW() WHERE jti = $1 AND revoked_at IS NULL",
        )
        .bind(jti)
        .execute(pool)
        .await?;
        Ok(())
    }
}