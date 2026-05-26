use crate::model::password_hash::PasswordHash;
use crate::model::username::ValidUsername;
use sqlx::PgPool;
use uuid::Uuid;

pub struct Invites;

pub enum RegisterWithInviteResult {
    Ok,
    InvalidInvite,
    UserExists,
}

impl Invites {
    pub async fn create(&self, created_by: Uuid, pool: &PgPool) -> Result<Uuid, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row { token: Uuid }
        let row: Row = sqlx::query_as("INSERT INTO invites (created_by) VALUES ($1) RETURNING token")
            .bind(created_by)
            .fetch_one(pool)
            .await?;
        Ok(row.token)
    }

    pub async fn consume_and_register(
        &self,
        token: Uuid,
        username: &ValidUsername,
        password_hash: &PasswordHash,
        pool: &PgPool,
    ) -> Result<RegisterWithInviteResult, sqlx::Error> {
        let mut transaction = pool.begin().await?;

        let rows = sqlx::query(
            "UPDATE invites SET used_at = NOW() \
             WHERE token = $1 AND used_at IS NULL AND expires_at > NOW()",
        )
        .bind(token)
        .execute(&mut *transaction)
        .await?
        .rows_affected();

        if rows == 0 {
            transaction.rollback().await?;
            return Ok(RegisterWithInviteResult::InvalidInvite);
        }

        let result = sqlx::query("INSERT INTO users (username, password_hash) VALUES ($1, $2)")
            .bind(username.as_str())
            .bind(password_hash.as_str())
            .execute(&mut *transaction)
            .await;

        match result {
            Ok(_) => {
                transaction.commit().await?;
                Ok(RegisterWithInviteResult::Ok)
            }
            Err(sqlx::Error::Database(_)) => {
                transaction.rollback().await?;
                Ok(RegisterWithInviteResult::UserExists)
            }
            Err(e) => {
                transaction.rollback().await?;
                Err(e)
            }
        }
    }
}