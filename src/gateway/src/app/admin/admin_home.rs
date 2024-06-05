use actix_web::HttpResponse;
use leptos::view;

use crate::app::component::{MyHtml, MyInput};

pub async fn admin_home_view() -> HttpResponse {
    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container-sm form-signin w-100 m-auto" >
                    <h1 class="text-center" >"Admin View"</h1>

                    <form action="/admin/payment" method="get" >
                        <h2 class="text-light mb-3 fw-normal">Get Payment</h2>
                        <div class="form-floating mb-3" >
                            <MyInput input_type="text" name="payment_id" label="Payment ID" required=false/>
                            <div class="input-group mb-3" >
                                <input
                                    type="submit"
                                    class="form-control btn btn-success"
                                    value="Submit"
                                />
                            </div>
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
