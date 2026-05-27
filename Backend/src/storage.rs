use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;

const BUCKET: &str = "crm-attachments";

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Clone)]
pub struct Storage {
    client: Client,
}

impl Storage {
    pub fn new(client: Client) -> Self {
        Self { client }
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

    pub async fn get_bytes(&self, key: &str) -> Result<Vec<u8>, BoxError> {
        let output = self
            .client
            .get_object()
            .bucket(BUCKET)
            .key(key)
            .send()
            .await?;
        let bytes = output.body.collect().await?.into_bytes();
        Ok(bytes.to_vec())
    }
}
