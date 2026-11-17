mod common;

use actix_web::http::header;
use actix_web::test::{self, TestRequest};
use dailycrm::jwt::jwt_secret;
use dailycrm::model::session::claims::Claims;
use jsonwebtoken::{EncodingKey, Header, encode};
use uuid::Uuid;

fn auth_token() -> String {
    let user_id = Uuid::new_v4();
    let claims = Claims::new(user_id, Uuid::new_v4(), "access".to_string(), 9999999999);
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
    .unwrap()
}

#[actix_web::test]
async fn test_get_archived_projects_returns_unauthorized_without_auth() {
    let app = init_test_app!();

    let req = TestRequest::get().uri("/api/archive/projects").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_archive_project_returns_unauthorized_without_auth() {
    let app = init_test_app!();

    let project_id = Uuid::new_v4();
    let req = TestRequest::post()
        .uri(&format!("/api/projects/{project_id}/archive"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_restore_archived_project_returns_unauthorized_without_auth() {
    let app = init_test_app!();

    let project_id = Uuid::new_v4();
    let req = TestRequest::post()
        .uri(&format!("/api/archive/projects/{project_id}/restore"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_get_archived_stages_returns_unauthorized_without_auth() {
    let app = init_test_app!();

    let project_id = Uuid::new_v4();
    let req = TestRequest::get()
        .uri(&format!("/api/archive/projects/{project_id}/stages"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_get_archived_comments_returns_unauthorized_without_auth() {
    let app = init_test_app!();

    let project_id = Uuid::new_v4();
    let req = TestRequest::get()
        .uri(&format!(
            "/api/archive/projects/{project_id}/stages/1/comments"
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_archive_returns_not_found_for_nonexistent_project() {
    let app = init_test_app!();
    let token = auth_token();

    let non_existent_id = Uuid::new_v4();
    let req = TestRequest::post()
        .uri(&format!("/api/projects/{non_existent_id}/archive"))
        .insert_header((header::AUTHORIZATION, format!("Bearer {token}")))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 500);
}

#[actix_web::test]
async fn test_restore_returns_not_found_for_nonexistent_archived_project() {
    let app = init_test_app!();
    let token = auth_token();

    let non_existent_id = Uuid::new_v4();
    let req = TestRequest::post()
        .uri(&format!("/api/archive/projects/{non_existent_id}/restore"))
        .insert_header((header::AUTHORIZATION, format!("Bearer {token}")))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 500);
}

#[actix_web::test]
async fn test_get_archived_projects_returns_ok_with_auth() {
    let app = init_test_app!();
    let token = auth_token();

    let req = TestRequest::get()
        .uri("/api/archive/projects")
        .insert_header((header::AUTHORIZATION, format!("Bearer {token}")))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 200);
}
