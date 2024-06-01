use actix_web::{
    cookie::{time::Duration, Cookie},
    http::header,
    web, HttpResponse,
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use leptos::view;
use serde::Deserialize;

use crate::app::component::{MyHtml, MyInput};

use super::admin_route_to_unauthorized;

pub async fn admin_login_form() -> HttpResponse {
    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container-sm form-signin w-100 m-auto text-center" >
                    <form action="/admin/login" method="post" >
                        <h1 class="text-light mb-3 fw-normal">Admin Login</h1>
                        <MyInput input_type="password" name="admin_password" label="Admin Password"/>
                    </form>
                </div>
            </MyHtml>
        }
    });

    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body(html.to_string())
}

#[derive(Debug, Deserialize)]
pub struct FormData {
    admin_password: String,
}

pub async fn admin_login(form: web::Form<FormData>) -> HttpResponse {
    if form.admin_password == "admin" {
        let salt = SaltString::from_b64("ZXRyYW5zZmVy").unwrap();
        let admin_hash = Argon2::default()
            .hash_password(b"admin", &salt)
            .unwrap()
            .to_string();

        let cookie = Cookie::build("admin", &admin_hash)
            .path("/admin")
            .http_only(true)
            .secure(true)
            .max_age(Duration::minutes(5))
            .finish()
            .to_string();

        HttpResponse::SeeOther()
            .insert_header((header::SET_COOKIE, cookie))
            .insert_header((header::LOCATION, "/admin/home"))
            .finish()
    } else {
        admin_route_to_unauthorized().await
    }
}
