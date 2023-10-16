mod middlewares;

use actix_web::{App, HttpServer};
use nb_from_env::{FromEnv, FromEnvDerive};

#[derive(FromEnvDerive)]
pub struct Config {
    pub listen_address: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let config = Config::from_env();
    HttpServer::new(move || App::new())
        .bind(config.listen_address)?
        .run()
        .await
}
