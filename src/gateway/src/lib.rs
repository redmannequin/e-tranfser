mod api;
mod app;
mod root_span;

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use actix_web_opentelemetry::RequestTracing;
use anyhow::Context;
use db::DbClient;
use root_span::DomainRootSpanBuilder;
use serde::Deserialize;
use tracing_actix_web::TracingLogger;

pub use db::DbConfig;
pub use truelayer::{TlClient, TlConfig, TlEnviorment};

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub http_port: u16,
    pub db_config: DbConfig,
    pub tl_config: TlConfig,
}

pub struct AppContext {
    db_client: DbClient,
    tl_client: TlClient,
}

impl AppContext {
    pub async fn init(config: AppConfig) -> anyhow::Result<Self> {
        Ok(AppContext {
            db_client: DbClient::connect(config.db_config)
                .await
                .context("postgres connection")?,
            tl_client: TlClient::new(config.tl_config),
        })
    }
}

pub async fn start(config: AppConfig) -> anyhow::Result<()> {
    let app_context = web::Data::new(AppContext::init(config.clone()).await?);

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(app_context.clone())
            .wrap(Logger::default())
            .wrap(RequestTracing::new())
            .wrap(TracingLogger::<DomainRootSpanBuilder>::new())
            .service(web::resource("/health_check").route(web::get().to(HttpResponse::Ok)))
            .service(web::resource("/").to(app::home))
            .service(web::resource("/callback").to(app::tl_callback))
            .service(web::resource("/data_callback").to(app::tl_data_callback))
            .service(web::resource("/deposit_select_account").to(app::deposit_select_account))
            .service(web::resource("/deposit").to(app::deposit))
            .service(web::resource("/payment_sent").to(app::payment_sent))
            .service(web::resource("/payment").to(app::payment))
            .service(web::resource("/register").to(app::register))
            .service(
                web::scope("/api")
                    .service(api::create_payment::create_payment)
                    .service(api::deposit_payment::deposit_payment)
                    .service(api::tl_webhooks::tl_webhook),
            )
            .default_service(web::to(app::not_found))
    })
    .bind(("0.0.0.0", config.http_port))?
    .run();

    http_server.await.context("http_server")
}
