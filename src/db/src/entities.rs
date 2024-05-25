use serde::{Deserialize, Serialize};
use tokio_postgres::types::Json;
use uuid::Uuid;

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
        payment_state: PaymentState,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PaymentState {
    // inbound status
    InboundCreated,
    InboundAuthorizing,
    InboundAuthorized,
    InboundExecuted,
    InboundSettled,
    InboundFailed,
    // refund status
    RefundCreated,
    RefundAuthorized,
    RefundExecuted,
    RefundFailed,
    // outbound status
    OutboundCreated,
    OutboundAuthorized,
    OutboundExecuted,
    OutboundFaild,
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
    V1 {
        first_name: String,
        last_name: String,
    },
}
