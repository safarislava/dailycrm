use crate::common::BoxError;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;

pub struct ProjectRegistration {
    pool: Arc<PgPool>,
    title: String,
}

impl ProjectRegistration {
    pub fn new(pool: Arc<PgPool>, title: String) -> Self {
        Self { pool, title }
    }
}

#[async_trait::async_trait]
impl Task for ProjectRegistration {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        sqlx::query("INSERT INTO projects (title) VALUES ($1)")
            .bind(&self.title)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
