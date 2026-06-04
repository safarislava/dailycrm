use crate::common::BoxError;
use crate::jwt::jwt_secret;
use crate::model::session::claims::Claims;
use crate::model::session::contract::token::Token;
use crate::model::session::token_kind::TokenKind;
use chrono::{DateTime, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use uuid::Uuid;

pub struct NewToken {
    user_id: Uuid,
    jti: Uuid,
    kind: TokenKind,
    expires_at: DateTime<Utc>,
}

impl NewToken {
    pub fn new(user_id: Uuid, jti: Uuid, kind: TokenKind, expires_at: DateTime<Utc>) -> Self {
        Self { user_id, jti, kind, expires_at }
    }
}

#[async_trait::async_trait]
impl Token for NewToken {
    async fn value(&self) -> Result<String, BoxError> {
        encode(
            &Header::default(),
            &Claims::new(
                self.user_id,
                self.jti,
                self.kind.as_str().to_owned(),
                self.expires_at.timestamp() as usize,
            ),
            &EncodingKey::from_secret(jwt_secret().as_bytes()),
        )
        .map_err(BoxError::from)
    }
}