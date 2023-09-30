use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::{db::CreatePayment, AppContext};

use super::{deserialize_body, PublicError};

#[derive(Debug, Deserialize)]
struct Request {
    payer: Payer,
    full_name: String,
    email: String,
    amount: u32,
    security_question: String,
    security_answer: String,
}

#[derive(Debug, Deserialize)]
struct Payer {
    full_name: String,
    email: String,
    phonenumber: Option<String>,
}

#[derive(Debug, Serialize)]
struct Response {
    payment_id: Uuid,
    resource_token: String,
    return_uri: String,
}

#[post("/create_payment")]
pub async fn create_payment(
    app: web::Data<AppContext>,
    _request: HttpRequest,
    body: String,
) -> Result<impl Responder, PublicError> {
    let request = deserialize_body(&body)?;
    execute(app, request).await
}

#[instrument(skip(app))]
async fn execute(
    app: web::Data<AppContext>,
    request: Request,
) -> Result<impl Responder, PublicError> {
    let payment = app
        .tl_client
        .create_ma_payment(
            &request.payer.full_name,
            &request.payer.email,
            request.payer.phonenumber.as_deref(),
            request.amount,
            "test",
        )
        .await
        .map_err(|_| PublicError::InternalServerError)?;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let security_answer = argon2
        .hash_password(request.security_answer.as_bytes(), &salt)
        .map_err(|_| PublicError::InternalServerError)?
        .to_string();

    app.db_client
        .insert_payment(CreatePayment {
            payment_id: payment.payment_id,
            full_name: request.full_name,
            email: request.email,
            amount: request.amount,
            security_question: request.security_question,
            security_answer,
            deposited: false,
        })
        .await?;

    Ok(HttpResponse::Created().json(Response {
        payment_id: payment.payment_id,
        resource_token: payment.resource_token,
        return_uri: app.tl_client.return_uri().into(),
    }))
}
