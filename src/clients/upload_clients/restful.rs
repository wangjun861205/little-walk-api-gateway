use crate::core::clients::upload::UploadClient as IUploadClient;
use crate::core::service::ByteStream;
use crate::utils::restful::RequestBody;
use crate::utils::restful::{make_request, parse_url};
use crate::{core::error::Error, utils::restful::request};
use reqwest::{Body, Client, Method};

#[derive(Clone)]
pub struct UploadClient {
    host_and_port: String,
}

impl UploadClient {
    pub fn new(host_and_port: &str) -> Self {
        Self {
            host_and_port: host_and_port.to_string(),
        }
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
        let url = parse_url(&self.host_and_port, "/files", None)?;
        let builder = Client::new()
            .request(Method::POST, url)
            .header("X-User-ID", user_id)
            .header("Content-Type", content_type_header)
            .header("X-Size-Limit", size_limit.to_string())
            .body(Body::wrap_stream(stream));
        request(builder).await
    }
    async fn download(&self, id: &str) -> Result<ByteStream, Error> {
        make_request(
            Method::GET,
            &self.host_and_port,
            format!("/files/{}", id).as_str(),
            None,
            Option::<()>::None,
            RequestBody::<()>::None,
        )
        .await
    }
}
