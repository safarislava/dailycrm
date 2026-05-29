use crate::common::BoxError;
use crate::model::credential::contract::contentable::Contentable;
use crate::model::credential::valid_username::ValidUsername;
use crate::model::task::task::Task;
use crate::model::user::user::User;
use sqlx::PgPool;

pub struct UsernameUpdate {
    pool: PgPool,
    user: User,
    new_username: ValidUsername,
}

impl UsernameUpdate {
    pub fn new(pool: PgPool, user: User, new_username: ValidUsername) -> Self {
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

    async fn output(&self) -> Result<(), BoxError> {
        let result = sqlx::query("UPDATE users SET username = $2 WHERE id = $1")
            .bind(self.user.id())
            .bind(self.new_username.content().await?)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
