use gateway::AppConfig;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig { http_port: 3000 };
    gateway::start(config).await
}
