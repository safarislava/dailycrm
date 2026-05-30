use crate::model::project::contract::list::List;
use sqlx::PgPool;
use std::sync::Arc;

pub struct Recipients {
    pool: Arc<PgPool>,
}

impl Recipients {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl List for Recipients {
    type Output = String;

    async fn items(&self) -> Result<Vec<String>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct EmailRow {
            email: String,
        }
        let rows = sqlx::query_as::<_, EmailRow>(
            "SELECT email FROM users
             WHERE email IS NOT NULL AND notifications_enabled = TRUE",
        )
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows.into_iter().map(|row| row.email).collect())
    }
}
