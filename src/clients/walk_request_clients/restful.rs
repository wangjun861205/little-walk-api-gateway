use bytes::Bytes;
use futures::{stream, Stream, StreamExt, TryStreamExt};
use http::{request, StatusCode};
use nb_to_query::ToQuery;
use url::Url;

use crate::{
    core::{
        error::Error,
        walk_request_client::{UpstreamWalkRequest, WalkRequest},
        walk_request_client::{
            WalkRequestClient as IWalkRequestClient, WalkRequestQuery,
        },
    },
    utils::restful::{parse_url, request},
};

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

impl IWalkRequestClient for WalkRequestClient {
    async fn query_walk_requests(
        &self,
        query: WalkRequestQuery,
    ) -> Result<Vec<UpstreamWalkRequest>, Error> {
        let url = parse_url(
            &self.host_and_port,
            "/apis/walk_requests/nearby",
            query.to_query("").as_deref(),
        )?;
        let bytes: Vec<u8> = request(reqwest::Client::new().get(url))
            .await?
            .try_collect::<Vec<Bytes>>()
            .await?
            .into_iter()
            .flat_map(|b| b.to_vec())
            .collect();
        let reqs: Vec<UpstreamWalkRequest> = serde_json::from_slice(&bytes)
            .map_err(|e| {
                Error::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e)
            })?;
        Ok(reqs)
    }
}
