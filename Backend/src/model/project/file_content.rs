use crate::common::BoxError;
use crate::model::project::contract::file::File;
use crate::storage::Storage;

#[derive(Clone)]
pub struct FileContent {
    filename: String,
    mime_type: String,
    data: Vec<u8>,
}

impl FileContent {
    pub fn new(filename: String, mime_type: String, data: Vec<u8>) -> Self {
        Self {
            filename,
            mime_type,
            data,
        }
    }
}

#[async_trait::async_trait]
impl File for FileContent {
    fn name(&self) -> &str {
        &self.filename
    }

    fn media_type(&self) -> &str {
        &self.mime_type
    }

    fn size_bytes(&self) -> i64 {
        self.data.len() as i64
    }

    async fn upload_to(&self, storage: &Storage, key: &str) -> Result<(), BoxError> {
        storage
            .upload(key, self.data.clone(), &self.mime_type, &self.filename)
            .await
    }
}