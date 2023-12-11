#![feature(iter_intersperse)]

mod core;

mod clients;
mod docs;
mod handlers;
mod middlewares;
mod utils;

use actix_web::{
    middleware::Logger,
    web::{self, get, scope, Data},
    App, HttpServer,
};
use clients::{
    auth_clients::restful::AuthClient, dog_clients::restful::DogClient,
    sms_verification_code_clients::restful::SMSVerificationCodeClient,
    upload_clients::restful::UploadClient,
    walk_request_clients::restful::WalkRequestClient,
};
use core::service::Service;
use docs::api::generate_api_doc;
use handlers::{common::pass_through, walk_request::fill_dogs};
use middlewares::auth::AuthMiddlewareFactory;
use nb_from_env::{FromEnv, FromEnvDerive};
use std::fs::create_dir_all;
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
    create_dir_all("./docs").expect("failed to create docs dir");
    generate_api_doc("./docs/api-spec.yml").expect("failed to generate docs");
    dotenv::dotenv().ok();
    let config = Config::from_env();
    env_logger::init_from_env(
        env_logger::Env::new().default_filter_or(&config.log_level),
    );
    let service = Data::new(Service::new(
        AuthClient::new(&config.auth_service_address),
        UploadClient::new(&config.upload_service_address),
        SMSVerificationCodeClient::new(
            &config.sms_verification_code_service_address,
        ),
        DogClient::new(&config.dog_service_address),
        WalkRequestClient::new(&config.walk_request_service_address),
    ));
    let auth_middleware_factory = Arc::new(AuthMiddlewareFactory::new(
        AuthClient::new("localhost:8001"),
    ));
    HttpServer::new(move || {
        let logger = Logger::new(&config.log_format)
            .log_target("little-walk-api-gateway");
        App::new()
            .wrap(logger)
            .app_data(service.clone())
            .service(scope("accounts").default_service(web::route().to(
                pass_through(&config.auth_service_address, None, |bytes| {
                    Box::pin(async { Ok(bytes) })
                }),
            )))
            .service(
                scope("apis")
                    .wrap(auth_middleware_factory.clone())
                    .service(scope("dogs").default_service(web::route().to(
                        pass_through(
                            &config.dog_service_address,
                            None,
                            |bytes| Box::pin(async { Ok(bytes) }),
                        ),
                    )))
                    .service(
                        scope("/walk_requests")
                            .default_service(web::route().to(pass_through(
                                &config.walk_request_service_address,
                                None,
                                |bytes| Box::pin(async { Ok(bytes) }),
                            )))
                            .route(
                                "nearby",
                                get().to(
                                    handlers::walk_request::nearby_requests::<
                                        AuthClient,
                                        UploadClient,
                                        SMSVerificationCodeClient,
                                        DogClient,
                                        WalkRequestClient,
                                    >,
                                ),
                            )
                            .route(
                                "/{id}/accepted_by",
                                web::route().to(pass_through(
                                    &config.walk_request_service_address,
                                    None,
                                    fill_dogs(service.clone()),
                                )),
                            )
                            .route(
                                "/{id}/start",
                                web::route().to(pass_through(
                                    &config.walk_request_service_address,
                                    None,
                                    fill_dogs(service.clone()),
                                )),
                            )
                            .route(
                                "/{id}/finish",
                                web::route().to(pass_through(
                                    &config.walk_request_service_address,
                                    None,
                                    fill_dogs(service.clone()),
                                )),
                            ),
                    )
                    .service(scope("/uploads").default_service(
                        web::route().to(pass_through(
                            &config.upload_service_address,
                            None,
                            |bytes| Box::pin(async { Ok(bytes) }),
                        )),
                    )),
            )
    })
    .bind(config.listen_address)?
    .run()
    .await
}
