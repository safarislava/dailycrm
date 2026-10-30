use crate::endpoint::api_error::ApiError;
use crate::model::project::id::ProjectId;
use crate::model::project::stage::id::StageId;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest, web};
use futures_util::future::{Ready, ready};

impl FromRequest for StageId {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Ok(path) = web::Path::<(ProjectId, i32, i32, uuid::Uuid)>::extract(req).into_inner()
        {
            let (project_id, parent_position, position, _) = path.into_inner();
            return ready(Ok(StageId::new_substage(
                project_id,
                parent_position,
                position,
            )));
        }
        if let Ok(path) = web::Path::<(ProjectId, i32, i32)>::extract(req).into_inner() {
            let (project_id, parent_position, position) = path.into_inner();
            return ready(Ok(StageId::new_substage(
                project_id,
                parent_position,
                position,
            )));
        }
        if let Ok(path) = web::Path::<(ProjectId, i32, uuid::Uuid)>::extract(req).into_inner() {
            let (project_id, position, _) = path.into_inner();
            return ready(Ok(StageId::new(project_id, position)));
        }
        if let Ok(path) = web::Path::<(ProjectId, i32)>::extract(req).into_inner() {
            let (project_id, position) = path.into_inner();
            return ready(Ok(StageId::new(project_id, position)));
        }
        ready(Err(ApiError::BadRequest(
            "Invalid stage id path".to_string(),
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::TestRequest;
    use uuid::Uuid;

    #[actix_web::test]
    async fn extracts_top_level_stage_id_from_two_tuple_path() {
        let uuid = Uuid::new_v4();
        let req = TestRequest::default()
            .param("project_id", uuid.to_string())
            .param("position", "2")
            .to_http_request();

        let mut payload = Payload::None;
        let res = StageId::from_request(&req, &mut payload).await;

        let expected = StageId::new(ProjectId::new(uuid), 2);
        assert_eq!(res.unwrap(), expected);
    }

    #[actix_web::test]
    async fn extracts_substage_id_from_three_tuple_path() {
        let uuid = Uuid::new_v4();
        let req = TestRequest::default()
            .param("project_id", uuid.to_string())
            .param("parent_position", "2")
            .param("position", "5")
            .to_http_request();

        let mut payload = Payload::None;
        let res = StageId::from_request(&req, &mut payload).await;

        let expected = StageId::new_substage(ProjectId::new(uuid), 2, 5);
        assert_eq!(res.unwrap(), expected);
    }

    #[actix_web::test]
    async fn extracts_stage_id_from_path_with_trailing_uuid() {
        let uuid = Uuid::new_v4();
        let item_id = Uuid::new_v4();
        let req = TestRequest::default()
            .param("project_id", uuid.to_string())
            .param("position", "2")
            .param("item_id", item_id.to_string())
            .to_http_request();

        let mut payload = Payload::None;
        let res = StageId::from_request(&req, &mut payload).await;

        let expected = StageId::new(ProjectId::new(uuid), 2);
        assert_eq!(res.unwrap(), expected);
    }

    #[actix_web::test]
    async fn extracts_substage_id_from_path_with_trailing_uuid() {
        let uuid = Uuid::new_v4();
        let item_id = Uuid::new_v4();
        let req = TestRequest::default()
            .param("project_id", uuid.to_string())
            .param("parent_position", "2")
            .param("position", "5")
            .param("item_id", item_id.to_string())
            .to_http_request();

        let mut payload = Payload::None;
        let res = StageId::from_request(&req, &mut payload).await;

        let expected = StageId::new_substage(ProjectId::new(uuid), 2, 5);
        assert_eq!(res.unwrap(), expected);
    }

    #[actix_web::test]
    async fn returns_bad_request_on_invalid_stage_path() {
        let req = TestRequest::default()
            .param("invalid", "path")
            .to_http_request();

        let mut payload = Payload::None;
        let res = StageId::from_request(&req, &mut payload).await;

        assert!(matches!(res, Err(ApiError::BadRequest(_))));
    }
}
