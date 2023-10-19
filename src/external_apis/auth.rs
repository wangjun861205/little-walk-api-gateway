use actix_web::{error::ErrorInternalServerError, Error};
use reqwest::Method;

use super::common::request;

pub async fn gen_token(url: String, phone: String) -> Result<String, Error> {
    request(Method::PUT, url)
}
