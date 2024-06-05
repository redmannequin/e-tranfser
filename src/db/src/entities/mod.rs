pub mod v1;

use serde::{Deserialize, Serialize};
use tokio_postgres::types::Json;
use uuid::Uuid;

use crate::entities::v1::{PaymentStatusesV1, UserDataV1};

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
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentStatuses {
    V1(PaymentStatusesV1),
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
