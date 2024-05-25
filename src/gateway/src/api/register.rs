use actix_web::{post, web, HttpResponse, Responder};
use domain::{User, UserId};
use serde::Deserialize;
use tracing::instrument;

use crate::AppContext;

use super::PublicError;

#[derive(Debug, Deserialize)]
pub struct FormData {
    first_name: String,
    last_name: String,
    email: String,
}

#[post("/deposit_payment")]
pub async fn deposit_payment(
    app: web::Data<AppContext>,
    form: web::Form<FormData>,
) -> Result<impl Responder, PublicError> {
    execute(app, form.0).await
}

#[instrument(skip(app))]
async fn execute(
    app: web::Data<AppContext>,
    request: FormData,
) -> Result<impl Responder, PublicError> {
    app.db_client
        .upsert_user(
            User {
                user_id: UserId::new(),
                email: request.email,
                first_name: request.first_name,
                last_name: request.last_name,
            },
            0,
        )
        .await?;
    Ok(HttpResponse::Unauthorized())
}
