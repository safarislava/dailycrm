use crate::jwt::jwt_secret;
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: Uuid,
    jti: Uuid,
    typ: String,
    exp: usize,
}

impl Claims {
    pub fn new(sub: Uuid, jti: Uuid, typ: String, exp: usize) -> Self {
        Self { sub, jti, typ, exp }
    }

    pub fn from(token: &str) -> Option<Self> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret().as_bytes()),
            &Validation::default(),
        )
        .ok()
        .map(|d| d.claims)
    }

    pub fn sub(&self) -> Uuid {
        self.sub
    }

    pub fn jti(&self) -> Uuid {
        self.jti
    }
}
