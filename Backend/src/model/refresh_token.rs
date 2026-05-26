use crate::auth::create_refresh_token;
use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

pub struct RefreshToken {
    pub jti: Uuid,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub token_string: String,
}

impl RefreshToken {
    pub fn new(user_id: Uuid) -> Result<Self, jsonwebtoken::errors::Error> {
        let jti = Uuid::new_v4();
        let expires_at = Utc::now() + Duration::days(7);
        let token_string = create_refresh_token(user_id, jti)?;
        Ok(Self { jti, user_id, expires_at, token_string })
    }
}