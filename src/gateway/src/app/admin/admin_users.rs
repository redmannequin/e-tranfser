use actix_web::{web, HttpResponse};
use domain::User;
use leptos::{component, view, CollectView, IntoView};

use crate::{app::component::MyHtml, AppContext};

pub async fn admin_users_view(app: web::Data<AppContext>) -> HttpResponse {
    let users = app.db_client.get_users::<User>(10, 0).await.unwrap();

    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container-sm w-50">
                    <h1 class="">Admin Users View</h1>
                    <UserListView users={users.iter().map(|(u, _)| u)} />
                </div>
            </MyHtml>
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html.to_string())
}

#[component]
fn user_list_view<'a, U>(users: U) -> impl IntoView
where
    U: Iterator<Item = &'a User> + 'a,
{
    let values = users
        .map(|user| {
            view! {
                <tr onclick={format!("window.location.href='/admin/user?user_id={}'", user.user_id())}>
                    <th scope="row">{user.user_id().to_string()}</th>
                    <td>{user.email().to_string()}</td>
                    <td>{user.first_name().to_string()}</td>
                    <td>{user.last_name().to_string()}</td>
                </tr>
            }
        })
        .collect_view();

    view! {
        <table class="table table-hover">
            <thead>
                <tr>
                    <th class="" scope="col">UserId</th>
                    <th class="" scope="col">Email</th>
                    <th class="" scope="col">Fistname</th>
                    <th class="" scope="col">Lastname</th>
                </tr>
            </thead>
            <tbody class="table-group-divider">
                { values }
            </tbody>
        </table>
    }
}
