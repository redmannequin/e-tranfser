use actix_web::{http::header, post, web, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::{db::{CreatePayment, PaymentState}, AppContext};

use super::PublicError;

#[derive(Debug, Deserialize)]
pub struct FormData {
    payer_full_name: String,
    payer_email: String,
    payee_full_name: String,
    payee_email: String,
    amount: u32,
    security_question: String,
    security_answer: String,
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
    form: web::Form<FormData>,
) -> Result<impl Responder, PublicError> {
    execute(app, form.0).await
}

#[instrument(skip(app))]
async fn execute(
    app: web::Data<AppContext>,
    form: FormData,
) -> Result<impl Responder, PublicError> {
    let payment = app
        .tl_client
        .create_ma_payment(
            &form.payer_full_name,
            &form.payer_email,
            None,
            form.amount,
            "test",
        )
        .await
        .map_err(|_| PublicError::InternalServerError)?;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let security_answer = argon2
        .hash_password(form.security_answer.as_bytes(), &salt)
        .map_err(|_| PublicError::InternalServerError)?
        .to_string();

    app.db_client
        .insert_payment(CreatePayment {
            payment_id: payment.payment_id,
            payer_full_name: form.payer_full_name,
            payer_email: form.payer_email,
            payee_full_name: form.payee_full_name,
            payee_email: form.payee_email,
            amount: form.amount,
            security_question: form.security_question,
            security_answer,
            state: PaymentState::InboundCreated,
        })
        .await?;

    let auth_link = format!(
        "https://payment.truelayer-sandbox.com/payments#payment_id={}&resource_token={}&return_uri={}", 
        payment.payment_id, 
        payment.resource_token, 
        app.tl_client.return_uri()
    );

    Ok(HttpResponse::SeeOther()
        .insert_header((header::LOCATION, auth_link))
        .finish())
}
