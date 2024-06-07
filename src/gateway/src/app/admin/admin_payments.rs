use actix_web::{web, HttpResponse};
use domain::Payment;
use leptos::{component, view, CollectView, IntoView};

use crate::{app::component::MyHtml, AppContext};

pub async fn admin_payments_view(app: web::Data<AppContext>) -> HttpResponse {
    let payments = app.db_client.get_payments::<Payment>(10, 0).await.unwrap();

    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container-sm w-50">
                    <h1 class="">Admin Payments View</h1>
                    <PaymentListView payments={payments.iter().map(|(p, _)| p)} />
                </div>
            </MyHtml>
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html.to_string())
}

#[component]
fn payment_list_view<'a, U>(payments: U) -> impl IntoView
where
    U: Iterator<Item = &'a Payment> + 'a,
{
    let values = payments
        .map(|payment| {
            view! {
                <tr onclick={format!("window.location.href='/admin/payment?payment_id={}'", payment.payment_id)}>
                    <th scope="row">{payment.payment_id.to_string()}</th>
                    <td>{payment.payer_email.clone()}</td>
                    <td>{payment.payee_email.clone()}</td>
                    <td>{payment.amount}</td>
                    <td>{payment.state().as_str()}</td>
                </tr>
            }
        })
        .collect_view();

    view! {
        <table class="table table-hover">
            <thead>
                <tr>
                    <th class="" scope="col">PaymentId</th>
                    <th class="" scope="col">Payer Email</th>
                    <th class="" scope="col">Payee Email</th>
                    <th class="" scope="col">Amount</th>
                    <th class="" scope="col">Status</th>
                </tr>
            </thead>
            <tbody class="table-group-divider">
                { values }
            </tbody>
        </table>
    }
}
