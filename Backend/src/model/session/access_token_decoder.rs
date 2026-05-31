use crate::auth::decoded_claims;
use crate::model::session::contract::access_token_decodable::AccessTokenDecodable;
use crate::model::session::token_kind::TokenKind;
use uuid::Uuid;

pub struct AccessTokenDecoder {
    raw: String,
}

impl AccessTokenDecoder {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }
}

impl AccessTokenDecodable for AccessTokenDecoder {
    fn user_id(&self) -> Option<Uuid> {
        decoded_claims(&self.raw)
            .filter(|c| c.typ == TokenKind::Access.as_str())
            .map(|c| c.sub)
    }
}
