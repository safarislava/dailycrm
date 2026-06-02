use crate::common::BoxError;
use crate::model::credential::contract::hash::Hash;
use crate::model::credential::contract::username::Username;
use crate::model::task::contract::task::Task;
use crate::model::user::contract::invite::Invite;
use crate::model::user::invite::InviteCode;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;

pub struct InviteConsumption {
    pool: Arc<PgPool>,
    invite: InviteCode,
    username: Box<dyn Username>,
    password: Box<dyn Hash>,
    email: String,
}

impl InviteConsumption {
    pub fn new(
        pool: Arc<PgPool>,
        invite: InviteCode,
        username: impl Username,
        password: impl Hash,
        email: String,
    ) -> Self {
        Self {
            pool,
            invite,
            username: Box::new(username),
            password: Box::new(password),
            email,
        }
    }
}

impl InviteConsumption {
    async fn invite_exists(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<bool, BoxError> {
        Ok(sqlx::query(
            "UPDATE invites SET used_at = NOW() \
             WHERE token = $1 AND used_at IS NULL AND expires_at > NOW()",
        )
        .bind(self.invite.token())
        .execute(&mut **transaction)
        .await?
        .rows_affected()
            > 0)
    }
}

#[async_trait::async_trait]
impl Task for InviteConsumption {
    type Output = InviteStatus;

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let mut transaction = self.pool.begin().await?;
        if !self.invite_exists(&mut transaction).await? {
            transaction.rollback().await?;
            return Ok(InviteStatus::InvalidInvite);
        }
        let result =
            sqlx::query("INSERT INTO users (username, password_hash, email) VALUES ($1, $2, $3)")
                .bind(self.username.value()?)
                .bind(self.password.value().await?)
                .bind(&self.email)
                .execute(&mut *transaction)
                .await;
        match result {
            Ok(_) => {
                transaction.commit().await?;
                Ok(InviteStatus::Ok)
            }
            Err(sqlx::Error::Database(_)) => {
                transaction.rollback().await?;
                Ok(InviteStatus::UserExists)
            }
            Err(e) => {
                transaction.rollback().await?;
                Err(Box::new(e))
            }
        }
    }
}

pub enum InviteStatus {
    Ok,
    InvalidInvite,
    UserExists,
}