use uuid::Uuid;

pub trait JtiSource {
    fn jti(&self) -> Option<Uuid>;
}
