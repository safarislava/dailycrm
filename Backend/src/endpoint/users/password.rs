use crate::auth::UserIdGettable;
use crate::model::authorized_user::UpdatePasswordError;
use crate::model::password::Password;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdatePasswordDto {
    current_password: String,
    new_password: String,
}

pub async fn patch(
    state: web::Data<AppState>,
    request: HttpRequest,
    body: web::Json<UpdatePasswordDto>,
) -> impl Responder {
    let user_id = match request.user_id() {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let current_password = match Password(body.current_password.clone()).validated() {
        Ok(p) => p,
        Err(_) => return HttpResponse::Unauthorized().body("Wrong current password"),
    };

    let new_password = match Password(body.new_password.clone()).validated() {
        Ok(p) => p,
        Err(e) => return HttpResponse::UnprocessableEntity().body(e.message()),
    };

    let user = state.users.user(user_id).confirming(current_password);

    match user.update_password(new_password).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(UpdatePasswordError::WrongPassword) => {
            HttpResponse::Unauthorized().body("Wrong current password")
        }
        Err(UpdatePasswordError::Internal) => {
            HttpResponse::InternalServerError().body("Something went wrong")
        }
    }
}
