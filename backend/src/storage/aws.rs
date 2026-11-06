use std::time::Duration;

use aws_config::BehaviorVersion;
use aws_sdk_s3::{
    config::{Credentials, RequestChecksumCalculation},
    presigning::PresigningConfig,
    Client as S3Client,
};
use hyper::Method;

use super::FoodieStorage;
use uuid::Uuid;

#[derive(Clone)]
pub struct FoodieAws {
    client: S3Client,
}

impl FoodieAws {
    pub async fn new() -> Self {
        let aws_url = dotenv::var("AWS_URL").expect("AWS_URL needed");
        let access_key_id = dotenv::var("AWS_USER").expect("AWS_USER needed");
        let secret_access_key = dotenv::var("AWS_PASSWORD").expect("AWS_PASSWORD needed");

        let aws = aws_config::defaults(BehaviorVersion::latest())
            .endpoint_url(aws_url)
            .credentials_provider(Credentials::new(
                access_key_id,
                secret_access_key,
                None,
                None,
                "static",
            ))
            .region("us-east-1");
        let conf = aws.load().await;
        // The browser supplies the upload body after this URL is signed. Do not
        // sign the SDK's default checksum for an empty body into that URL.
        let config_builder = aws_sdk_s3::config::Builder::from(&conf)
            .force_path_style(true)
            .request_checksum_calculation(RequestChecksumCalculation::WhenRequired);
        let client = S3Client::from_conf(config_builder.build());

        Self { client }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn presigned_upload_does_not_require_a_checksum_for_an_empty_body() {
        let storage = FoodieAws::new().await;
        let file = Uuid::new_v4();
        let url = storage.get_presigned_url(file, Method::PUT).await.unwrap();
        let url = reqwest::Url::parse(&url).unwrap();

        assert_eq!(url.path(), format!("/aws/{file}"));
        assert!(url.query_pairs().any(|(key, value)| {
            key.eq_ignore_ascii_case("x-amz-signedheaders") && value == "host"
        }));
        assert!(!url.query_pairs().any(|(key, _)| {
            key.starts_with("x-amz-checksum-") || key == "x-amz-sdk-checksum-algorithm"
        }));
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
                .bucket("aws")
                .key(file)
                .presigned(presigned)
                .await?
                .uri()
                .to_string(),
            Method::POST | Method::PUT => self
                .client
                .put_object()
                .bucket("aws")
                .key(file)
                .presigned(presigned)
                .await
                .unwrap()
                .uri()
                .to_string(),
            Method::DELETE => self
                .client
                .delete_object()
                .bucket("aws")
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
            .bucket("aws")
            .key(file)
            .send()
            .await?;
        Ok(())
    }
}
