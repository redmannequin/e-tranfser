pub mod v1;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::types::Json;
use uuid::Uuid;

use crate::entities::v1::UserDataV1;

////////////////////////////////////////////////////////////////////////////////
// Payment
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Payment {
    pub payment_id: Uuid,
    pub payment_data: Json<PaymentData>,
}

#[derive(Debug, Clone)]
pub struct UserPayment {
    pub payment_id: Uuid,
    pub user_id: Uuid,
    pub payment_data: Json<PaymentData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum PaymentData {
    V1 {
        payer_full_name: String,
        payer_email: String,
        payee_full_name: String,
        payee_email: String,
        amount: u32,
        security_question: String,
        security_answer: String,
        payment_statuses: PaymentStatuses,
        payout_data: Option<PayoutData>,
        refund_data: Option<RefundData>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PayoutData {
    PayoutRegistering {
        payout_registered_at: DateTime<Utc>,
    },
    PayoutCreated {
        payout_id: Uuid,
        payout_statuses: PayoutStatuses,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutStatuses {
    pub payout_created_at: DateTime<Utc>,
    pub payout_executed_at: Option<DateTime<Utc>>,
    pub payout_failed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundData {
    pub refund_id: Uuid,
    pub refund_statuses: RefundStatuses,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundStatuses {
    pub refund_created_at: DateTime<Utc>,
    pub refund_executed_at: Option<DateTime<Utc>>,
    pub refund_failed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentStatuses {
    pub inbound_created_at: DateTime<Utc>,
    pub inbound_authorized_at: Option<DateTime<Utc>>,
    pub inbound_executed_at: Option<DateTime<Utc>>,
    pub inbound_settled_at: Option<DateTime<Utc>>,
    pub inbound_failed_at: Option<DateTime<Utc>>,
}

////////////////////////////////////////////////////////////////////////////////
// User
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct User {
    pub user_id: Uuid,
    pub email: String,
    pub user_data: Json<UserData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum UserData {
    V1(UserDataV1),
}
