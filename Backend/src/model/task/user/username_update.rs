use crate::common::BoxError;
use crate::model::credential::contract::contentable::Contentable;
use crate::model::credential::valid_username::ValidUsername;
use crate::model::task::contract::task::Task;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct UsernameUpdate {
    pool: Arc<PgPool>,
    user: User,
    new_username: ValidUsername,
}

impl UsernameUpdate {
    pub fn new(pool: Arc<PgPool>, user: User, new_username: ValidUsername) -> Self {
        Self {
            pool,
            user,
            new_username,
        }
    }
}

#[async_trait::async_trait]
impl Task for UsernameUpdate {
    type Output = ();

    async fn done(&self) -> Result<(), BoxError> {
        let result = sqlx::query("UPDATE users SET username = $2 WHERE id = $1")
            .bind(self.user.id())
            .bind(self.new_username.content().await?)
            .execute(self.pool.as_ref())
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
