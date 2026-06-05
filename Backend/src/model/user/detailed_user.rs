use crate::common::BoxError;
use crate::model::project::contract::json::Json;
use crate::model::user::role::Role;
use crate::model::user::user::User;
use serde::Serialize;
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

    async fn profile(&self) -> Result<Profile, sqlx::Error> {
        sqlx::query_as::<_, Profile>(
            "SELECT username, email, notifications_enabled FROM users WHERE id = $1",
        )
        .bind(self.user.id())
        .fetch_one(self.pool.as_ref())
        .await
    }

    async fn roles(&self) -> Result<Vec<Role>, sqlx::Error> {
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

#[derive(sqlx::FromRow, Serialize)]
struct Profile {
    username: String,
    email: String,
    notifications_enabled: bool,
}

#[async_trait::async_trait]
impl Json for DetailedUser {
    async fn json(&self) -> Result<serde_json::Value, BoxError> {
        let (profile, roles) = futures_util::try_join!(self.profile(), self.roles())?;
        Ok(serde_json::json!({
            "username": profile.username,
            "email": profile.email,
            "notifications_enabled": profile.notifications_enabled,
            "roles": roles,
        }))
    }
}