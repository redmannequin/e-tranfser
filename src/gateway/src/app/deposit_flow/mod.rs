mod z01_deposit_form;
mod z02_deposit_auth;
mod z03_tl_despoit_callback;
mod z04_depsoit_select_account;
mod z05_create_payout;
mod z06_deposit_status;

use actix_web::web;
use z01_deposit_form::deposit_form;
use z02_deposit_auth::deposit_auth;
use z03_tl_despoit_callback::tl_deposit_callback;
use z04_depsoit_select_account::deposit_select_account;
use z05_create_payout::create_payout;
use z06_deposit_status::{deposit_status, deposit_status_update};

pub const DESPOSIT_CREATE_PAGE: &str = "/app/deposit";
pub const DEPOSIT_AUTH_PAGE: &str = "/app/deposit/auth";
pub const DESPOSIT_SELECT_ACCOUNT_PAGE: &str = "/app/deposit/select_account";
pub const DESPOSIT_STATUS_PAGE: &str = "/app/deposit/status";
pub const DESPOSIT_STATUS_UPDATE_PAGE: &str = "/app/deposit/status_update";
pub const DEPOSIT_CREATE_PAYOUT: &str = "/app/deposit/create_payout";
#[allow(unused)]
pub const DESPOSIT_TL_CALLBACK_PAGE: &str = "/app/deposit/tl_callback";

pub const PAYOUT_COOKIE: &str = "payout_init";

pub fn deposit_scope() -> actix_web::Scope {
    web::scope("deposit")
        .service(web::resource("").get(deposit_form))
        .service(web::resource("auth").post(deposit_auth))
        .service(web::resource("tl_callback").post(tl_deposit_callback))
        .service(web::resource("select_account").get(deposit_select_account))
        .service(web::resource("create_payout").get(create_payout))
        .service(web::resource("status").get(deposit_status))
        .service(web::resource("status_update").get(deposit_status_update))
}
