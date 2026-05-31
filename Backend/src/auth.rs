use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::{env, sync::OnceLock};
use uuid::Uuid;

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

pub fn decoded_claims(token: &str) -> Option<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret().as_bytes()),
        &Validation::default(),
    )
    .ok()
    .map(|d| d.claims)
}
