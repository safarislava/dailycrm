use crate::common::BoxError;
use crate::model::session::refresh_token::RefreshToken;
use crate::model::task::contract::task::Task;
use sqlx::PgPool;
use std::sync::Arc;

pub struct RefreshTokenRevocation {
    pool: Arc<PgPool>,
    refresh_token: RefreshToken,
}

impl RefreshTokenRevocation {
    pub fn new(pool: Arc<PgPool>, refresh_token: RefreshToken) -> Self {
        RefreshTokenRevocation {
            pool,
            refresh_token,
        }
    }
}

#[async_trait::async_trait]
impl Task for RefreshTokenRevocation {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        sqlx::query(
            "UPDATE refresh_tokens SET revoked_at = NOW() \
             WHERE jti = $1 AND revoked_at IS NULL",
        )
        .bind(self.refresh_token.id())
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
}
