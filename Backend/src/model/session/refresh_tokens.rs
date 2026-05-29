use crate::auth::{Claims, jwt_secret};
use crate::model::session::contract::refresh_tokens::RefreshTokens;
use crate::model::session::refresh_token::{NewRefreshToken, RefreshToken};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::PgPool;
use uuid::Uuid;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

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
    fn token(&self, jti: Uuid) -> RefreshToken {
        RefreshToken::new(self.pool.clone(), jti)
    }

    async fn new_token(&self, user_id: Uuid) -> Result<NewRefreshToken, BoxError> {
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
        sqlx::query("INSERT INTO refresh_tokens (jti, user_id, expires_at) VALUES ($1, $2, $3)")
            .bind(jti)
            .bind(user_id)
            .bind(expires_at)
            .execute(&self.pool)
            .await?;
        Ok(NewRefreshToken::new(token_string))
    }
}
