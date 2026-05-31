use crate::auth::decoded_claims;
use crate::model::session::contract::jti_source::JtiSource;
use crate::model::session::token_kind::TokenKind;
use uuid::Uuid;

pub struct SignedRefreshToken {
    raw: String,
}

impl SignedRefreshToken {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }
}

impl JtiSource for SignedRefreshToken {
    fn jti(&self) -> Option<Uuid> {
        decoded_claims(&self.raw)
            .filter(|c| c.typ == TokenKind::Refresh.as_str())
            .map(|c| c.jti)
    }
}
