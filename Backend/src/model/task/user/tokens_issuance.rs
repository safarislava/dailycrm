use crate::contract::RefreshTokens;
use crate::contract::task::Task;
use crate::model::credential::hash_verification::VerificationError;
use crate::model::session::access_token::AccessToken;
use crate::model::session::refresh_token::NewRefreshToken;
use aws_sdk_s3::error::BoxError;
use std::sync::Arc;
use crate::contract::protected::Protected;
use crate::model::user::user::User;

pub struct TokenIssuance {
    refresh_tokens: Arc<dyn RefreshTokens>,
    protected_user: Box<dyn Protected<Output=User>>,
}

impl TokenIssuance {
    pub fn new(refresh_tokens: Arc<dyn RefreshTokens>, protected_user: Box<dyn Protected<Output=User>>) -> Self {
        Self {
            refresh_tokens,
            protected_user,
        }
    }
}

impl Task for TokenIssuance {
    type Output = (AccessToken, NewRefreshToken);
    async fn output(&self) -> Result<Self::Output, BoxError> {
        let user = self.protected_user.unprotected().await?;
        let access_token = AccessToken::new(user.id());
        let refresh_token = self
            .refresh_tokens
            .new_token(user.id())
            .await
            .map_err(|_| VerificationError::Internal)?;
        Ok((access_token, refresh_token))
    }
}
