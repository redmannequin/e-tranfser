mod create_payout;
mod deposit_form;
mod deposit_status;
mod depsoit_select_account;
mod tl_despoit_callback;

use actix_web::web;
use create_payout::create_payout;
use deposit_form::deposit_form;
use deposit_status::{deposit_status, deposit_status_update};
use depsoit_select_account::deposit_select_account;
use tl_despoit_callback::tl_deposit_callback;

pub const DESPOSIT_CREATE_PAGE: &str = "/app/deposit";
pub const DESPOSIT_SELECT_ACCOUNT_PAGE: &str = "/app/deposit/select_account";
pub const DESPOSIT_STATUS_PAGE: &str = "/app/deposit/status";
pub const DESPOSIT_STATUS_UPDATE_PAGE: &str = "/app/deposit/status_update";
pub const DEPOSIT_CREATE_PAYOUT: &str = "/app/deposit/create_payout";
#[allow(unused)]
pub const DESPOSIT_TL_CALLBACK_PAGE: &str = "/app/deposit/tl_callback";

pub const PAYOUT_COOKIE: &str = "payout_init";

pub fn deposit_scope() -> actix_web::Scope {
    web::scope("deposit")
        .service(web::resource("").to(deposit_form))
        .service(web::resource("create_payout").post(create_payout))
        .service(web::resource("select_account").to(deposit_select_account))
        .service(web::resource("status").to(deposit_status))
        .service(web::resource("status_update").to(deposit_status_update))
        .service(web::resource("tl_callback").to(tl_deposit_callback))
}
