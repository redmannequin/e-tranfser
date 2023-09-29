use std::{
    net::{SocketAddr, TcpListener},
    str::FromStr,
    thread::JoinHandle,
    time::Duration,
};

use gateway::{AppConfig, DbConfig};
use sqlx::{postgres::PgConnectOptions, ConnectOptions};
use tokio::net::TcpStream;
use uuid::Uuid;

pub struct MockEnv {
    pub base_url: String,
    _server_join_handle: JoinHandle<()>,
}

impl MockEnv {
    pub async fn init() -> MockEnv {
        let http_port = TcpListener::bind(("0.0.0.0", 0))
            .unwrap()
            .local_addr()
            .expect("http_port")
            .port();

        let db_name = init_db().await;
        let config = AppConfig {
            http_port,
            db_config: DbConfig {
                name: db_name,
                host: "localhost".into(),
                port: 5432,
                username: "postgres".into(),
                password: "password".into(),
            },
        };

        let server_join_handle = std::thread::spawn(move || {
            actix_web::rt::System::new()
                .block_on(async { gateway::start(config).await })
                .expect("gateway::start")
        });

        wait_for_conntection(http_port, Duration::from_secs(4)).await;

        MockEnv {
            base_url: format!("http://localhost:{http_port}"),
            _server_join_handle: server_join_handle,
        }
    }
}

async fn wait_for_conntection(port: u16, timeout: Duration) {
    async fn wait(port: u16) {
        let addr: SocketAddr = format!("127.0.0.1:{port}").parse().unwrap();
        loop {
            match TcpStream::connect(addr).await {
                Ok(_) => break,
                _ => (),
            }
        }
    }

    tokio::time::timeout(timeout, wait(port))
        .await
        .expect("Failed to start")
}

async fn init_db() -> String {
    let database_name = format!("etranser-{}", Uuid::new_v4());

    let mut root = PgConnectOptions::from_str(&format!(
        "postgres://postgres:password@localhost:5432/{}",
        database_name
    ))
    .unwrap()
    .database("postgres")
    .connect()
    .await
    .expect("Failed to connect to postgres");

    sqlx::query(&format!("CREATE DATABASE \"{}\"", database_name))
        .execute(&mut root)
        .await
        .expect("Failed to create database");

    sqlx::migrate!("../../migrations")
        .run(
            &sqlx::PgPool::connect(&format!(
                "postgres://postgres:password@localhost:5432/{}",
                database_name
            ))
            .await
            .unwrap(),
        )
        .await
        .expect("Failed to migrate the database");

    database_name
}
