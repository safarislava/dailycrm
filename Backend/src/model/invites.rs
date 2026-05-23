use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct Invites {
    pool: PgPool,
}

pub enum RegisterWithInviteResult {
    Ok,
    InvalidInvite,
    UserExists,
}

impl Invites {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, created_by: Uuid) -> Result<Uuid, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row { token: Uuid }
        let row: Row = sqlx::query_as(
            "INSERT INTO invites (created_by) VALUES ($1) RETURNING token",
        )
        .bind(created_by)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.token)
    }

    pub async fn consume_and_register(
        &self,
        token: Uuid,
        username: &str,
        password_hash: &str,
    ) -> Result<RegisterWithInviteResult, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let rows = sqlx::query(
            "UPDATE invites SET used_at = NOW() \
             WHERE token = $1 AND used_at IS NULL AND expires_at > NOW()",
        )
        .bind(token)
        .execute(&mut *tx)
        .await?
        .rows_affected();

        if rows == 0 {
            tx.rollback().await?;
            return Ok(RegisterWithInviteResult::InvalidInvite);
        }

        let result = sqlx::query("INSERT INTO users (username, password_hash) VALUES ($1, $2)")
            .bind(username)
            .bind(password_hash)
            .execute(&mut *tx)
            .await;

        match result {
            Ok(_) => {
                tx.commit().await?;
                Ok(RegisterWithInviteResult::Ok)
            }
            Err(sqlx::Error::Database(_)) => {
                tx.rollback().await?;
                Ok(RegisterWithInviteResult::UserExists)
            }
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }
}