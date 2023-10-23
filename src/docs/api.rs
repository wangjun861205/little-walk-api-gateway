use crate::handlers::account;
use anyhow::Error;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(account::login_by_sms_verification_code))]
struct APIDoc;

pub fn generate_api_doc(path: &str) -> Result<(), Error> {
    let doc = APIDoc::openapi().to_pretty_json()?;
    std::fs::write(path, doc)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn generate_api_doc() {
        let doc = APIDoc::openapi().to_pretty_json().unwrap();
        println!("{}", doc);
    }
}
