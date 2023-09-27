mod api;

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use anyhow::Context;

pub struct AppConfig {
    pub http_port: u16,
}

pub struct AppContext;

pub async fn start(config: AppConfig) -> anyhow::Result<()> {
    let app_context = web::Data::new(AppContext);

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(app_context.clone())
            .wrap(Logger::default())
            .service(web::resource("/health_check").route(web::get().to(HttpResponse::Ok)))
            .service(api::create_payment::create_payment)
    })
    .bind(("0.0.0.0", config.http_port))?
    .run();

    http_server.await.context("http_server")
}
