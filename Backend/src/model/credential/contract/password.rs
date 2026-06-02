use crate::common::BoxError;

pub trait Password: Send + Sync + 'static {
    fn value(&self) -> Result<String, BoxError>;
}