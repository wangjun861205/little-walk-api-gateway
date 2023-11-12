use std::collections::HashMap;

use crate::{
    core::{
        dog_client::DogClient as IDogClient,
        error::Error,
        requests::{DogPortraitUpdate, DogQuery},
        service::ByteStream,
    },
    utils::{
        io::stream_to_bytes,
        restful::{parse_url, request},
    },
};
use reqwest::{Body, Client, Method, StatusCode};

use super::responses::IsOwnerOfTheDogResp;
use bytes::Bytes;

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

    async fn query_dogs(
        &self,
        query: &DogQuery,
        page: i32,
        size: i32,
    ) -> Result<ByteStream, Error> {
        let page = page.to_string();
        let size = size.to_string();
        let mut q = HashMap::new();
        q.insert("page", page.as_str());
        q.insert("size", size.as_str());
        if let Some(owner_id_eq) = &query.owner_id_eq {
            q.insert("owner_id_eq", owner_id_eq);
        }
        let url = parse_url(&self.host_and_port, "/dogs", Some(q))?;
        let builder = Client::new().request(Method::GET, url);
        request(builder).await
    }

    async fn is_owner_of_the_dog(
        &self,
        owner_id: &str,
        dog_id: &str,
    ) -> Result<bool, Error> {
        let mut url = parse_url(
            &self.host_and_port,
            "/dogs/exists",
            Some(
                vec![("owner_id", owner_id), ("id", dog_id)]
                    .into_iter()
                    .collect::<HashMap<&str, &str>>(),
            ),
        )?;
        let params =
            vec![format!("owner_id={}", owner_id), format!("id={}", dog_id)];
        url.set_query(Some(&params.join("&")));
        let builder = Client::new().request(Method::GET, url);
        let stream = request(builder).await?;
        let bytes = stream_to_bytes(stream).await?;
        let result: IsOwnerOfTheDogResp = serde_json::from_slice(&bytes)
            .map_err(|e| Error::new(StatusCode::INTERNAL_SERVER_ERROR, e))?;
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
                    Error::new(StatusCode::INTERNAL_SERVER_ERROR, e)
                })?,
            )
            .header("Content-Type", "application/json");
        request(builder).await
    }

    async fn query_breeds(&self, category: &str) -> Result<ByteStream, Error> {
        let url = parse_url(
            &self.host_and_port,
            "/breeds",
            Some(vec![("category_eq", category)].into_iter().collect()),
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
