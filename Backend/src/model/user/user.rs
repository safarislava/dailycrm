use crate::common::BoxError;
use crate::contract::contentable::Contentable;
use crate::model::user::authorized_user::ConfirmedUser;
use crate::model::credential::username::Username;
use crate::model::credential::valid_password::ValidPassword;
use crate::model::credential::valid_username::ValidUsername;
use sqlx::PgPool;
use uuid::Uuid;

pub struct User {
    pool: PgPool,
    id: Uuid,
}

impl User {
    pub fn new(pool: PgPool, id: Uuid) -> Self {
        Self { pool, id }
    }

    pub fn confirmed(&self, password: ValidPassword) -> ConfirmedUser {
        ConfirmedUser::new(self.pool.clone(), self.id, password)
    }

    pub async fn username(&self) -> Result<Option<Username>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            username: String,
        }
        let row = sqlx::query_as::<_, Row>("SELECT username FROM users WHERE id = $1")
            .bind(self.id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| Username::new(r.username)))
    }

    pub async fn update_username(&self, new_username: &ValidUsername) -> Result<bool, BoxError> {
        let result = sqlx::query("UPDATE users SET username = $2 WHERE id = $1")
            .bind(self.id)
            .bind(new_username.content().await?)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(true),
            Err(sqlx::Error::Database(_)) => Ok(false),
            Err(e) => Err(Box::new(e)),
        }
    }
}
