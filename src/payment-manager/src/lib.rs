use std::{net::ToSocketAddrs, sync::Arc};

use anyhow::Context;
use contracts::payment_manager::server::PaymentManagerServer;
use db::{DbClient, DbConfig};
use my_payment_manager::MyPaymentManager;
use serde::Deserialize;
use tonic::transport::{server::TcpIncoming, Server};
use truelayer::{TlClient, TlConfig};

mod handlers;
mod log;
mod my_payment_manager;
mod otel;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub grpc_port: u16,
    pub db_config: DbConfig,
    pub tl_config: TlConfig,
}

pub struct AppContext {
    pub db_client: DbClient,
    pub tl_client: TlClient,
}

impl AppContext {
    pub async fn init(config: AppConfig) -> anyhow::Result<Self> {
        Ok(AppContext {
            db_client: DbClient::connect(config.db_config)
                .await
                .context("postgres connection")?,
            tl_client: TlClient::new(config.tl_config)
                .await
                .context("truelayer connection")?,
        })
    }
}

pub async fn start(config: AppConfig) -> anyhow::Result<()> {
    let port = config.grpc_port;
    let app = Arc::new(AppContext::init(config).await?);
    let addr = ("0.0.0.0", port)
        .to_socket_addrs()
        .context("addr")?
        .next()
        .context("addr")?;

    let incoming =
        TcpIncoming::new(addr, true, None).map_err(|_| anyhow::anyhow!("Failed to bind gRPC"))?;

    Server::builder()
        .add_service(PaymentManagerServer::new(MyPaymentManager::new(app)))
        .serve_with_incoming(incoming)
        .await
        .context("gRPC Server Error")
}
