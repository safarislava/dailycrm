use crate::contract;
use std::sync::Arc;

pub struct AppState {
    pub pool: sqlx::PgPool,
    pub users: Arc<dyn contract::Users>,
    pub projects: Arc<dyn contract::Projects>,
    pub invites: Arc<dyn contract::Invites>,
    pub refresh_tokens: Arc<dyn contract::RefreshTokens>,
    pub deadlines: Arc<dyn contract::Deadlines>,
}
