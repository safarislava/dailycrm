use crate::common::BoxError;
use crate::model::credential::contract::username::Username;
use crate::model::task::contract::task::Task;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct UsernameUpdate {
    pool: Arc<PgPool>,
    user: User,
    new_username: Box<dyn Username>,
}

impl UsernameUpdate {
    pub fn new(pool: Arc<PgPool>, user: User, new_username: impl Username) -> Self {
        Self {
            pool,
            user,
            new_username: Box::new(new_username),
        }
    }
}

#[async_trait::async_trait]
impl Task for UsernameUpdate {
    type Output = ();

    async fn done(&self) -> Result<(), BoxError> {
        let result = sqlx::query("UPDATE users SET username = $2 WHERE id = $1")
            .bind(self.user.id())
            .bind(self.new_username.value()?)
            .execute(self.pool.as_ref())
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}