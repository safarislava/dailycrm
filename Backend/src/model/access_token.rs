use crate::auth::{Claims, jwt_secret};
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use uuid::Uuid;

pub struct AccessToken {
    user_id: Uuid,
}

impl AccessToken {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }

    fn encoded(&self) -> Result<String, jsonwebtoken::errors::Error> {
        let exp = (Utc::now().timestamp() + 15 * 60) as usize;
        encode(
            &Header::default(),
            &Claims {
                sub: self.user_id,
                jti: Uuid::new_v4(),
                typ: "access".into(),
                exp,
            },
            &EncodingKey::from_secret(jwt_secret().as_bytes()),
        )
    }
}

impl Serialize for AccessToken {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let token = self.encoded().map_err(serde::ser::Error::custom)?;
        let mut entry = serializer.serialize_struct("AccessToken", 1)?;
        entry.serialize_field("access_token", &token)?;
        entry.end()
    }
}