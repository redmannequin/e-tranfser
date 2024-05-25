use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use anyhow::{ensure, Context};
use chrono::{DateTime, Utc};
use domain::{Payment, PaymentId, PaymentState, PayoutId, RefundId};
use serde::Deserialize;
use tracing::warn;
use truelayer_signing::Method;
use uuid::Uuid;

use crate::{api::deserialize_body, log, AppContext};

use super::PublicError;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TlWebhook {
    PaymentAuthorized {
        event_id: Uuid,
        event_version: u32,
        payment_id: PaymentId,
        authorized_at: DateTime<Utc>,
        payment_source: Option<PaymentSource>,
    },
    PaymentExecuted {
        event_id: Uuid,
        event_version: u32,
        payment_id: PaymentId,
        executed_at: DateTime<Utc>,
        payment_source: Option<PaymentSource>,
    },
    PaymentFailed {
        event_id: Uuid,
        event_version: u32,
        payment_id: PaymentId,
        failed_at: DateTime<Utc>,
        failed_stage: String,
        failure_reason: String,
        payment_source: Option<PaymentSource>,
    },
    PaymentSettled {
        event_id: Uuid,
        event_version: u32,
        payment_id: PaymentId,
        settled_at: DateTime<Utc>,
        payment_source: Option<PaymentSource>,
        user_id: String,
    },
    ExternalPaymentReceived {
        event_id: Uuid,
        event_version: u32,
        transaction_id: Uuid,
        currency: String,
        amount_in_minor: String,
        settled_at: DateTime<Utc>,
        merchant_account_id: String,
        remitter: Remitter,
    },
    PayoutExecuted {
        event_id: Uuid,
        event_version: u32,
        payout_id: PayoutId,
        executed_at: DateTime<Utc>,
    },
    PayoutFailed {
        event_id: Uuid,
        event_version: u32,
        payout_id: PayoutId,
        failed_at: DateTime<Utc>,
    },
    RefundExecuted {
        event_id: Uuid,
        event_version: u32,
        refund_id: RefundId,
        payment_id: Uuid,
        executed_at: DateTime<Utc>,
    },
    RefundFailed {
        event_id: Uuid,
        event_version: u32,
        refund_id: RefundId,
        payment_id: Uuid,
        failed_at: DateTime<Utc>,
    },
}

#[derive(Debug, Deserialize)]
pub struct PaymentSource {
    pub account_holder_name: String,
    pub account_identifiers: Vec<AccountIdentifier>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AccountIdentifier {
    Iban {
        iban: String,
    },
    SortCodeAccountNumber {
        sort_code: String,
        account_number: String,
    },
}

#[derive(Debug, Deserialize)]
pub struct Remitter {
    pub account_holder_name: String,
    pub account_identifiers: Vec<AccountIdentifier>,
}

#[post("/tl_webhook")]
pub async fn tl_webhook(
    app: web::Data<AppContext>,
    req: HttpRequest,
    body: String,
) -> Result<impl Responder, PublicError> {
    if let Err(err) = verify_hook(req, body.as_bytes()).await {
        warn!("{err}");
        return Ok(HttpResponse::Unauthorized());
    }

    let webhook: TlWebhook = deserialize_body(&body)?;

    match webhook {
        TlWebhook::PaymentAuthorized {
            payment_id,
            authorized_at,
            ..
        } => {
            log::set_payment_id(payment_id);
            log::set_payment_state(PaymentState::InboundAuthorized);

            let (mut payment, version) = app
                .db_client
                .get_payment::<Payment>(payment_id)
                .await?
                .ok_or(PublicError::Invalid(String::from("test")))?;

            payment.payment_statuses.inbound_authorized_at = Some(authorized_at);
            app.db_client.upsert_payment(payment, version + 1).await?;
        }
        TlWebhook::PaymentExecuted {
            payment_id,
            executed_at,
            ..
        } => {
            log::set_payment_id(payment_id);
            log::set_payment_state(PaymentState::InboundExecuted);

            let (mut payment, version) = app
                .db_client
                .get_payment::<Payment>(payment_id)
                .await?
                .ok_or(PublicError::Invalid(String::from("test")))?;

            payment.payment_statuses.inbound_executed_at = Some(executed_at);
            app.db_client.upsert_payment(payment, version + 1).await?;
        }
        TlWebhook::PaymentSettled {
            payment_id,
            settled_at,
            ..
        } => {
            log::set_payment_id(payment_id);
            log::set_payment_state(PaymentState::InboundSettled);

            let (mut payment, version) = app
                .db_client
                .get_payment::<Payment>(payment_id)
                .await?
                .ok_or(PublicError::Invalid(String::from("test")))?;

            payment.payment_statuses.inbound_settled_at = Some(settled_at);
            app.db_client.upsert_payment(payment, version + 1).await?;
        }
        TlWebhook::PaymentFailed {
            payment_id,
            failed_at,
            ..
        } => {
            log::set_payment_id(payment_id);
            log::set_payment_state(PaymentState::InboundFailed);

            let (mut payment, version) = app
                .db_client
                .get_payment::<Payment>(payment_id)
                .await?
                .ok_or(PublicError::Invalid(String::from("test")))?;

            payment.payment_statuses.inbound_failed_at = Some(failed_at);
            app.db_client.upsert_payment(payment, version + 1).await?;
        }
        _ => unimplemented!(),
    }
    Ok(HttpResponse::Ok())
}

async fn verify_hook(parts: HttpRequest, body: &[u8]) -> anyhow::Result<()> {
    let tl_signature = parts
        .headers()
        .get("Tl-Signature")
        .context("missing Tl-Signature headers")?
        .to_str()
        .context("invalid non-string Tl-Signature")?;

    let jku = truelayer_signing::extract_jws_header(tl_signature)?
        .jku
        .context("jku missing")?;

    // ensure jku is an expected TrueLayer url
    ensure!(
        jku == "https://webhooks.truelayer.com/.well-known/jwks"
            || jku == "https://webhooks.truelayer-sandbox.com/.well-known/jwks",
        "Unpermitted jku {jku}"
    );

    // fetch jwks (cached according to cache-control headers)
    let jwks = reqwest::Client::builder()
        .build()
        .unwrap()
        .get(jku.as_ref())
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;

    // verify signature using the jwks
    truelayer_signing::verify_with_jwks(&jwks)
        .method(Method::Post)
        .path(parts.path())
        .headers(
            parts
                .headers()
                .iter()
                .map(|(h, v)| (h.as_str(), v.as_bytes())),
        )
        .body(body)
        .build_verifier()
        .verify(tl_signature)?;

    Ok(())
}
