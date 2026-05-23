pub mod login;
pub mod logout;
pub mod refresh;

use serde::Serialize;

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
}