use bytes::Bytes;
use futures_util::Stream;
use futures_util::StreamExt;
use opendal::Operator;
use opendal::services::S3;
use std::env;
use std::pin::Pin;

const BUCKET: &str = "crm-attachments";

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub type FileStream = Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>;

#[derive(Clone)]
pub struct Storage {
    op: Operator,
}

impl Storage {
    pub fn new(op: Operator) -> Self {
        Self { op }
    }

    pub async fn from_env() -> Self {
        let endpoint = env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINT must be set");
        let access_key = env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEY must be set");
        let secret_key = env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEY must be set");

        let builder = S3::default()
            .endpoint(&endpoint)
            .bucket(BUCKET)
            .access_key_id(&access_key)
            .secret_access_key(&secret_key)
            .region("us-east-1");

        let op = Operator::new(builder).expect("Failed to initialize S3 builder");

        let storage = Self::new(op);
        storage.ensure_bucket().await;
        storage
    }

    pub async fn ensure_bucket(&self) {
        let _ = self.op.create_dir("").await;
    }

    pub async fn check_health(&self) -> Result<(), BoxError> {
        self.op.check().await?;
        Ok(())
    }

    pub async fn writer(&self, key: &str, content_type: &str) -> Result<opendal::Writer, BoxError> {
        let writer = self.op.writer_with(key).content_type(content_type).await?;
        Ok(writer)
    }

    pub async fn delete(&self, key: &str) -> Result<(), BoxError> {
        self.op.delete(key).await?;
        Ok(())
    }

    pub async fn stream(&self, key: &str) -> Result<FileStream, BoxError> {
        let stream = self.op.reader(key).await?.into_bytes_stream(..).await?;
        let mapped = stream.map(|res| res.map_err(|e| std::io::Error::other(e.to_string())));
        Ok(Box::pin(mapped))
    }
}
