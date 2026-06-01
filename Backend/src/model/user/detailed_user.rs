use crate::model::credential::username::Username;
use crate::model::user::role::Role;
use crate::model::user::user::User;
use sqlx::PgPool;
use std::sync::Arc;

pub struct DetailedUser {
    pool: Arc<PgPool>,
    user: User,
}

impl DetailedUser {
    pub fn new(pool: Arc<PgPool>, user: User) -> DetailedUser {
        DetailedUser { pool, user }
    }

    pub async fn username(&self) -> Result<Option<Username>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            username: String,
        }
        let row = sqlx::query_as::<_, Row>("SELECT username FROM users WHERE id = $1")
            .bind(self.user.id())
            .fetch_optional(self.pool.as_ref())
            .await?;
        Ok(row.map(|r| Username::new(r.username)))
    }

    pub async fn email(&self) -> Result<Option<String>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            email: String,
        }
        let row = sqlx::query_as::<_, Row>("SELECT email FROM users WHERE id = $1")
            .bind(self.user.id())
            .fetch_optional(self.pool.as_ref())
            .await?;
        Ok(row.map(|r| r.email))
    }

    pub async fn notifications_enabled(&self) -> Result<Option<bool>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            notifications_enabled: bool,
        }
        let row = sqlx::query_as::<_, Row>("SELECT notifications_enabled FROM users WHERE id = $1")
            .bind(self.user.id())
            .fetch_optional(self.pool.as_ref())
            .await?;
        Ok(row.map(|r| r.notifications_enabled))
    }

    pub async fn roles(&self) -> Result<Vec<Role>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            role: Role,
        }
        let rows = sqlx::query_as::<_, Row>("SELECT role FROM user_roles WHERE user_id = $1")
            .bind(self.user.id())
            .fetch_all(self.pool.as_ref())
            .await?;
        Ok(rows.into_iter().map(|r| r.role).collect())
    }
}
