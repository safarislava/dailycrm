use crate::model::notification::contract::message::Message;

#[async_trait::async_trait]
pub trait Digest: Message {
    fn is_empty(&self) -> bool;
}