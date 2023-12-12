use crate::core::error::Error;

use crate::core::service::ByteStream;

pub trait UploadClient: Clone + 'static {
    async fn upload(
        &self,
        content_type_header: &str,
        user_id: &str,
        limit: usize,
        payload: ByteStream,
    ) -> Result<ByteStream, Error>;

    async fn download(&self, id: &str) -> Result<ByteStream, Error>;
}
