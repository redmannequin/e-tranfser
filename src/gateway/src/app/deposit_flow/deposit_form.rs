use actix_web::{web, HttpResponse};
use domain::Payment;
use leptos::view;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    app::component::{MyHtml, MyInput},
    AppContext,
};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    payment_id: Uuid,
}

pub async fn deposit_form(
    app: web::Data<AppContext>,
    query_params: web::Query<QueryParams>,
) -> HttpResponse {
    let payment = match app
        .db_client
        .get_payment::<Payment>(query_params.payment_id)
        .await
    {
        Ok(Some((payment, _))) => payment,
        _ => return HttpResponse::ServiceUnavailable().body("ummm"),
    };

    let from = payment.payer_full_name;
    let payment_id = payment.payment_id.to_string();
    let amount = payment.amount;
    let security_question = payment.security_question;

    let html = leptos::ssr::render_to_string(move || {
        view! {
            <MyHtml>
                <div class="container-sm form-signin w-100 m-auto text-center" >
                    <form action="../api/deposit_payment" method="post" >

                        <h1 class="text-light mb-3 fw-normal">Deposit Payment</h1>

                        <div class="form-floating mb-3" >
                            <input
                                type="text"
                                readonly
                                class="form-control"
                                id="from"
                                value={from}
                            />
                            <label for="from">From</label>
                        </div>

                        <div class="form-floating mb-3" >
                            <input
                                type="number"
                                readonly
                                class="form-control"
                                id="amount_test"
                                value={amount}
                            />
                            <label for="amount_test">Amount</label>
                        </div>

                        <div class="form-floating mb-3" >
                            <input
                                type="text"
                                readonly
                                class="form-control"
                                id="security_question"
                                value={security_question}
                            />
                            <label for="security_question">Security Question</label>
                        </div>

                        <MyInput input_type="text" name="security_answer" label="Security Answer" required=true/>

                        <input
                            type="hidden"
                            name="payment_id"
                            value={payment_id}
                        />

                        <div class="input-group mb-3">
                            <input
                                type="submit"
                                class="form-control btn btn-success"
                                value="DEPOSIT"
                            />
                        </div>

                    </form>

                </div>
            </MyHtml>
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html.to_string())
}
