use actix_web::web::Bytes;
use anyhow::Error;
use futures::Stream;
use std::pin::Pin;

pub trait UploadClient {
    async fn upload(&self, filename: &str, stream: impl Stream<Item = Result<Bytes, Error>>) -> Result<String, Error>;

    async fn download(&self, id: String) -> Result<Pin<Box<dyn Stream<Item = Result<Bytes, Error>>>>, Error>;
}
