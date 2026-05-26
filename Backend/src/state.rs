use crate::model::attachments::Attachments;
use crate::model::invites::Invites;
use crate::model::projects::Projects;
use crate::model::refresh_tokens::RefreshTokens;
use crate::model::users::Users;

#[derive(Clone)]
pub struct AppState {
    pub users: Users,
    pub projects: Projects,
    pub invites: Invites,
    pub attachments: Attachments,
    pub refresh_tokens: RefreshTokens,
}