use crate::model::invites::Invites;
use crate::model::projects::Projects;
use crate::model::refresh_tokens::RefreshTokens;
use crate::model::users::Users;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub users: Users,
    pub projects: Projects,
    pub invites: Invites,
    pub refresh_tokens: RefreshTokens,
}
