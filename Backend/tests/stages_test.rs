mod common;

use actix_web::test::{self, TestRequest};
use serde_json::json;

#[actix_web::test]
async fn test_get_stages_returns_unauthorized_without_auth() {
    let app = init_test_app!();

    let req = TestRequest::get()
        .uri("/api/projects/1/stages")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_create_stage_returns_unauthorized_without_auth() {
    let app = init_test_app!();

    let payload = json!({
        "title": "Stage 1"
    });

    let req = TestRequest::post()
        .uri("/api/projects/1/stages")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_create_payment_tranche_returns_unauthorized_without_auth() {
    let app = init_test_app!();

    let payload = json!({
        "amount": "10000"
    });

    let req = TestRequest::post()
        .uri("/api/projects/1/stages/1/advance-tranches")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_delete_payment_tranche_returns_unauthorized_without_auth() {
    let app = init_test_app!();

    let req = TestRequest::delete()
        .uri("/api/projects/1/stages/1/advance-tranches/00000000-0000-0000-0000-000000000000")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}
