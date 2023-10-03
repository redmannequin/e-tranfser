use actix_web::{web, HttpResponse};
use leptos::{view, IntoView};
use serde::Deserialize;
use uuid::Uuid;

use crate::app::component::MyHtml;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    payment_id: Uuid,
}

pub async fn payment_sent(query_params: web::Query<QueryParams>) -> HttpResponse {
    let link = format!("/deposit?payment_id={}", query_params.payment_id);
    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container-sm form-signin w-100 m-auto text-center" >
                    <h1>"Payment Sent!"</h1>
                    <a class="btn btn-success" href={link} >Deposit</a>
                </div>
            </MyHtml>
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html.as_str().to_string())
}
