use std::time::Duration;

use aws_config::BehaviorVersion;
use aws_sdk_s3::{config::Credentials, presigning::PresigningConfig, Client as S3Client};
use hyper::Method;

use super::FoodieStorage;
use uuid::Uuid;

#[derive(Clone)]
pub struct FoodieAws {
    client: S3Client,
}

// TODO: Do not store auth here of course
impl FoodieAws {
    pub async fn new() -> Self {
        let aws_url = dotenv::var("AWS_URL").expect("AWS_URL needed");
        let aws = aws_config::defaults(BehaviorVersion::latest())
            .endpoint_url(aws_url)
            .credentials_provider(Credentials::new(
                "admin",
                "admin",
                Some("test".to_string()),
                None,
                "testing",
            ))
            .region("sa-east-1");
        let conf = aws.load().await;
        let config_builder = aws_sdk_s3::config::Builder::from(&conf).force_path_style(true);
        let client = S3Client::from_conf(config_builder.build());

        Self { client }
    }
}

#[async_trait::async_trait]
impl FoodieStorage for FoodieAws {
    async fn get_presigned_url(&self, file: Uuid, method: Method) -> Result<String, anyhow::Error> {
        let presigned = PresigningConfig::expires_in(Duration::from_secs(60 * 10))?;
        let res = match method {
            Method::GET => self
                .client
                .get_object()
                .bucket("images")
                .key(file)
                .presigned(presigned)
                .await?
                .uri()
                .to_string(),
            Method::POST | Method::PUT => self
                .client
                .put_object()
                .bucket("images")
                .content_type("image/png")
                .key(file)
                .presigned(presigned)
                .await
                .unwrap()
                .uri()
                .to_string(),
            Method::DELETE => self
                .client
                .delete_object()
                .bucket("images")
                .key(file)
                .presigned(presigned)
                .await?
                .uri()
                .to_string(),
            _ => unimplemented!("Not supported for now"),
        };

        Ok(res)
    }

    async fn delete(&self, file: Uuid) -> Result<(), anyhow::Error> {
        self.client
            .delete_object()
            .bucket("images")
            .key(file)
            .send()
            .await?;
        Ok(())
    }
}
