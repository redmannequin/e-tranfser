use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreatePaymentResponse {
    #[serde(rename = "id")]
    pub payment_id: Uuid,
    pub user: UserObj,
    pub resource_token: String,
    #[serde(flatten)]
    pub status: CreatePaymentStatus,
}

#[derive(Debug, Deserialize)]
pub struct UserObj {
    #[serde(rename = "id")]
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum CreatePaymentStatus {
    AuthorizationRequired,
    Authorized,
    Failed { failure_reason: String },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureStage {
    AuthorizationRequired,
    Authorizing,
    Authorized,
}

#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    pub token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthErrorResponse {
    pub error: String,
    pub error_description: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePayoutResponse {
    #[serde(rename = "id")]
    pub payout_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct GetAccounts {
    pub results: Vec<Account>,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    pub account_id: String,
    pub account_type: String,
    pub display_name: String,
    pub currency: String,
    pub account_number: AccountNumber,
}

#[derive(Debug, Deserialize)]
pub struct AccountNumber {
    pub number: Option<String>,
    pub sort_code: Option<String>,
    pub iban: String,
}

#[derive(Debug, Deserialize)]
pub struct GetAccountBalance {
    pub results: Vec<AccountBalance>,
}

#[derive(Debug, Deserialize)]
pub struct AccountBalance {
    pub currency: String,
    pub current: f32,
}
