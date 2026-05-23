use aws_sdk_s3::Client;
use aws_sdk_s3::config::{BehaviorVersion, Builder, Credentials, Region};
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::primitives::ByteStream;
use std::env;
use std::time::Duration;

const BUCKET: &str = "crm-attachments";

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Clone)]
pub struct Storage {
    client: Client,
    endpoint: String,
    public_url: String,
}

impl Storage {
    pub fn new() -> Self {
        let endpoint = env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINT must be set");
        let public_url = env::var("MINIO_PUBLIC_URL").unwrap_or_else(|_| endpoint.clone());
        let access_key = env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEY must be set");
        let secret_key = env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEY must be set");

        let creds = Credentials::new(&access_key, &secret_key, None, None, "minio");
        let config = Builder::new()
            .behavior_version(BehaviorVersion::latest())
            .endpoint_url(&endpoint)
            .credentials_provider(creds)
            .region(Region::new("us-east-1"))
            .force_path_style(true)
            .build();

        Self { client: Client::from_conf(config), endpoint, public_url }
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

    pub async fn presigned_url(&self, key: &str) -> Result<String, BoxError> {
        let config = PresigningConfig::expires_in(Duration::from_secs(3600))?;
        let request = self
            .client
            .get_object()
            .bucket(BUCKET)
            .key(key)
            .presigned(config)
            .await?;
        // Replace internal Docker endpoint with the publicly accessible URL
        let url = request.uri().to_string().replacen(&self.endpoint, &self.public_url, 1);
        Ok(url)
    }
}