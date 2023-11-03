use crate::core::error::Error;

use super::service::ByteStream;

pub trait UploadClient {
    async fn upload(
        &self,
        filename: &str,
        stream: ByteStream,
    ) -> Result<ByteStream, Error>;

    async fn download(&self, id: &str) -> Result<ByteStream, Error>;
}
