use crate::auth::{Claims, jwt_secret};
use crate::common::BoxError;
use crate::model::credential::contract::contentable::Contentable;
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
        Self {
            user_id,
            jti,
            kind,
            expires_at,
        }
    }
}

#[async_trait::async_trait]
impl Contentable for NewToken {
    type Output = String;

    async fn content(&self) -> Result<Self::Output, BoxError> {
        encode(
            &Header::default(),
            &Claims {
                sub: self.user_id,
                jti: self.jti,
                typ: self.kind.as_str().to_owned(),
                exp: self.expires_at.timestamp() as usize,
            },
            &EncodingKey::from_secret(jwt_secret().as_bytes()),
        )
        .map_err(BoxError::from)
    }
}

impl Token for NewToken {}
