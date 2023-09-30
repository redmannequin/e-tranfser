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
    pub acesse_token: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    pub token_type: String,
}
