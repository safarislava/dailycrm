use crate::common::BoxError;
use crate::model::task::task::Task;
use crate::model::user::invite::Invite;
use crate::model::user::user::User;
use sqlx::PgPool;
use uuid::Uuid;

pub struct InviteCreation {
    pool: PgPool,
    user: User,
}

impl InviteCreation {
    pub fn new(pool: PgPool, user: User) -> Self {
        Self { pool, user }
    }
}

#[async_trait::async_trait]
impl Task for InviteCreation {
    type Output = Invite;

    async fn output(&self) -> Result<Self::Output, BoxError> {
        #[derive(sqlx::FromRow)]
        struct Row {
            token: Uuid,
        }
        let row: Row =
            sqlx::query_as("INSERT INTO invites (created_by) VALUES ($1) RETURNING token")
                .bind(self.user.id())
                .fetch_one(&self.pool)
                .await?;
        Ok(Invite::new(row.token))
    }
}
