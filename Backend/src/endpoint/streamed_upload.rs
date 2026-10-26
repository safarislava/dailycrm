use crate::endpoint::api_error::ApiError;
use crate::model::project::streamed_file::StreamedFile;
use actix_multipart::Field;
use futures_util::StreamExt;
use std::sync::Arc;
use std::sync::atomic::AtomicI64;

pub async fn create_streamed_file(
    mut field: Field,
    fallback_filename: &str,
    limit: usize,
) -> Result<Arc<StreamedFile>, ApiError> {
    let filename = field
        .content_disposition()
        .and_then(|cd| cd.get_filename())
        .map(|s| s.to_string())
        .unwrap_or_else(|| fallback_filename.to_string());
    let first_chunk = match field.next().await {
        Some(Ok(bytes)) if !bytes.is_empty() => bytes,
        Some(Ok(_)) => return Err(ApiError::BadRequest("File is empty".to_string())),
        Some(Err(_)) => return Err(ApiError::BadRequest("Invalid multipart data".to_string())),
        None => return Err(ApiError::BadRequest("No file provided".to_string())),
    };
    if first_chunk.len() > limit {
        return Err(ApiError::PayloadTooLarge);
    }
    let mime_type = infer::get(&first_chunk)
        .map(|kind| kind.mime_type().to_string())
        .unwrap_or_else(|| "application/octet-stream".to_string());

    let (tx, rx) = tokio::sync::mpsc::channel(2);
    let size_bytes = Arc::new(AtomicI64::new(0));

    if tx.send(Ok(first_chunk)).await.is_err() {
        return Err(ApiError::Internal("Upload channel closed".to_string()));
    }

    actix_web::rt::spawn(async move {
        while let Some(chunk_res) = field.next().await {
            match chunk_res {
                Ok(chunk) => {
                    if tx.send(Ok(chunk)).await.is_err() {
                        break;
                    }
                }
                Err(err) => {
                    let _ = tx.send(Err(std::io::Error::other(err.to_string()))).await;
                    break;
                }
            }
        }
    });

    Ok(Arc::new(StreamedFile::new(
        filename, mime_type, rx, limit, size_bytes,
    )))
}
