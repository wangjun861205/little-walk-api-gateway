use crate::core::service::ByteStream;
use crate::core::upload_client::UploadClient as IUploadClient;
use crate::utils::restful::make_request;
use crate::{core::error::Error, utils::restful::request};
use reqwest::{
    multipart::{Form, Part},
    Body, Client, Method,
};
use url::Url;

pub struct UploadClient {
    base_url: Url,
}

impl UploadClient {
    pub fn new(base_url: Url) -> Self {
        Self { base_url }
    }
}

impl IUploadClient for UploadClient {
    async fn upload(
        &self,
        content_type_header: &str,
        user_id: &str,
        size_limit: usize,
        stream: ByteStream,
    ) -> Result<ByteStream, Error> {
        let url = self
            .base_url
            .join("/files")
            .map_err(|e| Error::InvalidURL(e.to_string()))?;
        let builder = Client::new()
            .request(Method::POST, url)
            .header("X-User-ID", user_id)
            .header("Content-Type", content_type_header)
            .header("X-Size-Limit", size_limit.to_string())
            .body(Body::wrap_stream(stream));
        let (stream, _) = request(builder).await?;
        Ok(stream)
    }
    async fn download(&self, id: &str) -> Result<ByteStream, Error> {
        let url = self
            .base_url
            .join(format!("/files/{}", id).as_str())
            .map_err(|e| Error::InvalidURL(e.to_string()))?;
        make_request(
            Method::GET,
            url,
            None,
            Option::<String>::None,
            Option::<String>::None,
            None,
        )
        .await
    }
}
