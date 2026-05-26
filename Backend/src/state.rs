use crate::model::invites::Invites;
use crate::model::projects::Projects;
use crate::model::refresh_tokens::RefreshTokens;
use crate::model::users::Users;
use crate::storage::Storage;
use sqlx::PgPool;

pub struct AppState {
    pub pool: PgPool,
    pub storage: Storage,
    pub users: Users,
    pub projects: Projects,
    pub invites: Invites,
    pub refresh_tokens: RefreshTokens,
}