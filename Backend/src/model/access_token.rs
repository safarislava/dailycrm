use crate::auth::create_access_token;
use uuid::Uuid;

pub struct AccessToken {
    pub token_string: String,
}

impl AccessToken {
    pub fn new(user_id: Uuid) -> Result<Self, jsonwebtoken::errors::Error> {
        Ok(Self { token_string: create_access_token(user_id)? })
    }
}