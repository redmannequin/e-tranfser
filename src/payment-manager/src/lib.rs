use std::sync::Arc;

use anyhow::Context;
use contracts::payment_manager::server::PaymentManagerServer;
use db::{DbClient, DbConfig};
use my_payment_manager::MyPaymentManager;
use serde::Deserialize;
use tonic::transport::{
    server::{Router, TcpIncoming},
    Server,
};
use truelayer::{TlClient, TlConfig};

mod handlers;
mod my_payment_manager;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub http_port: u16,
    pub db_config: DbConfig,
    pub tl_config: TlConfig,
    pub otel_config: OtelConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OtelConfig {
    pub service_name: String,
    pub exporter_otlp_endpoint: String,
    pub exporter_otlp_headers: Vec<(String, String)>,
    pub exporter_otlp_protocol: String,
}

pub struct AppContext {
    pub db_client: DbClient,
    pub tl_client: TlClient,
}

pub async fn start(config: AppConfig) -> anyhow::Result<()> {
    todo!()
}

pub struct PaymentManager {
    router: Router,
    incoming: TcpIncoming,
}

impl PaymentManager {
    pub async fn build_and_bind(app: Arc<AppContext>) -> anyhow::Result<Self> {
        let addr = "[::1]:50051".parse().context("addr")?;

        let incoming = TcpIncoming::new(addr, true, None)
            .map_err(|_| anyhow::anyhow!("Failed to bind gRPC"))?;

        let payment_manager = MyPaymentManager::new(app);
        let router = Server::builder().add_service(PaymentManagerServer::new(payment_manager));

        Ok(PaymentManager { router, incoming })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        self.router
            .serve_with_incoming(self.incoming)
            .await
            .context("gRPC Server Error")
    }
}
