use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////
// Payment
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentStatusesV1 {
    pub inbound_created_at: DateTime<Utc>,
    pub inbound_authorized_at: Option<DateTime<Utc>>,
    pub inbound_executed_at: Option<DateTime<Utc>>,
    pub inbound_settled_at: Option<DateTime<Utc>>,
    pub inbound_failed_at: Option<DateTime<Utc>>,
}

////////////////////////////////////////////////////////////////////////////////
// User
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "state")]
pub enum UserDataV1 {
    Registering {
        first_name: String,
        last_name: String,
        code: String,
        timestamp: DateTime<Utc>,
    },
    Registered {
        first_name: String,
        last_name: String,
    },
}

impl UserDataV1 {
    pub fn first_name(&self) -> &str {
        match self {
            UserDataV1::Registering { first_name, .. } => first_name,
            UserDataV1::Registered { first_name, .. } => first_name,
        }
    }

    pub fn last_name(&self) -> &str {
        match self {
            UserDataV1::Registering { last_name, .. } => last_name,
            UserDataV1::Registered { last_name, .. } => last_name,
        }
    }
}
