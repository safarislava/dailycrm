use crate::model::credential::username::Username;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct User {
    pool: PgPool,
    id: Uuid,
}

impl User {
    pub fn new(pool: PgPool, id: Uuid) -> Self {
        Self { pool, id }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub async fn username(&self) -> Result<Option<Username>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            username: String,
        }
        let row = sqlx::query_as::<_, Row>("SELECT username FROM users WHERE id = $1")
            .bind(self.id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| Username::new(r.username)))
    }
}
