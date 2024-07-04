use actix_web::{http::StatusCode, web, HttpResponse};
use domain::{Payment, PaymentState};
use leptos::view;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    app::{
        component::MyHtml, deposit_flow::DESPOSIT_CREATE_PAGE,
        payment_flow::PAYMENT_STATUS_UPDATE_PAGE,
    },
    AppContext,
};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    payment_id: Uuid,
}

pub async fn payment_status(query_params: web::Query<QueryParams>) -> HttpResponse {
    let deposit_link = format!(
        "{}?payment_id={}",
        DESPOSIT_CREATE_PAGE, query_params.payment_id
    );
    let payment_status_link = format!(
        "{}?payment_id={}",
        PAYMENT_STATUS_UPDATE_PAGE, query_params.payment_id
    );

    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container-sm form-signin w-100 m-auto text-center" >
                    <h1>"Payment Sent!"</h1>
                    <div hx-get={payment_status_link} hx-trigger="every 600ms">
                        <p>loading status...</p>
                    </div>
                    <a class="btn btn-success" href={deposit_link} >Deposit</a>
                </div>
            </MyHtml>
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html.to_string())
}

pub async fn payment_status_update(
    app: web::Data<AppContext>,
    query_params: web::Query<QueryParams>,
) -> Result<HttpResponse, actix_web::Error> {
    let (payment, _) = app
        .db_client
        .get_payment::<Payment>(query_params.payment_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
        .ok_or(actix_web::error::ErrorNotFound(format!(
            "PaymentId: {}",
            query_params.payment_id
        )))?;

    let status = match payment.state() {
        PaymentState::InboundCreated => "Payment Created...",
        PaymentState::InboundAuthorized => "Payment Authorized...",
        PaymentState::InboundExecuted => "Payment Executed...",
        PaymentState::InboundSettled => "Payment Settled Into Landing Account",
        PaymentState::InboundFailed => "Payment Failed",
        PaymentState::PayoutRegistered => "Payment Deposit Init...",
        PaymentState::PayoutCreated => "Payment Deposit Created...",
        PaymentState::PayoutExecuted => "Payment Deposit Executed",
        PaymentState::PayoutFailed => "Payment Deposit Failed...",
        PaymentState::RefundCreated => "Payment Refund Created...",
        PaymentState::RefundExecuted => "Payment Refunded",
        PaymentState::RefundFailed => "Payment Refund Failed",
    };

    let html = leptos::ssr::render_to_string(move || view! {<p>{status}</p>});
    let mut res = HttpResponse::Ok();

    // stop polling when payment reaches the states below
    if matches!(
        payment.state(),
        PaymentState::InboundFailed
            | PaymentState::RefundFailed
            | PaymentState::PayoutFailed
            | PaymentState::RefundExecuted
            | PaymentState::PayoutExecuted
            | PaymentState::InboundSettled
    ) {
        res.status(StatusCode::from_u16(286).unwrap());
    }

    Ok(res
        .content_type("text/html; charset=utf-8")
        .body(html.to_string()))
}
