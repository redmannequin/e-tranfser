mod client;
mod error;
pub mod model;

use serde::Deserialize;
use uuid::Uuid;

pub use self::{client::TlClient, error::TlError, model::CreatePaymentResponse};

#[derive(Debug, Clone, Deserialize)]
pub struct TlConfig {
    pub enviornment: TlEnviorment,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub merchant_account_id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TlEnviorment {
    Mock { url: String },
    Sandbox,
    Production,
}

impl TlEnviorment {
    const SANDBOX_URI: &str = "truelayer-sandbox.com";
    const PRODUCTION_URI: &str = "truelayer.com";

    pub fn uri(&self) -> String {
        match self {
            TlEnviorment::Mock { url } => url.clone(),
            TlEnviorment::Sandbox => Self::SANDBOX_URI.into(),
            TlEnviorment::Production => Self::PRODUCTION_URI.into(),
        }
    }
}
