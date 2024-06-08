mod admin_home;
mod admin_login_;
mod admin_payment;
mod admin_payments;
mod admin_user;
mod admin_users;
mod auth;

use actix_web::{http::header, web, HttpResponse};
use admin_home::admin_home_view;
use admin_login_::{admin_login, admin_login_form};
use admin_payment::admin_payment_view;
use admin_payments::admin_payments_view;
use admin_user::admin_user_view;
use admin_users::admin_users_view;
use auth::AdminAuth;
use leptos::view;

use crate::app::component::MyHtml;

pub fn admin_scope() -> actix_web::Scope {
    web::scope("admin")
        .service(
            web::resource("login")
                .get(admin_login_form)
                .post(admin_login),
        )
        .service(web::resource("unauthorized").get(admin_unauthorized))
        .service(
            web::scope("")
                .wrap(AdminAuth)
                .service(web::resource("home").get(admin_home_view))
                .service(web::resource("payment").get(admin_payment_view))
                .service(web::resource("payments").get(admin_payments_view))
                .service(web::resource("user").get(admin_user_view))
                .service(web::resource("users").get(admin_users_view)),
        )
        .default_service(web::to(admin_route_to_unauthorized))
}

pub async fn admin_unauthorized() -> HttpResponse {
    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container-sm form-signin w-100 m-auto" >
                    <h1 class="text-center" >"unauthorized"</h1>
                </div>
            </MyHtml>
        }
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html.to_string())
}

pub async fn admin_route_to_unauthorized() -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/admin/unauthorized"))
        .finish()
}
