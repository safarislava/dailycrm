use crate::auth::decoded_claims;
use crate::model::session::contract::user_id_source::UserIdSource;
use crate::model::session::token_kind::TokenKind;
use uuid::Uuid;

pub struct SignedAccessToken {
    raw: String,
}

impl SignedAccessToken {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }
}

impl UserIdSource for SignedAccessToken {
    fn user_id(&self) -> Option<Uuid> {
        decoded_claims(&self.raw)
            .filter(|c| c.typ == TokenKind::Access.as_str())
            .map(|c| c.sub)
    }
}
