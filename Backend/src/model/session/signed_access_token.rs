use crate::model::session::claims::Claims;
use crate::model::session::contract::user_id_source::UserIdSource;
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
        Claims::from(&self.raw).map(|c| c.sub())
    }
}
