use http::{request, StatusCode};
use url::Url;

use crate::{
    core::{
        error::Error,
        walk_request_client::WalkRequestClient as IWalkRequestClient,
    },
    utils::restful::request,
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
    async fn nearby_requests(
        &self,
        lat: f64,
        long: f64,
        page: i32,
        size: i32,
    ) -> Result<crate::core::service::ByteStream, crate::core::error::Error>
    {
        request(
            reqwest::Client::new().get(
                Url::parse_with_params(
                    &format!(
                        "http://{}/walk_requests/nearby",
                        self.host_and_port
                    ),
                    vec![
                        ("lat", lat.to_string()),
                        ("long", long.to_string()),
                        ("page", page.to_string()),
                        ("size", size.to_string()),
                    ],
                )
                .map_err(|e| {
                    Error::new(StatusCode::INTERNAL_SERVER_ERROR, e)
                })?,
            ),
        )
        .await
    }
}
