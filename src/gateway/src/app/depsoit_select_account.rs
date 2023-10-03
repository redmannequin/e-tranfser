use actix_web::{web, HttpResponse};
use futures::future::join_all;
use leptos::{view, CollectView, IntoView};
use serde::Deserialize;
use uuid::Uuid;

use crate::{app::component::MyHtml, truelayer::model::AccountBalance, AppContext};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    payment_id: Uuid,
    code: String,
}

pub async fn deposit_select_account(
    app: web::Data<AppContext>,
    query_params: web::Query<QueryParams>,
) -> HttpResponse {
    let token = app
        .tl_client
        .auth_data(&query_params.code)
        .await
        .unwrap()
        .access_token;

    let accounts = app.tl_client.get_accounts(&token).await.unwrap().results;

    let account_balances: Vec<AccountBalance> = join_all::<Vec<_>>(
        accounts
            .iter()
            .map(|a| async {
                app.tl_client
                    .get_account_balance(&token, a.account_id.clone())
                    .await
            })
            .collect(),
    )
    .await
    .into_iter()
    .map(|a: Result<AccountBalance, _>| a.unwrap())
    .collect();

    let accounts_view = accounts
        .into_iter()
        .zip(account_balances.into_iter())
        .map(|(a, b)| {
            view! {
                <p class="btn btn-success" >
                    <p>Name: {a.display_name}</p>
                    <p>Balance: {b.current}</p>
                    <p>Iban: {a.account_number.iban}</p>
                </p>
            }
        })
        .collect_view();

    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container-sm form-signin w-100 m-auto text-center" >
                    <h1>"Select Account!"</h1>
                    {accounts_view}
                </div>
            </MyHtml>
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html.as_str().to_string())
}
