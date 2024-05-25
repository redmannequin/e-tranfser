use actix_web::{web, HttpResponse};
use domain::Payment;
use leptos::{component, view, IntoView};
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
        .body(html.as_str().to_string())
}

#[component]
fn payment_view(payment: Payment) -> impl IntoView {
    let payment_id = payment.payment_id.to_string();
    let payment_state = payment.payment_state.as_str();

    view! {
        <table class="table">
            <thead>
                <tr>
                    <th scope="col">Field</th>
                    <th scope="col">Value</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <th scope="row">payment_id</th>
                    <td>{payment_id}</td>
                </tr>
                <tr>
                    <th scope="row">payment_state</th>
                    <td>{payment_state}</td>
                </tr>
            </tbody>
        </table>
    }
}
