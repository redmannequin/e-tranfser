use actix_session::Session;
use actix_web::{web, HttpResponse};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use base64::{
    engine::general_purpose::{STANDARD_NO_PAD, URL_SAFE},
    Engine,
};
use futures::future::join_all;
use leptos::{component, view, CollectView, IntoView};
use serde::Deserialize;
use truelayer::model::AccountBalance;
use uuid::Uuid;

use crate::{
    app::{
        component::MyHtml,
        deposit_flow::{DEPOSIT_CREATE_PAYOUT, PAYOUT_COOKIE},
    },
    AppContext,
};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    #[allow(unused)]
    payment_id: Uuid,
    code: String,
}

pub async fn deposit_select_account(
    app: web::Data<AppContext>,
    session: Session,
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

    let salt_b64 = STANDARD_NO_PAD.encode(query_params.payment_id);
    let salt = SaltString::from_b64(salt_b64.as_str()).unwrap();
    let valid_ibans = accounts
        .iter()
        .map(|a| {
            Argon2::default()
                .hash_password(a.iban.as_bytes(), &salt)
                .unwrap()
                .to_string()
        })
        .collect::<Vec<_>>()
        .join(",");

    session.insert(PAYOUT_COOKIE, valid_ibans).unwrap();

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
        .body(html.to_string())
}

struct Account {
    name: String,
    balance: f32,
    iban: String,
}

#[component]
fn account_list(accounts: Vec<Account>) -> impl IntoView {
    let accounts_view = accounts.into_iter().map(|a| {
        let iban_b64 = URL_SAFE.encode(a.iban.as_str());
        let link = format!("{}?iban={}", DEPOSIT_CREATE_PAYOUT, iban_b64);
        view! {
            <a href={link} class="list-group-item list-group-item-action flex-column align-items-start">
                <div class="d-flex w-100 justify-content-between" >
                    <h5 class="mb-1" >{a.name}</h5>
                    <small>{a.balance}</small>
                </div>
                <small>{a.iban}</small>
            </a>
        }
    }).collect_view();

    view! {
        <ul class="list-group list-group-flush">
            { accounts_view }
        </ul>
    }
}
