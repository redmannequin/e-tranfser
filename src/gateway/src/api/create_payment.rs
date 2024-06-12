use actix_web::{http::header, post, web, HttpResponse};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use chrono::Utc;
use domain::{Payment, PaymentId, PaymentState, PaymentStatuses};
use serde::Deserialize;

use crate::{log, AppContext};

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

#[post("/create_payment")]
pub async fn create_payment(
    app: web::Data<AppContext>,
    form: web::Form<FormData>,
) -> Result<HttpResponse, PublicError> {
    let form = form.0;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let security_answer = argon2
        .hash_password(form.security_answer.as_bytes(), &salt)
        .map_err(|_| PublicError::InternalServerError)?
        .to_string();

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

    let payment_id = PaymentId::from_uuid(payment.payment_id);
    log::set_payment_id(payment_id);
    log::set_payment_state(PaymentState::InboundCreated);

    app.db_client
        .upsert_payment(
            Payment {
                payment_id,
                payer_full_name: form.payer_full_name,
                payer_email: form.payer_email,
                payee_full_name: form.payee_full_name,
                payee_email: form.payee_email,
                amount: form.amount,
                security_question: form.security_question,
                security_answer,
                payment_statuses: PaymentStatuses {
                    inbound_created_at: Utc::now(),
                    inbound_authorized_at: None,
                    inbound_executed_at: None,
                    inbound_settled_at: None,
                    inbound_failed_at: None,
                },
                payout_data: None,
                refund_data: None,
            },
            0,
        )
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
