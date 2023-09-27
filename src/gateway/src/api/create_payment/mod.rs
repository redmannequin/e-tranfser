use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

use crate::AppContext;

use super::{deserialize_body, PublicError};

#[derive(Debug, Deserialize)]
struct Request {
    full_name: String,
    email: String,
    amount: String,
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
    execute(request).await
}

async fn execute(_request: Request) -> Result<impl Responder, PublicError> {
    // TODD: create payment
    Ok(HttpResponse::Ok())
}
