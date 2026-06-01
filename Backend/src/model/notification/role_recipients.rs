use crate::model::project::contract::list::List;
use crate::model::user::role::Role;
use sqlx::PgPool;
use std::sync::Arc;

pub struct RoleRecipients {
    pool: Arc<PgPool>,
    role: Role,
}

impl RoleRecipients {
    pub fn new(pool: Arc<PgPool>, role: Role) -> Self {
        Self { pool, role }
    }
}

#[async_trait::async_trait]
impl List for RoleRecipients {
    type Output = String;

    async fn items(&self) -> Result<Vec<String>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            email: String,
        }
        let rows = sqlx::query_as::<_, Row>(
            "SELECT u.email FROM users u
             JOIN user_roles r ON r.user_id = u.id
             WHERE r.role = $1 AND u.notifications_enabled = TRUE",
        )
        .bind(&self.role)
        .fetch_all(self.pool.as_ref())
        .await?;
        Ok(rows.into_iter().map(|r| r.email).collect())
    }
}
