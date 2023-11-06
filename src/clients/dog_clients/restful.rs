use crate::{
    core::{
        dog_client::DogClient as IDogClient, error::Error, requests::DogQuery,
        service::ByteStream,
    },
    utils::restful::request,
};
use reqwest::{Body, Client, Method, StatusCode};
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

    async fn query_dogs(
        &self,
        query: &DogQuery,
        page: i32,
        size: i32,
    ) -> Result<(ByteStream, StatusCode), Error> {
        let mut url = self.base_url.join("/dogs")?;
        let mut params =
            vec![format!("page={}", page), format!("size={}", size)];
        if let Some(owner_id_eq) = &query.owner_id_eq {
            params.push(format!("owner_id_eq={}", owner_id_eq));
        }
        url.set_query(Some(&params.join("&")));
        let builder = Client::new().request(Method::GET, url);
        request(builder).await
    }
}
