#![feature(iter_intersperse)]

mod core;

mod clients;
mod docs;
mod external_apis;
mod handlers;
mod middlewares;
mod utils;

use actix_web::{
    middleware::Logger,
    web::{get, post, put, scope, Data},
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
use handlers::account::{login_by_password, login_by_sms_verification_code};
use middlewares::auth::AuthMiddlewareFactory;
use nb_from_env::{FromEnv, FromEnvDerive};
use std::fs::create_dir_all;
use std::sync::Arc;

#[derive(FromEnvDerive)]
pub struct Config {
    pub listen_address: String,
    #[env_default("info")]
    pub log_level: String,
    #[env_default("%t %s %r %a %D")]
    pub log_format: String,
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
    env_logger::init_from_env(
        env_logger::Env::new().default_filter_or(config.log_level),
    );
    let service = Data::new(Service::new(
        AuthClient::new("localhost:8001"),
        UploadClient::new("localhost:8002"),
        SMSVerificationCodeClient::new("localhost:8003"),
        DogClient::new("localhost:8004"),
        WalkRequestClient::new("localhost:8005"),
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
            .service(
                scope("accounts")
                    .service(
                        scope("login")
                            .route(
                                "by_sms_verification_code",
                                put().to(login_by_sms_verification_code::<
                                    AuthClient,
                                    UploadClient,
                                    SMSVerificationCodeClient,
                                    DogClient,
                                    WalkRequestClient,
                                >),
                            )
                            .route(
                                "by_password",
                                put().to(login_by_password::<
                                    AuthClient,
                                    UploadClient,
                                    SMSVerificationCodeClient,
                                    DogClient,
                                    WalkRequestClient,
                                >),
                            ),
                    )
                    .route(
                        "signup",
                        post().to(handlers::account::signup::<
                            AuthClient,
                            UploadClient,
                            SMSVerificationCodeClient,
                            DogClient,
                            WalkRequestClient,
                        >),
                    )
                    .route(
                        "phones/{phone}/verification_codes",
                        put().to(handlers::account::send_verification_code::<
                            AuthClient,
                            UploadClient,
                            SMSVerificationCodeClient,
                            DogClient,
                            WalkRequestClient,
                        >),
                    )
                    .route(
                        "tokens/{token}/verification",
                        get().to(handlers::account::verify_auth_token::<
                            AuthClient,
                            UploadClient,
                            SMSVerificationCodeClient,
                            DogClient,
                            WalkRequestClient,
                        >),
                    ),
            )
            .service(
                scope("apis")
                    .wrap(auth_middleware_factory.clone())
                    .service(
                        scope("dogs")
                            .route(
                                "",
                                post().to(handlers::dog::add_dog::<
                                    AuthClient,
                                    UploadClient,
                                    SMSVerificationCodeClient,
                                    DogClient,
                                    WalkRequestClient,
                                >),
                            )
                            .route(
                                "",
                                get().to(handlers::common::pass_through(
                                    "localhost:8004",
                                    Some("/dogs"),
                                )),
                            )
                            .route(
                                "portraits",
                                post().to(handlers::dog::upload_portrait::<
                                    AuthClient,
                                    UploadClient,
                                    SMSVerificationCodeClient,
                                    DogClient,
                                    WalkRequestClient,
                                >),
                            )
                            .route(
                                "portraits/{id}",
                                get().to(handlers::dog::download_portrait::<
                                    AuthClient,
                                    UploadClient,
                                    SMSVerificationCodeClient,
                                    DogClient,
                                    WalkRequestClient,
                                >),
                            )
                            .route(
                                "mine",
                                get().to(handlers::common::pass_through(
                                    "localhost:8004",
                                    None,
                                )),
                            )
                            .route(
                                "{id}/portrait",
                                put().to(handlers::dog::update_dog_portrait::<
                                    AuthClient,
                                    UploadClient,
                                    SMSVerificationCodeClient,
                                    DogClient,
                                    WalkRequestClient,
                                >),
                            )
                            .route(
                                "breeds",
                                get().to(handlers::dog::query_breeds::<
                                    AuthClient,
                                    UploadClient,
                                    SMSVerificationCodeClient,
                                    DogClient,
                                    WalkRequestClient,
                                >),
                            )
                            .route(
                                "{id}",
                                put().to(handlers::dog::update_dog::<
                                    AuthClient,
                                    UploadClient,
                                    SMSVerificationCodeClient,
                                    DogClient,
                                    WalkRequestClient,
                                >),
                            ),
                    )
                    .service(
                        scope("/walk_requests")
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
                                "",
                                post().to(handlers::common::pass_through(
                                    "localhost:8005",
                                    None,
                                )),
                            ),
                    ),
            )
    })
    .bind(config.listen_address)?
    .run()
    .await
}
