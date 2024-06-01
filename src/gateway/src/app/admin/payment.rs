use actix_web::{web, HttpResponse};
use domain::Payment;
use leptos::{component, view, CollectView, IntoView};
use serde::Deserialize;
use uuid::Uuid;

use crate::{app::component::MyHtml, AppContext};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    payment_id: Uuid,
}

pub async fn admin_payment_view(
    app: web::Data<AppContext>,
    query_params: web::Query<QueryParams>,
) -> HttpResponse {
    let payment = app
        .db_client
        .get_payment::<Payment>(query_params.payment_id)
        .await
        .unwrap()
        .map(|(p, _)| p)
        .unwrap();

    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container-sm form-signin w-100 m-auto" >
                    <h1 class="text-center" >"Admin Payment View"</h1>
                    <PaymentView payment={payment} />
                </div>
            </MyHtml>
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html.to_string())
}

#[component]
fn payment_view(payment: Payment) -> impl IntoView {
    let feilds_and_values = [
        ("payment_id", payment.payment_id.to_string()),
        (
            "inbound_created_at",
            payment.payment_statuses.inbound_created_at.to_rfc3339(),
        ),
        (
            "inbound_authorized_at",
            payment
                .payment_statuses
                .inbound_authorized_at
                .map(|s| s.to_rfc3339())
                .unwrap_or_default(),
        ),
        (
            "inbound_executed_at",
            payment
                .payment_statuses
                .inbound_executed_at
                .map(|s| s.to_rfc3339())
                .unwrap_or_default(),
        ),
        (
            "inbound_settled_at",
            payment
                .payment_statuses
                .inbound_settled_at
                .map(|s| s.to_rfc3339())
                .unwrap_or_default(),
        ),
        (
            "inbound_failed_at",
            payment
                .payment_statuses
                .inbound_failed_at
                .map(|s| s.to_rfc3339())
                .unwrap_or_default(),
        ),
    ];

    let feilds_and_values = feilds_and_values
        .into_iter()
        .map(|(field, value)| {
            view! {
                <tr>
                    <th scope="row">{field}</th>
                    <td>{value}</td>
                </tr>
            }
        })
        .collect_view();

    view! {
        <table class="table">
            <thead>
                <tr>
                    <th scope="col">Field</th>
                    <th scope="col">Value</th>
                </tr>
            </thead>
            <tbody>
                { feilds_and_values }
            </tbody>
        </table>
    }
}
