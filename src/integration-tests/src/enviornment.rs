use std::{net::TcpListener, str::FromStr, thread::JoinHandle, time::Duration};

use gateway::{AppConfig, DbConfig, TlConfig, TlEnviorment};
use sqlx::{postgres::PgConnectOptions, ConnectOptions};
use uuid::Uuid;

use crate::{tl_mock::TlMock, wait_for_conntection};

pub struct MockEnv {
    pub base_url: String,
    pub tl_mock: TlMock,
    _server_join_handle: JoinHandle<()>,
}

impl MockEnv {
    pub async fn init() -> MockEnv {
        let http_port = TcpListener::bind(("0.0.0.0", 0))
            .unwrap()
            .local_addr()
            .expect("http_port")
            .port();

        let tl_client_id = String::from("test-client");
        let tl_client_redirect_uri = String::from("test");
        let merchant_account_id = Uuid::new_v4();

        let db_name = init_db().await;
        let tl_mock = TlMock::init().await;
        tl_mock.add_client(tl_client_id.clone(), tl_client_redirect_uri.clone());
        tl_mock.add_merchant_account(&tl_client_id, merchant_account_id, "GBP".into(), 0);

        let config = AppConfig {
            http_port,
            db_config: DbConfig {
                name: db_name,
                host: "localhost".into(),
                port: 5432,
                username: "postgres".into(),
                password: "password".into(),
            },
            tl_config: TlConfig {
                client_id: tl_client_id,
                client_secret: tl_client_redirect_uri,
                merchant_account_id,
                kid: Uuid::new_v4().to_string(),
                private_key: "test".into(),
                redirect_uri: "".into(),
                data_redirect_uri: "".into(),
                enviornment: TlEnviorment::Mock {
                    url: tl_mock.base_url().into(),
                },
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
            tl_mock,
            _server_join_handle: server_join_handle,
        }
    }
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
