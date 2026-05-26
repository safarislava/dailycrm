use crate::state::AppState;
use actix_multipart::Multipart;
use actix_web::{HttpResponse, Responder, web};
use futures_util::StreamExt;
use uuid::Uuid;

const MAX_FILE_SIZE: usize = 50 * 1_048_576; // 50 MB

enum CollectError {
    TooLarge,
    Read,
}

async fn collect_bytes(
    field: &mut actix_multipart::Field,
    limit: usize,
) -> Result<Vec<u8>, CollectError> {
    let mut data = Vec::new();
    while let Some(chunk) = field.next().await {
        match chunk {
            Ok(bytes) => {
                data.extend_from_slice(&bytes);
                if data.len() > limit {
                    return Err(CollectError::TooLarge);
                }
            }
            Err(_) => return Err(CollectError::Read),
        }
    }
    Ok(data)
}

pub async fn post(
    state: web::Data<AppState>,
    path: web::Path<(Uuid, i32)>,
    mut payload: Multipart,
) -> impl Responder {
    let (project_id, stage_position) = path.into_inner();
    let attachments = state
        .projects
        .project_link(project_id)
        .stages()
        .stage_link(stage_position)
        .attachments();

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(_) => return HttpResponse::BadRequest().body("Invalid multipart data"),
        };

        let filename = field
            .content_disposition()
            .and_then(|cd| cd.get_filename())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "file".to_string());

        let data = match collect_bytes(&mut field, MAX_FILE_SIZE).await {
            Ok(bytes) => bytes,
            Err(CollectError::TooLarge) => return HttpResponse::PayloadTooLarge().finish(),
            Err(CollectError::Read) => {
                return HttpResponse::InternalServerError().body("Upload error");
            }
        };

        let mime_type = infer::get(&data)
            .map(|kind| kind.mime_type().to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        return match attachments.upload(filename, mime_type, data).await {
            Ok(id) => HttpResponse::Created().json(serde_json::json!({ "id": id })),
            Err(_) => HttpResponse::InternalServerError().body("Something went wrong"),
        };
    }

    HttpResponse::BadRequest().body("No file provided")
}