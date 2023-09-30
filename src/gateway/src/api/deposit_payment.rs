use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

use crate::{db::CreatePayment, AppContext};

use super::{deserialize_body, PublicError};

#[derive(Debug, Deserialize)]
struct Request {
    payment_id: Uuid,
    email: String,
    security_answer: String,
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

    let iban = "";
    let payment_reference = "";

    app.tl_client
        .create_payout(&payment.full_name, iban, payment.amount, payment_reference)
        .await;

    Ok(HttpResponse::Ok())
}
