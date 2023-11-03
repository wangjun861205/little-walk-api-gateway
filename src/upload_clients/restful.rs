use crate::core::error::Error;
use crate::core::service::ByteStream;
use crate::core::upload_client::UploadClient as IUploadClient;
use crate::utils::restful::make_request;
use bytes::Bytes;
use futures::Stream;
use reqwest::{
    multipart::{Form, Part},
    Body, Method,
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
        filename: &str,
        stream: ByteStream,
    ) -> Result<ByteStream, Error> {
        let url = self
            .base_url
            .join("/files")
            .map_err(|e| Error::InvalidURL(e.to_string()))?;
        let body = Body::wrap_stream(stream);
        let form = Form::new()
            .part("file", Part::stream(body).file_name(filename.to_owned()));
        make_request(
            Method::POST,
            url,
            Option::<String>::None,
            Option::<String>::None,
            Some(form),
        )
        .await
    }
    async fn download(&self, id: &str) -> Result<ByteStream, Error> {
        let url = self
            .base_url
            .join(format!("/files/{}", id).as_str())
            .map_err(|e| Error::InvalidURL(e.to_string()))?;
        make_request(
            Method::GET,
            url,
            Option::<String>::None,
            Option::<String>::None,
            None,
        )
        .await
    }
}
