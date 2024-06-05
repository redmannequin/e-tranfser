use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use domain::{User, UserId};
use leptos::view;
use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;
use tracing::instrument;

use crate::{
    api::PublicError,
    app::component::{MyHtml, MyInput},
    AppContext,
};

#[derive(Debug, Deserialize)]
pub struct FormData {
    first_name: String,
    last_name: String,
    email: String,
}

pub async fn register(
    app: web::Data<AppContext>,
    form: web::Form<FormData>,
) -> Result<impl Responder, PublicError> {
    execute(app, form.into_inner()).await
}

#[instrument(skip(app))]
async fn execute(
    app: web::Data<AppContext>,
    request: FormData,
) -> Result<impl Responder, PublicError> {
    let code = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
    app.db_client
        .upsert_user(
            User::Registering {
                user_id: UserId::new(),
                email: request.email.clone(),
                first_name: request.first_name,
                last_name: request.last_name,
                code,
                timestamp: Utc::now(),
            },
            0,
        )
        .await?;

    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container-sm form-signin w-100 m-auto text-center" >
                    <form>
                        <h1 class="text-light mb-3 fw-normal">Confirm Code</h1>

                        <div class="form-floating mb-3" >
                            <input
                                type="email"
                                readonly
                                class="form-control"
                                id="email"
                                value={request.email}
                            />
                            <label for="from">Email</label>
                        </div>

                        <MyInput input_type="text" name="email_code" label="Email Code" required=true/>

                        <div class="input-group mb-3" >
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

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html.to_string()))
}
