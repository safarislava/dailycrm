use crate::model::hash_verification::VerificationError as LoginError;
use crate::model::password::Password;
use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use crate::model::valid_password::ValidPassword;

#[derive(Deserialize)]
pub struct LoginDto {
    username: String,
    password: String,
}

pub async fn post(state: web::Data<AppState>, body: web::Json<LoginDto>) -> impl Responder {
    let user = match state.users.user_by_username(&body.username).await {
        Ok(Some(u)) => u,
        Ok(None) => return HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
    };

    let password = ValidPassword::new(Password::new(body.password.clone()));
    let (access_token, refresh_token) = match user
        .confirmed(password)
        .tokens(state.refresh_tokens.as_ref())
        .await
    {
        Ok(tokens) => tokens,
        Err(LoginError::WrongPassword) => {
            return HttpResponse::Unauthorized().body("Invalid credentials");
        }
        Err(LoginError::Internal) => {
            return HttpResponse::InternalServerError().body("Something went wrong");
        }
    };

    HttpResponse::Ok().cookie(refresh_token.cookie()).json(access_token)
}
