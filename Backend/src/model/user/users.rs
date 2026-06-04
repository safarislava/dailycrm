use crate::common::BoxError;
use crate::model::project::contract::list::List;
use crate::model::user::contract::username_search::UsernameSearch;
use crate::model::user::user::User;
use sqlx::{Error, PgPool};
use std::sync::Arc;
use uuid::Uuid;
use crate::model::credential::contract::username::Username;

pub struct Users {
    pool: Arc<PgPool>,
}

impl Users {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UsernameSearch for Users {
    async fn found(&self, username: impl Username) -> Result<Option<User>, BoxError> {
        let id = sqlx::query_scalar::<_, Uuid>("SELECT id FROM users WHERE username = $1")
            .bind(username.value()?)
            .fetch_optional(self.pool.as_ref())
            .await?;

        Ok(id.map(|id| User::new(id)))
    }
}

#[async_trait::async_trait]
impl List for Users {
    type Output = User;

    async fn items(&self) -> Result<Vec<Self::Output>, Error> {
        let ids = sqlx::query_scalar::<_, Uuid>("SELECT id FROM users")
            .fetch_all(self.pool.as_ref())
            .await?;
        Ok(ids.into_iter().map(|id| User::new(id)).collect())
    }
}
