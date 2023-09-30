use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{deserialize_body, AppContext, PublicError};

#[derive(Debug, Deserialize)]
struct Request {
    amount_in_minor: u32,
    currency: String,
    payment_method: PaymentMethod,
    user: UserObj,
}

#[derive(Debug, Deserialize)]
struct PaymentMethod {
    r#type: String,
    beneficiary: Beneficiary,
}

#[derive(Debug, Deserialize)]
struct Beneficiary {
    r#type: String,
    merchant_account_id: Uuid,
}

#[derive(Debug, Deserialize)]
struct UserObj {
    name: String,
}

#[derive(Debug, Serialize)]
struct Response {
    id: Uuid,
    user: ResponseUserObj,
    resource_token: String,
    status: String,
}

#[derive(Debug, Serialize)]
struct ResponseUserObj {
    #[serde(rename = "id")]
    user_id: Uuid,
}

#[post("/v3/payments")]
pub async fn create_payment(
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
    let payment_id = match request.payment_method.r#type.as_str() {
        "bank_transfer" => match request.payment_method.beneficiary.r#type.as_str() {
            "merchant_account" => app
                .state
                .create_ma_payment(
                    client_id,
                    request.payment_method.beneficiary.merchant_account_id,
                    &request.currency,
                    request.amount_in_minor as _,
                )
                .ok_or_else(|| {
                    PublicError::Invalid("Invalid merchant_account_id currency pair".into())
                })?,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    Ok(HttpResponse::Created().json(Response {
        id: payment_id,
        user: ResponseUserObj {
            user_id: Uuid::new_v4(),
        },
        resource_token: "test".into(),
        status: "authorization_required".into(),
    }))
}
