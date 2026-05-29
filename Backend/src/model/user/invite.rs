use uuid::Uuid;
use crate::common::BoxError;
use crate::model::credential::contract::contentable::Contentable;

pub struct Invite {
    token: Uuid,
}

impl Invite {
    pub fn new(token: Uuid) -> Invite {
        Invite { token }
    }
}

#[async_trait::async_trait]
impl Contentable for Invite {
    type Output = Uuid;

    async fn content(&self) -> Result<Self::Output, BoxError> {
        Ok(self.token)
    }
}