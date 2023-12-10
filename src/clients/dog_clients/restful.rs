use crate::{
    core::{
        clients::dog::{Dog, DogClient as IDogClient},
        error::Error,
        requests::{DogPortraitUpdate, DogQuery},
        service::ByteStream,
    },
    handlers::dog::BreedQuery,
    utils::{
        io::stream_to_bytes,
        restful::{parse_url, request, to_query_string},
    },
};
use http::StatusCode;
use reqwest::{Body, Client, Method};
use serde_json::from_slice;

use super::responses::IsOwnerOfTheDogResp;
use bytes::Bytes;
use futures::TryStreamExt;

pub struct DogClient {
    host_and_port: String,
}

impl DogClient {
    pub fn new(host_and_port: &str) -> Self {
        Self {
            host_and_port: host_and_port.to_string(),
        }
    }
}

impl IDogClient for DogClient {
    async fn add_dog(
        &self,
        owner_id: &str,
        body: ByteStream,
    ) -> Result<ByteStream, Error> {
        let url = parse_url(&self.host_and_port, "/dogs", None)?;
        let stream = request(
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
        let url = parse_url(
            &self.host_and_port,
            format!("/users/{}/dogs", owner_id).as_str(),
            None,
        )?;
        let stream = request(Client::new().get(url)).await?;
        Ok(stream)
    }

    async fn query_dogs(&self, query: &DogQuery) -> Result<Vec<Dog>, Error> {
        let url = parse_url(
            &self.host_and_port,
            "/apis/dogs",
            to_query_string(query)?.as_deref(),
        )?;
        let builder = Client::new().request(Method::GET, url);
        let bs: Vec<u8> = request(builder)
            .await?
            .try_collect::<Vec<Bytes>>()
            .await?
            .into_iter()
            .flat_map(|b| b.to_vec())
            .collect();
        from_slice(&bs).map_err(|e| {
            Error::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e)
        })
    }

    async fn is_owner_of_the_dog(
        &self,
        owner_id: &str,
        dog_id: &str,
    ) -> Result<bool, Error> {
        let mut url = parse_url(
            &self.host_and_port,
            "/dogs/exists",
            to_query_string(&DogQuery {
                id: Some(dog_id.to_owned()),
                owner_id: Some(owner_id.to_owned()),
                ..Default::default()
            })?
            .as_deref(),
        )?;
        let params =
            vec![format!("owner_id={}", owner_id), format!("id={}", dog_id)];
        url.set_query(Some(&params.join("&")));
        let builder = Client::new().request(Method::GET, url);
        let stream = request(builder).await?;
        let bytes = stream_to_bytes(stream).await?;
        let result: IsOwnerOfTheDogResp = serde_json::from_slice(&bytes)
            .map_err(|e| {
                Error::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e)
            })?;
        Ok(result.is_owner)
    }

    async fn update_dog_portrait(
        &self,
        dog_id: &str,
        portrait_id: &str,
    ) -> Result<ByteStream, Error> {
        let url = parse_url(
            &self.host_and_port,
            &format!("/dogs/{}/portrait", dog_id),
            None,
        )?;
        let builder = Client::new()
            .request(Method::PUT, url)
            .body(
                serde_json::to_string(&DogPortraitUpdate {
                    portrait_id: portrait_id.into(),
                })
                .map_err(|e| {
                    Error::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e)
                })?,
            )
            .header("Content-Type", "application/json");
        request(builder).await
    }

    async fn query_breeds(&self, category: &str) -> Result<ByteStream, Error> {
        let url = parse_url(
            &self.host_and_port,
            "/breeds",
            to_query_string(&BreedQuery {
                category_eq: category.to_owned(),
            })?
            .as_deref(),
        )?;
        let builder = Client::new().request(Method::GET, url);
        request(builder).await
    }

    async fn update_dog(
        &self,
        dog_id: &str,
        body: Bytes,
    ) -> Result<ByteStream, Error> {
        let url =
            parse_url(&self.host_and_port, &format!("/dogs/{}", dog_id), None)?;
        let builder = Client::new()
            .request(Method::PUT, url)
            .body(Body::from(body))
            .header("Content-Type", "application/json");
        request(builder).await
    }
}
