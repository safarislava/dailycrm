use crate::common::BoxError;

pub trait Task {
    type Output;

    async fn output(&self) -> Result<Self::Output, BoxError>;
}
