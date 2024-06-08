use actix_web::{HttpRequest, HttpResponse};
use leptos::view;

use crate::app::{
    component::MyHtml, payment_flow::PAYMENT_CREATE_PAGE, registration_flow::REGISTER_PAGE,
};

pub async fn home(_req: HttpRequest) -> HttpResponse {
    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container text-light text-center pt-4" >
                    <h1 class="text-center" >"Welcome to e-transfer"</h1>
                    <a class="btn btn-success" href={PAYMENT_CREATE_PAGE} >Move Money</a>
                    <a class="btn btn-success ms-1" href="/login" >Sign In</a>
                    <a class="btn btn-success ms-1" href={REGISTER_PAGE} >Register</a>
                </div>
            </MyHtml>
        }
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html.to_string())
}
