use crate::auth::decoded_claims;
use crate::model::session::contract::refresh_token_decodable::RefreshTokenDecodable;
use crate::model::session::token_kind::TokenKind;
use uuid::Uuid;

pub struct RefreshTokenDecoder {
    raw: String,
}

impl RefreshTokenDecoder {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }
}

impl RefreshTokenDecodable for RefreshTokenDecoder {
    fn jti(&self) -> Option<Uuid> {
        decoded_claims(&self.raw)
            .filter(|c| c.typ == TokenKind::Refresh.as_str())
            .map(|c| c.jti)
    }
}
