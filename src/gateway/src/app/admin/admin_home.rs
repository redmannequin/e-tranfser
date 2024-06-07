use actix_web::HttpResponse;
use leptos::view;

use crate::app::component::MyHtml;

pub async fn admin_home_view() -> HttpResponse {
    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container text-light text-center pt-4">
                    <h1 class="text-center" >"Admin View"</h1>

                    <a class="btn btn-success ms-1" href="/admin/payments" >Payments</a>
                    <a class="btn btn-success ms-1" href="/admin/users" >Users</a>

                </div>
            </MyHtml>
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html.to_string())
}
