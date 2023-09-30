use std::{net::SocketAddr, time::Duration};

use tokio::net::TcpStream;

pub mod enviornment;
pub mod tl_mock;

pub async fn wait_for_conntection(port: u16, timeout: Duration) {
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
