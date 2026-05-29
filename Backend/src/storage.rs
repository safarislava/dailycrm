use aws_sdk_s3::Client;
use aws_sdk_s3::config::{BehaviorVersion, Builder, Credentials, Region};
use aws_sdk_s3::primitives::ByteStream;
use bytes::Bytes;
use futures_util::Stream;
use std::env;
use std::pin::Pin;
use tokio_util::io::ReaderStream;

const BUCKET: &str = "crm-attachments";

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub type FileStream = Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>;

#[derive(Clone)]
pub struct Storage {
    client: Client,
}

impl Storage {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn from_env() -> Self {
        let endpoint = env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINT must be set");
        let access_key = env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEY must be set");
        let secret_key = env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEY must be set");
        let credentials = Credentials::new(&access_key, &secret_key, None, None, "minio");
        let client = Client::from_conf(
            Builder::new()
                .behavior_version(BehaviorVersion::latest())
                .endpoint_url(&endpoint)
                .credentials_provider(credentials)
                .region(Region::new("us-east-1"))
                .force_path_style(true)
                .build(),
        );
        let storage = Self::new(client);
        storage.ensure_bucket().await;
        storage
    }

    pub async fn ensure_bucket(&self) {
        let _ = self.client.create_bucket().bucket(BUCKET).send().await;
    }

    pub async fn upload(
        &self,
        key: &str,
        data: Vec<u8>,
        content_type: &str,
        filename: &str,
    ) -> Result<(), BoxError> {
        let disposition = format!("attachment; filename=\"{}\"", filename.replace('"', "\\\""));
        self.client
            .put_object()
            .bucket(BUCKET)
            .key(key)
            .body(ByteStream::from(data))
            .content_type(content_type)
            .content_disposition(disposition)
            .send()
            .await?;
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> Result<(), BoxError> {
        self.client
            .delete_object()
            .bucket(BUCKET)
            .key(key)
            .send()
            .await?;
        Ok(())
    }

    pub async fn get_stream(&self, key: &str) -> Result<FileStream, BoxError> {
        let output = self
            .client
            .get_object()
            .bucket(BUCKET)
            .key(key)
            .send()
            .await?;
        Ok(Box::pin(ReaderStream::new(output.body.into_async_read())))
    }
}
