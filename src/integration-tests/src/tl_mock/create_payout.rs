use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{deserialize_body, AppContext, PublicError};

#[derive(Debug, Deserialize)]
struct Request {
    merchant_account_id: Uuid,
    amount_in_minor: u32,
    currency: String,
    beneficiary: Beneficiary,
}

#[derive(Debug, Deserialize)]
pub struct Beneficiary {
    r#type: String,
    reference: String,
    account_holder_name: String,
    account_identifier: AccountTdentifier,
}

#[derive(Debug, Deserialize)]
pub struct AccountTdentifier {
    r#type: String,
    iban: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    #[serde(rename = "id")]
    payout_id: Uuid,
}

#[post("/v3/payouts")]
pub async fn create_payout(
    app: web::Data<AppContext>,
    request: HttpRequest,
    body: String,
) -> Result<impl Responder, PublicError> {
    let client_id = request
        .headers()
        .get("authorization")
        .unwrap()
        .to_str()
        .unwrap();

    let request = deserialize_body(&body)?;
    execute(app, &client_id, request).await
}

async fn execute(
    app: web::Data<AppContext>,
    client_id: &str,
    request: Request,
) -> Result<impl Responder, PublicError> {
    let payout_id = match request.beneficiary.r#type.as_str() {
        "external_account" => match request.beneficiary.account_identifier.r#type.as_str() {
            "iban" => app
                .state
                .create_payout(
                    client_id,
                    request.merchant_account_id,
                    &request.currency,
                    request.amount_in_minor as _,
                )
                .ok_or_else(|| {
                    PublicError::Invalid(
                        "Invalid merchant_account_id currency pair or balance too low".into(),
                    )
                })?,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    Ok(HttpResponse::Created().json(Response { payout_id }))
}
