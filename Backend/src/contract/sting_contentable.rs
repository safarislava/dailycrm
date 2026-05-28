use crate::common::BoxError;

pub trait StringContentable {
    fn content(&self) -> Result<String, BoxError>;
}