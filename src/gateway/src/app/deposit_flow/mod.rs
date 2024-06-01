mod deposit_form;
mod deposit_status;
mod depsoit_select_account;
mod tl_despoit_callback;

use actix_web::web;
use deposit_form::deposit_form;
use deposit_status::{deposit_status, deposit_status_update};
use depsoit_select_account::deposit_select_account;
use tl_despoit_callback::tl_deposit_callback;

pub const DESPOSIT_CREATE_PAGE: &str = "/deposit";
pub const DESPOSIT_STATUS_PAGE: &str = "/deposit/status";
pub const DESPOSIT_STATUS_UPDATE_PAGE: &str = "/deposit/status_update";
#[allow(unused)]
pub const DESPOSIT_TL_CALLBACK_PAGE: &str = "/deposit/tl_callback";

pub fn deposit_scope() -> actix_web::Scope {
    web::scope("/deposit")
        .service(web::resource("").to(deposit_form))
        .service(web::resource("/select_account").to(deposit_select_account))
        .service(web::resource("/status").to(deposit_status))
        .service(web::resource("/status_update").to(deposit_status_update))
        .service(web::resource("/tl_callback").to(tl_deposit_callback))
}
