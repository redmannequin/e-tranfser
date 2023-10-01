use actix_web::{HttpRequest, HttpResponse};
use leptos::{view, IntoView};

use crate::app::component::MyHtml;

pub async fn home(_req: HttpRequest) -> HttpResponse {
    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container text-light text-center pt-4" >
                    <h1 class="text-center" >"Welcome to e-transfer"</h1>
                    <a class="btn btn-success" href="/payment" >Move Money</a>
                    <a class="btn btn-success ms-1" href="/login" >Sgin In</a>
                </div>
            </MyHtml>
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html.as_str().to_string())
}
