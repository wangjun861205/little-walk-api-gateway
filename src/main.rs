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
};
use core::service::Service;
use docs::api::generate_api_doc;
use handlers::account::{login_by_password, login_by_sms_verification_code};
use middlewares::auth::AuthMiddlewareFactory;
use nb_from_env::{FromEnv, FromEnvDerive};
use std::fs::create_dir_all;
use std::sync::Arc;
use url::Url;

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
        AuthClient::new(
            Url::parse("http://localhost:8001")
                .expect("invalid auth service url"),
        ),
        UploadClient::new(
            Url::parse("http://localhost:8002")
                .expect("invalid upload service url"),
        ),
        SMSVerificationCodeClient::new(
            Url::parse("http://localhost:8003")
                .expect("invalid sms verification code service url"),
        ),
        DogClient::new(
            Url::parse("http://localhost:8004")
                .expect("invalid dog service url"),
        ),
    ));
    let auth_middleware_factory =
        Arc::new(AuthMiddlewareFactory::new(AuthClient::new(
            Url::parse("http://localhost:8001")
                .expect("invalid auth service url"),
        )));
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
                                >),
                            )
                            .route(
                                "by_password",
                                put().to(login_by_password::<
                                    AuthClient,
                                    UploadClient,
                                    SMSVerificationCodeClient,
                                    DogClient,
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
                        >),
                    )
                    .route(
                        "phones/{phone}/verification_codes",
                        put().to(handlers::account::send_verification_code::<
                            AuthClient,
                            UploadClient,
                            SMSVerificationCodeClient,
                            DogClient,
                        >),
                    )
                    .route(
                        "tokens/{token}/verification",
                        get().to(handlers::account::verify_auth_token::<
                            AuthClient,
                            UploadClient,
                            SMSVerificationCodeClient,
                            DogClient,
                        >),
                    ),
            )
            .service(
                scope("apis").wrap(auth_middleware_factory.clone()).service(
                    scope("dogs")
                        .route(
                            "",
                            post().to(handlers::dog::add_dog::<
                                AuthClient,
                                UploadClient,
                                SMSVerificationCodeClient,
                                DogClient,
                            >),
                        )
                        .route(
                            "portraits",
                            post().to(handlers::dog::upload_portrait::<
                                AuthClient,
                                UploadClient,
                                SMSVerificationCodeClient,
                                DogClient,
                            >),
                        )
                        .route(
                            "portraits/{id}",
                            get().to(handlers::dog::download_portrait::<
                                AuthClient,
                                UploadClient,
                                SMSVerificationCodeClient,
                                DogClient,
                            >),
                        ),
                ),
            )
    })
    .bind(config.listen_address)?
    .run()
    .await
}
