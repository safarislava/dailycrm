use crate::auth::{Claims, jwt_secret};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
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
        let token_string = encode(
            &Header::default(),
            &Claims {
                sub: user_id,
                jti,
                typ: "refresh".into(),
                exp: expires_at.timestamp() as usize,
            },
            &EncodingKey::from_secret(jwt_secret().as_bytes()),
        )?;
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
