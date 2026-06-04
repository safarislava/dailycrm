use crate::model::user::contract::invite::Invite;
use uuid::Uuid;

pub struct InviteCode {
    token: Uuid,
}

impl InviteCode {
    pub fn new(token: Uuid) -> Self {
        Self { token }
    }
}

impl Invite for InviteCode {
    fn token(&self) -> Uuid {
        self.token
    }
}