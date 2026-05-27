use crate::auth::{Claims, jwt_secret};
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct AccessToken {
    access_token: String,
}

impl AccessToken {
    pub fn new(user_id: Uuid) -> Result<Self, jsonwebtoken::errors::Error> {
        let exp = (Utc::now().timestamp() + 15 * 60) as usize;
        let access_token = encode(
            &Header::default(),
            &Claims {
                sub: user_id,
                jti: Uuid::new_v4(),
                typ: "access".into(),
                exp,
            },
            &EncodingKey::from_secret(jwt_secret().as_bytes()),
        )?;
        Ok(Self { access_token })
    }
}