use actix_web::{web, HttpResponse};
use domain::User;
use leptos::{component, view, CollectView, IntoView};
use serde::Deserialize;
use uuid::Uuid;

use crate::{app::component::MyHtml, AppContext};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    user_id: Uuid,
}

pub async fn admin_user_view(
    app: web::Data<AppContext>,
    query_params: web::Query<QueryParams>,
) -> HttpResponse {
    let user = app
        .db_client
        .get_user::<User>(query_params.user_id)
        .await
        .unwrap()
        .map(|(u, _)| u)
        .unwrap();

    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container-sm w-50" >
                    <h1 class="">Admin User View</h1>
                    <UserView user={user} />
                </div>
            </MyHtml>
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html.to_string())
}

#[component]
fn user_view(user: User) -> impl IntoView {
    let feilds_and_values = [
        ("user_id", user.user_id().to_string()),
        ("email", user.email().to_string()),
        ("firstname", user.first_name().to_string()),
        ("lastname", user.last_name().to_string()),
        (
            "code",
            user.registration_code()
                .map(|(c, _)| c.to_string())
                .unwrap_or_default(),
        ),
        (
            "code timestamp",
            user.registration_code()
                .map(|(_, t)| t.to_rfc3339())
                .unwrap_or_default(),
        ),
    ];

    let feilds_and_values = feilds_and_values
        .into_iter()
        .map(|(field, value)| {
            view! {
                <tr>
                    <th scope="row">{field}</th>
                    <td>{value}</td>
                </tr>
            }
        })
        .collect_view();

    view! {
        <table class="table">
            <thead>
                <tr>
                    <th scope="col">Field</th>
                    <th scope="col">Value</th>
                </tr>
            </thead>
            <tbody class="table-group-divider">
                { feilds_and_values }
            </tbody>
        </table>
    }
}
