use super::service::ByteStream;
use crate::core::error::Error;

pub trait WalkRequestClient {
    async fn nearby_requests(
        &self,
        lat: f64,
        long: f64,
        page: i32,
        size: i32,
    ) -> Result<ByteStream, Error>;
}
