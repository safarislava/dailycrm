use crate::model::session::claims::Claims;
use crate::model::session::contract::jti_source::JtiSource;
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
        Claims::from(&self.raw).map(|c| c.jti())
    }
}
