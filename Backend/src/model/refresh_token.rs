use crate::auth::{Claims, jwt_secret};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::PgPool;
use uuid::Uuid;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub struct RefreshToken {
    user_id: Uuid,
}

impl RefreshToken {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }

    pub async fn store(self, pool: &PgPool) -> Result<String, BoxError> {
        let jti = Uuid::new_v4();
        let expires_at = Utc::now() + Duration::days(7);
        let token_string = encode(
            &Header::default(),
            &Claims {
                sub: self.user_id,
                jti,
                typ: "refresh".into(),
                exp: expires_at.timestamp() as usize,
            },
            &EncodingKey::from_secret(jwt_secret().as_bytes()),
        )?;
        sqlx::query("INSERT INTO refresh_tokens (jti, user_id, expires_at) VALUES ($1, $2, $3)")
            .bind(jti)
            .bind(self.user_id)
            .bind(expires_at)
            .execute(pool)
            .await?;
        Ok(token_string)
    }
}