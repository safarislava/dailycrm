use sqlx::PgPool;

#[derive(Clone)]
pub struct Users {
    pool: PgPool,
}

impl Users {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn register(&self, username: &str, password_hash: &str) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO users (username, password_hash) VALUES ($1, $2)")
            .bind(username)
            .bind(password_hash)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
