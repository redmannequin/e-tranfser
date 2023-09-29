mod api;
mod db;

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use actix_web_opentelemetry::RequestTracing;
use anyhow::Context;
use db::DbClient;
use serde::Deserialize;
use tracing_actix_web::TracingLogger;

pub use db::DbConfig;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub http_port: u16,
    pub db_config: DbConfig,
}

pub struct AppContext {
    db_client: DbClient,
}

impl AppContext {
    pub async fn init(config: AppConfig) -> anyhow::Result<Self> {
        Ok(AppContext {
            db_client: DbClient::connect(config.db_config)
                .await
                .context("postgres connection")?,
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
            .wrap(TracingLogger::default())
            .service(web::resource("/health_check").route(web::get().to(HttpResponse::Ok)))
            .service(api::create_payment::create_payment)
    })
    .bind(("0.0.0.0", config.http_port))?
    .run();

    http_server.await.context("http_server")
}
