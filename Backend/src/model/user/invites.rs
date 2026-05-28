use crate::common::BoxError;
use crate::contract::Invites;
use crate::contract::contentable::Contentable;
use crate::model::credential::hash::Hash;
use crate::model::credential::valid_username::ValidUsername;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

pub enum RegisterWithInviteResult {
    Ok,
    InvalidInvite,
    UserExists,
}

pub struct PgInvites {
    pool: PgPool,
}

impl PgInvites {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Invites for PgInvites {
    async fn create(&self, created_by: Uuid) -> Result<Uuid, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            token: Uuid,
        }
        let row: Row =
            sqlx::query_as("INSERT INTO invites (created_by) VALUES ($1) RETURNING token")
                .bind(created_by)
                .fetch_one(&self.pool)
                .await?;
        Ok(row.token)
    }

    async fn consume_and_register(
        &self,
        token: Uuid,
        username: &ValidUsername,
        password_hash: &Hash,
    ) -> Result<RegisterWithInviteResult, BoxError> {
        let mut transaction = self.pool.begin().await?;

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
            .bind(username.content().await?)
            .bind(password_hash.content().await?)
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
                Err(Box::new(e))
            }
        }
    }
}
