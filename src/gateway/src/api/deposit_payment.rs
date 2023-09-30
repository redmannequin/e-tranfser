use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

use crate::AppContext;

use super::{deserialize_body, PublicError};

#[derive(Debug, Deserialize)]
struct Request {
    payment_id: Uuid,
    security_answer: String,
    iban: String,
}

#[post("/deposit_payment")]
pub async fn deposit_payment(
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
    let payment = app.db_client.get_payment(request.payment_id).await.unwrap();

    let is_vaild = {
        let parsed_hash = PasswordHash::new(&payment.security_answer).unwrap();
        Argon2::default()
            .verify_password(request.security_answer.as_bytes(), &parsed_hash)
            .map_or(false, |_| true)
            & !payment.deposited
    };

    if is_vaild {
        let payment_reference = "";

        app.db_client
            .set_payment_deposited(request.payment_id)
            .await?;

        let _ = app
            .tl_client
            .create_payout(
                &payment.full_name,
                &request.iban,
                payment.amount,
                payment_reference,
            )
            .await
            .map_err(|_| PublicError::InternalServerError)?;

        Ok(HttpResponse::Ok())
    } else {
        Ok(HttpResponse::Unauthorized())
    }
}
