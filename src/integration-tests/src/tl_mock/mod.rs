mod auth;
mod create_payment;
mod create_payout;
mod state;

use std::{net::TcpListener, thread::JoinHandle, time::Duration};

use actix_web::{web, App, HttpServer};
use anyhow::Context;
use serde::Deserialize;
use uuid::Uuid;

use crate::wait_for_conntection;

use self::state::State;

pub struct AppContext {
    pub state: State,
}

pub struct TlMock {
    base_url: String,
    state: State,
    _server_join_handle: JoinHandle<()>,
}

impl TlMock {
    pub async fn init() -> Self {
        let state = State::new();
        let app_context = web::Data::new(AppContext {
            state: state.clone(),
        });
        let http_port = TcpListener::bind(("0.0.0.0", 0))
            .unwrap()
            .local_addr()
            .expect("http_port")
            .port();

        let server_join_handle = std::thread::spawn(move || {
            actix_web::rt::System::new()
                .block_on(async {
                    let http_server = HttpServer::new(move || {
                        App::new()
                            .app_data(app_context.clone())
                            .service(auth::auth)
                            .service(create_payment::create_payment)
                            .service(create_payout::create_payout)
                    })
                    .bind(("0.0.0.0", http_port))?
                    .run();

                    http_server.await.context("tl-mock server")
                })
                .expect("tl_mock server")
        });

        wait_for_conntection(http_port, Duration::from_secs(4)).await;

        TlMock {
            base_url: format!("localhost:{http_port}"),
            state,
            _server_join_handle: server_join_handle,
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn add_client(&self, client_id: String, redirect_uri: String) {
        self.state.create_client(client_id, redirect_uri);
    }

    pub fn add_merchant_account(
        &self,
        client_id: &str,
        merchant_account_id: Uuid,
        currency: String,
        starting_balance: u64,
    ) {
        self.state
            .add_merchant_account(client_id, merchant_account_id, currency, starting_balance);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PublicError {
    #[error("Something went wrong. Please retry later.")]
    InternalServerError,
    #[error("{0}")]
    Invalid(String),
}

impl From<PublicError> for actix_web::Error {
    #[inline]
    fn from(err: PublicError) -> Self {
        match err {
            err @ PublicError::Invalid(_) => actix_web::error::ErrorBadRequest(err),
            err => actix_web::error::ErrorInternalServerError(err),
        }
    }
}

pub fn deserialize_body<'de, T>(body: &'de str) -> Result<T, PublicError>
where
    T: Deserialize<'de>,
{
    let jd = &mut serde_json::Deserializer::from_str(body);
    serde_path_to_error::deserialize(jd).map_err(|e| PublicError::Invalid(format!("{e}")))
}
