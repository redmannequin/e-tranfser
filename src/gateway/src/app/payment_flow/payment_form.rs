use actix_web::{HttpRequest, HttpResponse};
use leptos::view;

use crate::app::component::{MyHtml, MyInput};

pub async fn payment_form(_req: HttpRequest) -> HttpResponse {
    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container-sm form-signin w-100 m-auto text-center" >
                    <form action="../api/create_payment" method="post" >
                        <h1 class="text-light mb-3 fw-normal">Create Payment</h1>
                        <MyInput input_type="text" name="payer_full_name" label="Benefactor Name" required=true/>
                        <MyInput input_type="email" name="payer_email" label="Benefactor Email" required=true/>
                        <MyInput input_type="text" name="payee_full_name" label="Recipiant Name" required=true/>
                        <MyInput input_type="email" name="payee_email" label="Recipiant Email" required=true/>
                        <MyInput input_type="text" name="amount" label="Amount" required=true/>
                        <MyInput input_type="text" name="security_question" label="Security Question" required=true/>
                        <MyInput input_type="text" name="security_answer" label="Security Answer" required=true/>
                        <div class="input-group mb-3" >
                            <input
                                type="submit"
                                class="form-control btn btn-success"
                                value="CONTINUE"
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
