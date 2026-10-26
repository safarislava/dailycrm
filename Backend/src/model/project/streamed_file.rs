use crate::endpoint::api_error::ApiError;
use crate::model::contract::box_error::BoxError;
use crate::model::project::contract::file::File;
use crate::storage::Storage;
use bytes::Bytes;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};
use tokio::sync::Mutex;
use tokio::sync::mpsc::Receiver;

pub struct StreamedFile {
    filename: String,
    mime_type: String,
    receiver: Mutex<Option<Receiver<Result<Bytes, std::io::Error>>>>,
    limit: usize,
    size_bytes: Arc<AtomicI64>,
}

impl StreamedFile {
    pub fn new(
        filename: String,
        mime_type: String,
        receiver: Receiver<Result<Bytes, std::io::Error>>,
        limit: usize,
        size_bytes: Arc<AtomicI64>,
    ) -> Self {
        Self {
            filename,
            mime_type,
            receiver: Mutex::new(Some(receiver)),
            limit,
            size_bytes,
        }
    }
}

#[async_trait::async_trait]
impl File for StreamedFile {
    fn name(&self) -> &str {
        &self.filename
    }

    fn media_type(&self) -> &str {
        &self.mime_type
    }

    fn size_bytes(&self) -> i64 {
        self.size_bytes.load(Ordering::SeqCst)
    }

    async fn upload_to(&self, storage: &Storage, key: &str) -> Result<(), BoxError> {
        let mut rx = self
            .receiver
            .lock()
            .await
            .take()
            .ok_or_else(|| "Stream already consumed".to_string())?;

        let mut writer = storage.writer(key, &self.mime_type).await?;
        let mut total: i64 = 0;

        while let Some(chunk_res) = rx.recv().await {
            let chunk = chunk_res?;
            total += chunk.len() as i64;
            if total > self.limit as i64 {
                let _ = storage.delete(key).await;
                return Err(Box::new(ApiError::PayloadTooLarge));
            }
            writer.write(chunk).await?;
        }

        writer.close().await?;
        self.size_bytes.store(total, Ordering::SeqCst);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provides_file_attributes() {
        let size_bytes = Arc::new(AtomicI64::new(11));
        let file = StreamedFile {
            filename: "document.pdf".to_string(),
            mime_type: "application/pdf".to_string(),
            receiver: Mutex::new(None),
            limit: 1024,
            size_bytes,
        };

        assert_eq!(file.name(), "document.pdf");
        assert_eq!(file.media_type(), "application/pdf");
        assert_eq!(file.size_bytes(), 11);
    }
}
