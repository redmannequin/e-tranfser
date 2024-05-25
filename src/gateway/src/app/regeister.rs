use actix_web::{HttpRequest, HttpResponse};
use leptos::view;

use crate::app::component::{MyHtml, MyInput};

pub async fn register(_req: HttpRequest) -> HttpResponse {
    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container-sm form-signin w-100 m-auto text-center" >

                    <form action="api/register" method="post" >

                        <h1 class="text-light mb-3 fw-normal">Register</h1>

                        <MyInput input_type="text" name="user_first_name" label="First Name" />
                        <MyInput input_type="text" name="user_last_name" label="Last Name" />
                        <MyInput input_type="email" name="payer_email" label="Email" />

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

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html.as_str().to_string())
}

pub async fn register_confirm_code(_req: HttpRequest) -> HttpResponse {
    todo!()
}
