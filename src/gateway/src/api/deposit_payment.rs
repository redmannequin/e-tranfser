use actix_web::{http::header, post, web, HttpResponse, Responder};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use domain::{Payment, PaymentState};
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

use crate::AppContext;

use super::PublicError;

#[derive(Debug, Deserialize)]
pub struct FormData {
    payment_id: Uuid,
    security_answer: String,
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
    let payment = app
        .db_client
        .get_payment::<Payment>(request.payment_id)
        .await?
        .map(|(p, _)| p)
        .ok_or(PublicError::Invalid(String::from("test")))?;

    let is_vaild = {
        let parsed_hash = PasswordHash::new(&payment.security_answer).unwrap();
        Argon2::default()
            .verify_password(request.security_answer.as_bytes(), &parsed_hash)
            .map_or(false, |_| true)
            && (payment.payment_state as u8) < (PaymentState::OutboundCreated as u8)
    };

    if is_vaild {
        let link = UriBuilder::new(&format!(
            "https://auth.{}/",
            app.tl_client.enviornment.uri()
        ))
        .add_param("response_type", "code")
        .add_param("client_id", &app.tl_client.client_id)
        .add_param("scope", "info%20accounts%20balance")
        .add_param("redirect_uri", &app.tl_client.data_redirect_uri)
        .add_param("providers", "uk-cs-mock%20uk-ob-all%20uk-oauth-all")
        .add_param("state", &payment.payment_id.to_string())
        .build();
        Ok(HttpResponse::SeeOther()
            .insert_header((header::LOCATION, link))
            .take())
    } else {
        Ok(HttpResponse::Unauthorized())
    }
}

struct UriBuilder<'a> {
    path: &'a str,
    params: String,
}

impl<'a> UriBuilder<'a> {
    pub fn new(path: &'a str) -> Self {
        Self {
            path,
            params: String::new(),
        }
    }

    pub fn add_param(mut self, name: &str, value: &str) -> Self {
        if !self.params.is_empty() {
            self.params.push_str(&format!("&{}={}", name, value));
        } else {
            self.params.push_str(&format!("?{}={}", name, value));
        }
        self
    }

    pub fn build(self) -> String {
        format!("{}{}", self.path, self.params)
    }
}
