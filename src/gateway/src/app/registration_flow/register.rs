use actix_web::{http::header, web, HttpResponse, Responder};
use chrono::Utc;
use domain::{User, UserId};
use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;
use tracing::instrument;

use crate::{api::PublicError, app::registration_flow::REGISTER_EMAIL_CODE_PAGE, AppContext};

#[derive(Debug, Deserialize)]
pub struct FormData {
    first_name: String,
    last_name: String,
    email: String,
}

pub async fn register(
    app: web::Data<AppContext>,
    form: web::Form<FormData>,
) -> Result<impl Responder, PublicError> {
    execute(app, form.into_inner()).await
}

#[instrument(skip(app))]
async fn execute(
    app: web::Data<AppContext>,
    request: FormData,
) -> Result<impl Responder, PublicError> {
    let code = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);

    let (user, user_version) = match app.db_client.get_user_by_email(&request.email).await? {
        Some((User::Registered { .. }, _)) => {
            return Err(PublicError::InternalServerError);
        }
        Some((User::Registering { user_id, email, .. }, v)) => (
            User::Registering {
                user_id,
                email,
                first_name: request.first_name,
                last_name: request.last_name,
                code,
                timestamp: Utc::now(),
            },
            v + 1,
        ),
        None => (
            User::Registering {
                user_id: UserId::new(),
                email: request.email.clone(),
                first_name: request.first_name,
                last_name: request.last_name,
                code,
                timestamp: Utc::now(),
            },
            0,
        ),
    };

    let link = format!("{}?user_id={}", REGISTER_EMAIL_CODE_PAGE, user.user_id());
    app.db_client.upsert_user(user, user_version).await?;

    Ok(HttpResponse::SeeOther()
        .insert_header((header::LOCATION, link))
        .finish())
}
