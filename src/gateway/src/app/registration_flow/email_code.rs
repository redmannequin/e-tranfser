use actix_web::{web, HttpResponse};
use domain::User;
use leptos::view;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    api::PublicError,
    app::component::{MyHtml, MyInput},
    AppContext,
};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    user_id: Uuid,
}

pub async fn email_code_confirm(
    app: web::Data<AppContext>,
    query_params: web::Query<QueryParams>,
) -> Result<HttpResponse, PublicError> {
    let email = match app.db_client.get_user::<User>(query_params.user_id).await? {
        Some((User::Registering { email, .. }, _)) => email,
        Some((User::Registered { .. }, _)) => {
            return Err(PublicError::InternalServerError);
        }
        None => {
            return Err(PublicError::InternalServerError);
        }
    };

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
                                value={email}
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
