use crate::common::BoxError;
use crate::model::session::contract::token::Token;
use chrono::{Duration, TimeDelta};
use uuid::Uuid;

pub const REFRESH_LIFETIME: TimeDelta = Duration::weeks(1);

pub struct RefreshToken {
    id: Uuid,
    token: Box<dyn Token>,
}

impl RefreshToken {
    pub fn new(id: Uuid, token: Box<dyn Token>) -> Self {
        Self { id, token }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

#[async_trait::async_trait]
impl Token for RefreshToken {
    async fn value(&self) -> Result<String, BoxError> {
        self.token.value().await
    }
}