mod admin_home;
mod admin_login_;
mod admin_payment;

use actix_web::{cookie::Cookie, http::header, web, HttpRequest, HttpResponse};
use admin_home::admin_home_view;
use admin_login_::{admin_login, admin_login_form};
use admin_payment::admin_payment_view;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use leptos::view;

use crate::app::component::MyHtml;

pub fn admin_scope() -> actix_web::Scope {
    web::scope("/admin")
        .service(web::resource("").get(admin_login_form))
        .service(web::resource("/login").post(admin_login))
        .service(web::resource("/home").get(admin_home_view))
        .service(web::resource("/payment").get(admin_payment_view))
        .service(web::resource("/unauthorized").get(admin_unauthorized))
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

pub fn admin_flag(req: &HttpRequest) -> bool {
    // TODO: make a middleware to check admin cookie
    req.headers()
        .get(header::COOKIE)
        .and_then(|cookie_header| {
            cookie_header.to_str().ok().and_then(|cookie_str| {
                cookie_str
                    .split(';')
                    .map(|s| Cookie::parse(s.trim()).ok())
                    .find(|x| x.as_ref().map(|x| x.name() == "admin").unwrap_or(false))
            })
        })
        .flatten()
        .as_ref()
        .map(|admin_cookie| admin_cookie.value())
        .map(|auth_value| {
            let admin_hash = PasswordHash::new(auth_value).unwrap();
            Argon2::default()
                .verify_password(b"admin", &admin_hash)
                .map_or(false, |_| true)
        })
        .unwrap_or(false)
}

pub async fn admin_route_to_unauthorized() -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/admin/unauthorized"))
        .finish()
}
