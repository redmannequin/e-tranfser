use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use super::{deserialize_body, AppContext, PublicError};

#[derive(Debug, Deserialize)]
struct Request {
    grant_type: String,
    client_id: String,
    client_secret: String,
    scope: String,
}

#[derive(Debug, Serialize)]
struct Response {
    access_token: String,
    expires_in: u64,
    refresh_token: Option<String>,
    token_type: String,
}

#[post("/connect/token")]
pub async fn auth(
    app: web::Data<AppContext>,
    _request: HttpRequest,
    body: String,
) -> Result<impl Responder, PublicError> {
    let request = deserialize_body(&body)?;
    execute(app, request).await
}

async fn execute(
    _app: web::Data<AppContext>,
    request: Request,
) -> Result<impl Responder, PublicError> {
    Ok(HttpResponse::Ok().json(Response {
        access_token: request.client_id,
        expires_in: 0,
        refresh_token: None,
        token_type: "Bearer".into(),
    }))
}
