mod docs;
mod external_apis;
mod handlers;
mod middlewares;

use actix_web::{
    middleware::Logger,
    web::{delete, get, post, put, resource, route, scope, service, Data},
    App, HttpServer,
};
use docs::api::generate_api_doc;
use handlers::account::{login_by_sms_verification_code, logout, register};
use middlewares::auth::AuthMW;
use nb_from_env::{FromEnv, FromEnvDerive};
use std::fs::create_dir_all;

#[derive(FromEnvDerive)]
pub struct Config {
    pub listen_address: String,
    #[env_default("info")]
    pub log_level: String,
    #[env_default("%t %s %r %a %D")]
    pub log_format: String,
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
    create_dir_all("./docs").expect("failed to create docs dir");
    generate_api_doc("./docs/api-spec.yml").expect("failed to generate docs");
    dotenv::dotenv().ok();
    let config = Config::from_env();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(config.log_level));
    HttpServer::new(move || {
        let logger = Logger::new(&config.log_format).log_target("little-walk-api-gateway");
        App::new()
            .wrap(logger)
            .app_data(Data::new(ServiceAddresses {
                auth_service_address: config.auth_service_address.clone(),
                sms_verification_code_service_address: config
                    .sms_verification_code_service_address
                    .clone(),
            }))
            .service(
                scope("accounts")
                    .service(scope("login").route(
                        "by_sms_verification_code",
                        put().to(login_by_sms_verification_code),
                    ))
                    .route("logout", delete().to(logout))
                    .route("register", post().to(register)),
            )
            .service(scope("apis").wrap(AuthMW::new(config.auth_service_address.clone())))
    })
    .bind(config.listen_address)?
    .run()
    .await
}
