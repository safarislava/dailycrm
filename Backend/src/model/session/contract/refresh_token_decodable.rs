use uuid::Uuid;

pub trait RefreshTokenDecodable {
    fn jti(&self) -> Option<Uuid>;
}
