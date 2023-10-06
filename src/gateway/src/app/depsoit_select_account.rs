use actix_web::{web, HttpResponse};
use futures::future::join_all;
use leptos::{component, view, CollectView, IntoView};
use serde::Deserialize;
use uuid::Uuid;

use crate::{app::component::MyHtml, truelayer::model::AccountBalance, AppContext, TlClient};

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

    let accounts: Vec<Account> = {
        let accounts = app.tl_client.get_accounts(&token).await.unwrap().results;
        join_all::<Vec<_>>(
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
        .zip(accounts.into_iter())
        .map(|(b, a)| Account {
            name: a.display_name,
            balance: b.current,
            iban: a.account_number.iban,
        })
        .collect()
    };

    let html = leptos::ssr::render_to_string(|| {
        view! {
            <MyHtml>
                <div class="container-sm form-signin w-100 m-auto" >
                    <h1 class="text-center" >"Select Account"</h1>
                    <AccountList accounts={accounts} />
                </div>
            </MyHtml>
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html.as_str().to_string())
}

struct Account {
    name: String,
    balance: f32,
    iban: String,
}

#[component]
fn account_list(accounts: Vec<Account>) -> impl IntoView {
    let accounts_view = accounts.into_iter().map(|a| view! {
        <a href="#" class="list-group-item list-group-item-action flex-column align-items-start" >
            <div class="d-flex w-100 justify-content-between" >
                <h5 class="mb-1" >{a.name}</h5>
                <small>{a.balance}</small>
            </div>
            <small>{a.iban}</small>
        </a>
    }).collect_view();

    return view! {
        <ul class="list-group list-group-flush">
            { accounts_view }
        </ul>
    };
}
