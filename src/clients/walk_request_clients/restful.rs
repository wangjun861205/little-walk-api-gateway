use bytes::Bytes;
use futures::{stream, Stream, StreamExt, TryStreamExt};
use http::{request, StatusCode};
use url::Url;

use crate::{
    core::{clients::walk_request, error::Error},
    utils::restful::{parse_url, request, to_query_string},
};

#[derive(Debug, Clone)]
pub(crate) struct WalkRequestClient {
    host_and_port: String,
}

impl WalkRequestClient {
    pub(crate) fn new(host_and_port: impl Into<String>) -> Self {
        Self {
            host_and_port: host_and_port.into(),
        }
    }
}

impl walk_request::WalkRequestClient for WalkRequestClient {
    async fn query_walk_requests(
        &self,
        query: walk_request::WalkRequestQuery,
    ) -> Result<Vec<walk_request::WalkRequest>, Error> {
        let url = parse_url(
            &self.host_and_port,
            "/apis/walk_requests/nearby",
            to_query_string(&query)?.as_deref(),
        )?;
        let bytes: Vec<u8> = request(reqwest::Client::new().get(url))
            .await?
            .try_collect::<Vec<Bytes>>()
            .await?
            .into_iter()
            .flat_map(|b| b.to_vec())
            .collect();
        let reqs: Vec<walk_request::WalkRequest> =
            serde_json::from_slice(&bytes).map_err(|e| {
                Error::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e)
            })?;
        Ok(reqs)
    }
}
