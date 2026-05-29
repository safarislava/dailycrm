use crate::model::project::contract::deadlines::Deadlines;
use crate::model::project::contract::projects::Projects;
use crate::model::project::contract::users::Users;
use crate::model::session::contract::refresh_tokens::RefreshTokens;
use std::sync::Arc;

pub struct AppState {
    pub pool: sqlx::PgPool,
    pub users: Arc<dyn Users>,
    pub projects: Arc<dyn Projects>,
    pub refresh_tokens: Arc<dyn RefreshTokens>,
    pub deadlines: Arc<dyn Deadlines>,
}
