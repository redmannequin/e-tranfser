use actix_web::{HttpRequest, HttpResponse};
use leptos::{view, IntoView};

use crate::app::component::MyHtml;

pub async fn not_found(_req: HttpRequest) -> HttpResponse {
    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container text-light" >
                    <h1>"Not Found"</h1>
                </div>
            </MyHtml>
        }
    });

    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body(html.as_str().to_string())
}
