use crate::common::BoxError;

pub trait Username: Send + Sync + 'static {
    fn value(&self) -> Result<String, BoxError>;
}