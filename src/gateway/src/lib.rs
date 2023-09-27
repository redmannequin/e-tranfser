use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use anyhow::Context;

pub struct AppConfig {
    pub http_port: u16,
}

pub async fn start(config: AppConfig) -> anyhow::Result<()> {
    let app_context = ();

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(app_context)
            .wrap(Logger::default())
            .service(web::resource("/health_check").route(web::get().to(HttpResponse::Ok)))
    })
    .bind(("0.0.0.0", config.http_port))?
    .run();

    http_server.await.context("http_server")
}
