use actix_web::{error::ErrorInternalServerError, Error};
use reqwest::{Method, Url};
use serde::Deserialize;
use serde_json::from_str;

pub(crate) async fn request<T>(method: Method, url: Url) -> Result<T, Error>
where
    for<'de> T: Deserialize<'de>,
{
    let client = reqwest::Client::new();
    match client.request(method, url).send().await {
        Ok(response) => match response.error_for_status() {
            Ok(response) => match response.text().await {
                Ok(text) => match from_str(&text) {
                    Ok(data) => Ok(data),
                    Err(e) => Err(ErrorInternalServerError(e)),
                },
                Err(e) => Err(ErrorInternalServerError(e)),
            },
            Err(e) => Err(ErrorInternalServerError(e)),
        },
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}
