use actix_web::{HttpResponse};
use leptos::{view, IntoView};



use crate::app::component::MyHtml;

pub async fn payment_sent() -> HttpResponse {
    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container text-light" >
                    <h1>"Payment Sent!"</h1>
                </div>
            </MyHtml>
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html.as_str().to_string())
}
