use chrono::{Duration, TimeDelta};
use crate::common::BoxError;
use crate::model::credential::contract::contentable::Contentable;
use crate::model::session::contract::token::Token;
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
impl Contentable for RefreshToken {
    type Output = String;

    async fn content(&self) -> Result<Self::Output, BoxError> {
        Ok(self.token.content().await?)
    }
}

impl Token for RefreshToken {}
