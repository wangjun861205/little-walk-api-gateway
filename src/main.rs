mod external_apis;
mod handlers;
mod middlewares;

use actix_web::{
    web::{delete, get, post, put, scope, Data},
    App, HttpServer,
};
use handlers::account::{login, logout, register};
use middlewares::auth::AuthMW;
use nb_from_env::{FromEnv, FromEnvDerive};

#[derive(FromEnvDerive)]
pub struct Config {
    pub listen_address: String,
    pub auth_service_address: String,
    pub sms_verification_code_service_address: String,
}

#[derive(Debug)]
pub struct ServiceAddresses {
    pub auth_service_address: String,
    pub sms_verification_code_service_address: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let config = Config::from_env();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(ServiceAddresses {
                auth_service_address: config.auth_service_address.clone(),
                sms_verification_code_service_address: config
                    .sms_verification_code_service_address
                    .clone(),
            }))
            .service(
                scope("accounts")
                    .route("login", put().to(login))
                    .route("logout", delete().to(logout))
                    .route("register", post().to(register)),
            )
            .service(scope("apis").wrap(AuthMW::new(config.auth_service_address.clone())))
    })
    .bind(config.listen_address)?
    .run()
    .await
}
