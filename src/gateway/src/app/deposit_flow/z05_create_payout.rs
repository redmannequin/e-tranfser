use actix_session::Session;
use actix_web::{http::header, web, HttpResponse};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use base64::{
    engine::general_purpose::{STANDARD_NO_PAD, URL_SAFE},
    Engine,
};
use domain::{PaymentId, PaymentState};
use serde::Deserialize;

use crate::{
    api::PublicError,
    app::deposit_flow::{DESPOSIT_STATUS_PAGE, PAYOUT_COOKIE},
    log, AppContext,
};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    iban: String,
    payment_id: PaymentId,
}

pub async fn create_payout(
    app: web::Data<AppContext>,
    session: Session,
    query_params: web::Query<QueryParams>,
) -> Result<HttpResponse, PublicError> {
    let payment_id = query_params.payment_id;

    let salt_b64 = STANDARD_NO_PAD.encode(query_params.payment_id.as_uuid());
    let salt = SaltString::from_b64(salt_b64.as_str()).unwrap();
    let iban = String::from_utf8(URL_SAFE.decode(query_params.iban.as_str()).unwrap()).unwrap();
    let hash_iban = Argon2::default()
        .hash_password(iban.as_bytes(), &salt)
        .unwrap()
        .to_string();

    log::set_payment_id(payment_id);

    let valid_ibans: String = session.get(PAYOUT_COOKIE).unwrap().unwrap();
    if !valid_ibans.contains(hash_iban.as_str()) {
        todo!()
    }

    let payout_id = app
        .pm_client
        .create_payout(payment_id, iban, String::from("ref"))
        .await
        .unwrap();

    log::set_payout_id(payout_id);
    log::set_payment_state(PaymentState::PayoutCreated);

    let link = format!("{}?payment_id={}", DESPOSIT_STATUS_PAGE, payment_id);

    Ok(HttpResponse::SeeOther()
        .insert_header((header::LOCATION, link))
        .finish())
}
