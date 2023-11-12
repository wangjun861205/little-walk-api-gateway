use crate::{
    core::{
        dog_client::DogClient as IDogClient,
        error::Error,
        requests::{DogPortraitUpdate, DogQuery},
        service::ByteStream,
    },
    utils::{io::stream_to_bytes, restful::request},
};
use reqwest::{Body, Client, Method, StatusCode};
use url::Url;

use super::responses::IsOwnerOfTheDogResp;

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

    async fn is_owner_of_the_dog(
        &self,
        owner_id: &str,
        dog_id: &str,
    ) -> Result<bool, Error> {
        let mut url = self.base_url.join("/dogs/exists")?;
        let params =
            vec![format!("owner_id={}", owner_id), format!("id={}", dog_id)];
        url.set_query(Some(&params.join("&")));
        let builder = Client::new().request(Method::GET, url);
        let (stream, _) = request(builder).await?;
        let bytes = stream_to_bytes(stream).await?;
        let result: IsOwnerOfTheDogResp = serde_json::from_slice(&bytes)?;
        Ok(result.is_owner)
    }

    async fn update_dog_portrait(
        &self,
        dog_id: &str,
        portrait_id: &str,
    ) -> Result<(ByteStream, StatusCode), Error> {
        let url = self.base_url.join(&format!("/dogs/{}/portrait", dog_id))?;
        let builder = Client::new()
            .request(Method::PUT, url)
            .body(serde_json::to_string(&DogPortraitUpdate {
                portrait_id: portrait_id.into(),
            })?)
            .header("Content-Type", "application/json");
        request(builder).await
    }

    async fn query_breeds(&self, category: &str) -> Result<ByteStream, Error> {
        let url = self
            .base_url
            .join(&format!("/breeds?category_eq={}", category))?;
        let builder = Client::new().request(Method::GET, url);
        let (stream, status) = request(builder).await?;
        if status != StatusCode::OK {
            let bs = stream_to_bytes(stream).await?;
            let err_str = String::from_utf8(bs.to_vec())
                .map_err(|e| Error::InvalidResponse(e.to_string()))?;
            return Err(Error::DogServiceError(err_str));
        }
        Ok(stream)
    }
}
