use actix_web::{web, HttpResponse};
use leptos::{view, IntoView};
use serde::Deserialize;
use uuid::Uuid;

use crate::{app::component::MyHtml, AppContext};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    payment_id: Uuid,
}

pub async fn deposit(
    app: web::Data<AppContext>,
    query_params: web::Query<QueryParams>,
) -> HttpResponse {
    let payment = match app.db_client.get_payment(query_params.payment_id).await {
        Ok(payment) => payment,
        _ => return HttpResponse::ServiceUnavailable().body("ummm"),
    };

    let payment_id = payment.payment_id.to_string();
    let security_question = payment.security_question;

    let html = leptos::ssr::render_to_string(move || {
        view! {
            <MyHtml>
                <div class="container-sm text-light" >

                    <div class="row mt-3" >

                        <div class="col" />
                        <div class="col" >

                            <h1 class="text-center" >Deposit Payment</h1>

                            <form action="api/deposit_payment" method="post" >

                                <div class="input-group mb-3" >
                                    <input
                                        type="text"
                                        name="security_question"
                                        class="form-control"
                                            value={security_question}
                                        aria-label="securit question"
                                        aria-describedby="basic-addon1"
                                        disabled
                                    />
                                </div>

                                <div class="input-group mb-3" >
                                    <input
                                        type="text"
                                        name="security_answer"
                                        class="form-control"
                                        placeholder="security answer"
                                        aria-label="security answer"
                                        aria-describedby="basic-addon1"
                                    />
                                </div>

                                <input
                                    type="hidden"
                                    name="payment_id"
                                    value={payment_id}
                                />

                                <div class="input-group mb-3" >
                                    <input
                                        type="submit"
                                        class="form-control btn btn-success"
                                        value="DEPOSIT"
                                        aria-label="continue button"
                                        aria-describedby="basic-addon1"
                                    />

                                </div>

                            </form>

                        </div>
                        <div class="col" />

                    </div>

                </div>
            </MyHtml>
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html.as_str().to_string())
}
