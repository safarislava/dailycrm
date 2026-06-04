use crate::common::BoxError;
use crate::model::credential::hash_user_verification::VerificationError;
use crate::model::session::access_token::AccessToken;
use crate::model::session::new_token::NewToken;
use crate::model::session::refresh_token::{REFRESH_LIFETIME, RefreshToken};
use crate::model::session::token_kind::TokenKind;
use crate::model::task::contract::task::Task;
use crate::model::task::session::refresh_token_submission::RefreshTokenSubmission;
use crate::model::user::contract::protected_user::ProtectedUser;
use chrono::{Duration, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct TokenIssuance {
    pool: Arc<PgPool>,
    protected_user: Box<dyn ProtectedUser>,
}

impl TokenIssuance {
    pub fn new(pool: Arc<PgPool>, protected_user: Box<dyn ProtectedUser>) -> Self {
        Self {
            pool,
            protected_user,
        }
    }
}

#[async_trait::async_trait]
impl Task for TokenIssuance {
    type Output = Option<(AccessToken, RefreshToken)>;

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let user = match self.protected_user.unprotected().await {
            Ok(user) => user,
            Err(VerificationError::Wrong) => return Ok(None),
            Err(other) => return Err(BoxError::from(other)),
        };
        let access_token = AccessToken::new(Box::new(NewToken::new(
            user.id(),
            Uuid::new_v4(),
            TokenKind::Access,
            Utc::now() + Duration::minutes(15),
        )));
        let jti = Uuid::new_v4();
        let refresh_expires_at = Utc::now() + REFRESH_LIFETIME;
        let refresh_token = RefreshToken::new(
            jti,
            Box::new(NewToken::new(
                user.id(),
                jti,
                TokenKind::Refresh,
                refresh_expires_at,
            )),
        );
        RefreshTokenSubmission::new(self.pool.clone(), user.id(), jti, refresh_expires_at)
            .done()
            .await?;
        Ok(Some((access_token, refresh_token)))
    }
}
