use actix_web::{web, HttpResponse};
use domain::{Payment, PaymentState};
use leptos::view;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    app::{component::MyHtml, deposit_flow::DESPOSIT_STATUS_UPDATE_PAGE},
    AppContext,
};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    payment_id: Uuid,
}

pub async fn deposit_status(query_params: web::Query<QueryParams>) -> HttpResponse {
    let deposit_status_link = format!(
        "{}?payment_id={}",
        DESPOSIT_STATUS_UPDATE_PAGE, query_params.payment_id
    );

    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container-sm form-signin w-100 m-auto text-center" >
                    <h1>"Despoit Created!"</h1>
                    <div hx-get={deposit_status_link} hx-trigger="every 600ms">
                        <p>loading status...</p>
                    </div>
                </div>
            </MyHtml>
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html.to_string())
}

pub async fn deposit_status_update(
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
        PaymentState::OutboundCreated => "Payment Deposit Created...",
        PaymentState::OutboundAuthorized => "Payment Deposit Authorized...",
        PaymentState::OutboundExecuted => "Payment Deposit Executed",
        PaymentState::OutboundFailed => "Payment Deposit Failed...",
        _ => return Err(actix_web::error::ErrorInternalServerError("ummm...")),
    };

    let html = leptos::ssr::render_to_string(move || view! {<p>{status}</p>});
    let mut res = HttpResponse::Ok();

    // stop polling when payment reaches the states below via htmx header
    if matches!(
        payment.state(),
        PaymentState::OutboundFailed | PaymentState::OutboundExecuted
    ) {
        res.append_header(("HX-Trigger", "done"));
    }

    Ok(res
        .content_type("text/html; charset=utf-8")
        .body(html.to_string()))
}
