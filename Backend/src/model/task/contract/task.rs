use crate::common::BoxError;

#[async_trait::async_trait]
pub trait Task {
    type Output;

    async fn done(&self) -> Result<Self::Output, BoxError>;
}
