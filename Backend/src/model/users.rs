use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct Users {
    pool: PgPool,
}

impl Users {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn register(&self, username: &str, password_hash: &str) -> Result<Uuid, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row { id: Uuid }
        let row: Row = sqlx::query_as(
            "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id",
        )
        .bind(username)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.id)
    }

    pub async fn find_by_username(
        &self,
        username: &str,
    ) -> Result<Option<(Uuid, String)>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            id: Uuid,
            password_hash: String,
        }
        let row = sqlx::query_as::<_, Row>(
            "SELECT id, password_hash FROM users WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|r| (r.id, r.password_hash)))
    }
}