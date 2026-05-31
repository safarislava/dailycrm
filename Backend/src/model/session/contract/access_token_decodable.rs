use uuid::Uuid;

pub trait AccessTokenDecodable {
    fn user_id(&self) -> Option<Uuid>;
}
