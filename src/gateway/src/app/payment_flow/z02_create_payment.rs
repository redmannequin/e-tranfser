use actix_web::{http::header, web, HttpResponse};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use contracts::payment_manager::Payment;
use domain::PaymentState;
use serde::Deserialize;

use crate::{api::PublicError, log, AppContext};

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

    let (payment_id, resource_token) = app
        .pm_client
        .create_deposit(Payment {
            payer_full_name: form.payer_full_name,
            payer_email: form.payer_email,
            payee_full_name: form.payee_full_name,
            payee_email: form.payee_email,
            amount: form.amount,
            reference: String::from("todo"),
            security_question: form.security_question,
            security_answer,
        })
        .await
        .map_err(|_| PublicError::InternalServerError)?;

    log::set_payment_id(payment_id);
    log::set_payment_state(PaymentState::InboundCreated);

    let auth_link = format!(
        "https://payment.truelayer-sandbox.com/payments#payment_id={}&resource_token={}&return_uri={}", 
        payment_id,
        resource_token,
        app.tl_client.return_uri()
    );

    Ok(HttpResponse::SeeOther()
        .insert_header((header::LOCATION, auth_link))
        .finish())
}
