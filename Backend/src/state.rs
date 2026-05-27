use crate::contract;
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState {
    pub pool: PgPool,
    pub users: Arc<dyn contract::Users>,
    pub projects: Arc<dyn contract::Projects>,
    pub invites: Arc<dyn contract::Invites>,
    pub refresh_tokens: Arc<dyn contract::RefreshTokens>,
    pub deadlines: Arc<dyn contract::Deadlines>,
    pub attachments: Arc<dyn contract::Attachments>,
    pub stages: Arc<dyn contract::Stages>,
    pub stage_fields: Arc<dyn contract::StageFields>,
}
