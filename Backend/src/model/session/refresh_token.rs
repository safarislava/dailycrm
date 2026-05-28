use sqlx::PgPool;
use uuid::Uuid;

pub struct RefreshToken {
    pool: PgPool,
    jti: Uuid,
}

impl RefreshToken {
    pub fn new(pool: PgPool, jti: Uuid) -> Self {
        Self { pool, jti }
    }

    pub async fn user_id_with_revocation(&self) -> Result<Option<Uuid>, sqlx::Error> {
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

    pub async fn revoke(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE refresh_tokens SET revoked_at = NOW() \
             WHERE jti = $1 AND revoked_at IS NULL",
        )
        .bind(self.jti)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

pub struct NewRefreshToken {
    encoded: String,
}

impl NewRefreshToken {
    pub fn new(encoded: String) -> Self {
        Self { encoded }
    }

    pub fn cookie(&self) -> actix_web::cookie::Cookie<'static> {
        actix_web::cookie::Cookie::build("refresh_token", self.encoded.clone())
            .http_only(true)
            .secure(true)
            .same_site(actix_web::cookie::SameSite::Strict)
            .path("/api/auth")
            .max_age(actix_web::cookie::time::Duration::days(7))
            .finish()
    }
}
