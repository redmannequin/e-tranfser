use std::{
    net::{SocketAddr, TcpListener},
    thread::JoinHandle,
    time::Duration,
};

use gateway::AppConfig;
use tokio::net::TcpStream;

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
        let config = AppConfig { http_port };
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
