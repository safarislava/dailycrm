use crate::model::contract::box_error::BoxError;
use crate::model::project::contract::file::File;
use crate::storage::Storage;
use aws_sdk_s3::primitives::ByteStream;
use bytes::Bytes;

pub struct StreamedFile {
    filename: String,
    mime_type: String,
    bytes: Bytes,
}

impl StreamedFile {
    pub fn new(filename: String, mime_type: String, bytes: Bytes) -> Self {
        Self {
            filename,
            mime_type,
            bytes,
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
        self.bytes.len() as i64
    }

    async fn upload_to(&self, storage: &Storage, key: &str) -> Result<(), BoxError> {
        let size = self.size_bytes();
        storage
            .upload_stream(
                key,
                ByteStream::from(self.bytes.clone()),
                &self.mime_type,
                &self.filename,
                Some(size),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provides_file_attributes() {
        let bytes = Bytes::from_static(b"hello world");
        let file = StreamedFile::new(
            "document.pdf".to_string(),
            "application/pdf".to_string(),
            bytes,
        );

        assert_eq!(file.name(), "document.pdf");
        assert_eq!(file.media_type(), "application/pdf");
        assert_eq!(file.size_bytes(), 11);
    }
}
