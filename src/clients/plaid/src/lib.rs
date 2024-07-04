mod client;
mod error;
pub mod model;

use serde::Deserialize;

pub use crate::{client::PlaidClient, error::PlaidError};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlaidEnviorment {
    Mock { url: String },
    Sandbox,
    Production,
}

impl PlaidEnviorment {
    const SANDBOX_URI: &'static str = "sandbox.plaid.com";
    const PRODUCTION_URI: &'static str = "plaid.com";

    pub fn uri(&self) -> String {
        match self {
            PlaidEnviorment::Mock { url } => url.clone(),
            PlaidEnviorment::Sandbox => Self::SANDBOX_URI.into(),
            PlaidEnviorment::Production => Self::PRODUCTION_URI.into(),
        }
    }
}
