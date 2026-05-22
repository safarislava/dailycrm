use sqlx::PgPool;

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, username: &str, password_hash: &str) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO users (username, password_hash) VALUES ($1, $2)")
            .bind(username)
            .bind(password_hash)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
