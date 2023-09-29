use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

use crate::{db::CreatePayment, AppContext};

use super::{deserialize_body, PublicError};

#[derive(Debug, Deserialize)]
struct Request {
    full_name: String,
    email: String,
    amount: u32,
    security_question: String,
    security_answer: String,
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
    app.db_client
        .insert_payment(CreatePayment {
            payment_id: Uuid::new_v4(),
            full_name: request.full_name,
            email: request.email,
            amount: request.amount,
            security_question: request.security_question,
            security_answer: request.security_answer,
        })
        .await
        .unwrap();

    Ok(HttpResponse::Ok())
}
