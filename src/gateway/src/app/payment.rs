use actix_web::{HttpRequest, HttpResponse};
use leptos::{view, IntoView};

use crate::app::component::MyHtml;

pub async fn payment(_req: HttpRequest) -> HttpResponse {
    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container-sm text-light" >

                    <div class="row mt-3" >

                        <div class="col" />
                        <div class="col" >

                            <h1>Create Payment</h1>

                            <form action="api/create_payment" method="post" >

                                <div class="input-group mb-3" >
                                    <input
                                        type="text"
                                        name="payer_full_name"
                                        class="form-control"
                                        placeholder="benefactor name"
                                        aria-label="benefactor name"
                                        aria-describedby="basic-addon1"
                                    />
                                </div>

                                <div class="input-group mb-3" >
                                    <input
                                        type="email"
                                        name="payer_email"
                                        class="form-control"
                                        placeholder="benefactor@example.com"
                                        aria-label="benefactor email"
                                        aria-describedby="basic-addon1"
                                    />
                                </div>

                               <div class="input-group mb-3" >
                                    <input
                                        type="text"
                                        name="payee_full_name"
                                        class="form-control"
                                        placeholder="recipiant name"
                                        aria-label="recipiant name"
                                        aria-describedby="basic-addon1"
                                    />
                                </div>

                                <div class="input-group mb-3" >
                                    <input
                                        type="email"
                                        name="payee_email"
                                        class="form-control"
                                        placeholder="recipiant@example.com"
                                        aria-label="email"
                                        aria-describedby="basic-addon1"
                                    />
                                </div>

                                <div class="input-group mb-3" >
                                    <input
                                        type="text"
                                        name="amount"
                                        class="form-control"
                                        placeholder="amount"
                                        aria-label="amount"
                                        aria-describedby="basic-addon1"
                                    />
                                </div>

                                <div class="input-group mb-3" >
                                    <input
                                        type="text"
                                        name="security_question"
                                        class="form-control"
                                        placeholder="security question"
                                        aria-label="securit question"
                                        aria-describedby="basic-addon1"
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

                                <div class="input-group mb-3" >
                                    <input
                                        type="submit"
                                        class="form-control btn btn-success"
                                        value="CONTINUE"
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
