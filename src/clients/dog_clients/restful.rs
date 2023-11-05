use crate::{
    core::{
        dog_client::DogClient as IDogClient, error::Error, service::ByteStream,
    },
    utils::restful::request,
};
use reqwest::{
    header::{HeaderName, HeaderValue},
    Body, Client, Method, Request, RequestBuilder,
};
use std::str::FromStr;
use url::Url;

pub struct DogClient {
    base_url: Url,
}

impl DogClient {
    pub fn new(base_url: Url) -> Self {
        Self { base_url }
    }
}

impl IDogClient for DogClient {
    async fn add_dog(
        &self,
        owner_id: &str,
        body: ByteStream,
    ) -> Result<ByteStream, Error> {
        let url = self.base_url.join("/dogs")?;
        let (stream, _) = request(
            Client::new()
                .post(url)
                .header("X-User-ID", owner_id)
                .header("Content-Type", "application/json")
                .body(Body::wrap_stream(body)),
        )
        .await?;
        Ok(stream)
    }

    async fn dogs_by_owner_id(
        &self,
        owner_id: &str,
    ) -> Result<ByteStream, Error> {
        let url = self
            .base_url
            .join(format!("/users/{}/dogs", owner_id).as_str())?;
        let (stream, _) = request(Client::new().get(url)).await?;
        Ok(stream)
    }
}
