#![feature(iter_intersperse)]

mod clients;
mod core;
mod handlers;
mod middlewares;
mod utils;

use crate::clients::auth_clients::restful::AuthClient;
use actix_web::{
    middleware::Logger,
    web::{self, scope, Data},
    App, HttpServer,
};
use core::service::Service;
use handlers::common::pass_through;
use middlewares::auth::AuthMiddlewareFactory;
use nb_from_env::{FromEnv, FromEnvDerive};
use std::sync::Arc;

#[derive(FromEnvDerive, Clone)]
pub struct Config {
    pub listen_address: String,
    #[env_default("info")]
    pub log_level: String,
    #[env_default("%t %s %r %a %D")]
    pub log_format: String,
    pub auth_service_address: String,
    pub upload_service_address: String,
    pub sms_verification_code_service_address: String,
    pub dog_service_address: String,
    pub walk_request_service_address: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let config = Config::from_env();
    env_logger::init_from_env(
        env_logger::Env::new().default_filter_or(&config.log_level),
    );
    let service = Data::new(Service::new());
    let auth_middleware_factory = Arc::new(AuthMiddlewareFactory::new(
        AuthClient::new(&config.auth_service_address),
    ));
    HttpServer::new(move || {
        let logger = Logger::new(&config.log_format)
            .log_target("little-walk-api-gateway");
        App::new()
            .wrap(logger)
            .app_data(service.clone())
            .service(scope("accounts").default_service(web::route().to(
                pass_through(
                    &config.auth_service_address,
                    None,
                    service.no_op_request_body_processor(),
                    service.no_op_processor(),
                ),
            )))
            .service(
                scope("apis")
                    .wrap(auth_middleware_factory.clone())
                    .service(
                        scope("dogs")
                            .default_service(web::route().to(pass_through(
                                &config.dog_service_address,
                                None,
                                service.no_op_request_body_processor(),
                                service.no_op_processor(),
                            )))
                            .route(
                                "",
                                web::post().to(pass_through(
                                    &config.dog_service_address,
                                    None,
                                    service.create_dog_request_body_processor(),
                                    service.no_op_processor(),
                                )),
                            ),
                    )
                    .service(scope("breeds").default_service(web::route().to(
                        pass_through(
                            &config.dog_service_address,
                            None,
                            service.no_op_request_body_processor(),
                            service.no_op_processor(),
                        ),
                    )))
                    .service(scope("/walk_requests").default_service(
                        web::route().to(pass_through(
                            &config.walk_request_service_address,
                            None,
                            service.no_op_request_body_processor(),
                            service.no_op_processor(),
                        )),
                    ))
                    .service(scope("/uploads").default_service(
                        web::route().to(pass_through(
                            &config.upload_service_address,
                            None,
                            service.no_op_request_body_processor(),
                            service.no_op_processor(),
                        )),
                    )),
            )
    })
    .bind(config.listen_address)?
    .run()
    .await
}
