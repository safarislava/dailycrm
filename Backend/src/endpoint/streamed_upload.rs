use crate::endpoint::api_error::ApiError;
use crate::model::project::streamed_file::StreamedFile;
use actix_multipart::Field;
use bytes::BytesMut;
use futures_util::StreamExt;
use std::sync::Arc;

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
    let mut data = BytesMut::new();
    while let Some(chunk_res) = field.next().await {
        let chunk =
            chunk_res.map_err(|_| ApiError::BadRequest("Invalid multipart data".to_string()))?;
        if data.len() + chunk.len() > limit {
            return Err(ApiError::PayloadTooLarge);
        }
        data.extend_from_slice(&chunk);
    }
    if data.is_empty() {
        return Err(ApiError::BadRequest("File is empty".to_string()));
    }
    let bytes = data.freeze();
    let mime_type = infer::get(&bytes)
        .map(|kind| kind.mime_type().to_string())
        .unwrap_or_else(|| "application/octet-stream".to_string());
    Ok(Arc::new(StreamedFile::new(filename, mime_type, bytes)))
}
