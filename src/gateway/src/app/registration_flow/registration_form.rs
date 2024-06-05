use actix_web::{web, HttpResponse};
use domain::UserState;
use leptos::{component, view, IntoView};
use serde::Deserialize;

use crate::{
    app::component::{MyHtml, MyInput},
    AppContext,
};

pub async fn registration_form() -> HttpResponse {
    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container-sm form-signin w-100 m-auto text-center">
                    <form action="/register" method="post">
                        <h1 class="text-light mb-3 fw-normal">Register</h1>
                        <MyInput input_type="text" name="first_name" label="First Name" required=true/>
                        <MyInput input_type="text" name="last_name" label="Last Name" required=true/>
                        <EmailInput email={String::from("")} registered=false check=false/>
                        <div class="input-group mb-3">
                            <input
                                type="submit"
                                class="form-control btn btn-success"
                                value="REGISTER"
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

#[derive(Debug, Deserialize)]
pub struct FormData {
    email: String,
}

pub async fn check_email(app: web::Data<AppContext>, form: web::Form<FormData>) -> HttpResponse {
    let email = form.0.email;
    let registered = app
        .db_client
        .get_user_by_email::<domain::User>(&email)
        .await
        .unwrap()
        .map(|(u, _)| u.state() == UserState::Registered)
        .unwrap_or_default();

    let html = leptos::ssr::render_to_string(move || {
        view! {
            <EmailInput email={email} registered={registered} check=true/>
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html.to_string())
}

#[component]
pub fn email_input(email: String, registered: bool, check: bool) -> impl IntoView {
    let (class, err_msg) = match (
        check,
        registered,
        email_address::EmailAddress::is_valid(&email),
    ) {
        (true, true, _) => (
            "form-control is-invalid",
            "That email is already taken.  Please enter another email...",
        ),
        (true, _, false) => (
            "form-control is-invalid",
            "Please enter a valid email address...",
        ),
        (false, _, _) => ("form-control", ""),
        (_, _, _) => ("form-control is-valid", ""),
    };

    view! {
        <div class="form-floating mb-3 has-validation" hx-target="this" hx-swap="outerHTML">
            <input
                class={class}
                type="email"
                id="email"
                name="email"
                data-1p-ignore
                hx-post="/register/check_email"
                value={email}
                required=true
            />
            <label for="user_email">Email Address</label>
            <div class="invalid-feedback">
                {err_msg}
            </div>
        </div>
    }
}
