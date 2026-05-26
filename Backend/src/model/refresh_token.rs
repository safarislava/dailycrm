use crate::auth::create_refresh_token;
use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct RefreshToken {
    jti: Uuid,
    user_id: Uuid,
    expires_at: DateTime<Utc>,
    token_string: String,
}

impl RefreshToken {
    pub fn new(user_id: Uuid) -> Result<Self, jsonwebtoken::errors::Error> {
        let jti = Uuid::new_v4();
        let expires_at = Utc::now() + Duration::days(7);
        let token_string = create_refresh_token(user_id, jti)?;
        Ok(Self {
            jti,
            user_id,
            expires_at,
            token_string,
        })
    }

    pub async fn store(self, pool: &PgPool) -> Result<String, sqlx::Error> {
        sqlx::query("INSERT INTO refresh_tokens (jti, user_id, expires_at) VALUES ($1, $2, $3)")
            .bind(self.jti)
            .bind(self.user_id)
            .bind(self.expires_at)
            .execute(pool)
            .await?;
        Ok(self.token_string)
    }
}
