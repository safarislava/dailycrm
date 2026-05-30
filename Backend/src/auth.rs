use crate::model::session::token_kind::TokenKind;
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::{env, sync::OnceLock};
use uuid::Uuid;

// TODO: refactor this shit
static JWT_SECRET: OnceLock<String> = OnceLock::new();

pub fn jwt_secret() -> &'static str {
    JWT_SECRET.get_or_init(|| env::var("JWT_SECRET").expect("JWT_SECRET must be set"))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub jti: Uuid,
    pub typ: String,
    pub exp: usize,
}

pub struct JwtToken {
    raw: String,
}

impl JwtToken {
    pub fn new(raw: &str) -> Self {
        Self {
            raw: raw.to_owned(),
        }
    }

    pub fn access_user_id(&self) -> Option<Uuid> {
        self.decode()
            .ok()
            .filter(|c| c.typ == TokenKind::Access.as_str())
            .map(|c| c.sub)
    }

    pub fn jti(&self) -> Option<Uuid> {
        self.decode()
            .ok()
            .filter(|c| c.typ == TokenKind::Refresh.as_str())
            .map(|c| c.jti)
    }

    fn decode(&self) -> Result<Claims, jsonwebtoken::errors::Error> {
        let data = decode::<Claims>(
            &self.raw,
            &DecodingKey::from_secret(jwt_secret().as_bytes()),
            &Validation::default(),
        )?;
        Ok(data.claims)
    }
}
