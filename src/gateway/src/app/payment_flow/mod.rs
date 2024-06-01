mod payment_form;
mod payment_status;
mod tl_payment_callback;

use actix_web::web;
use payment_form::payment_form;
use payment_status::{payment_status, payment_status_update};
use tl_payment_callback::tl_payment_callback;

pub const PAYMENT_CREATE_PAGE: &str = "/payment";
pub const PAYMENT_STATUS_PAGE: &str = "/payment/status";
pub const PAYMENT_STATUS_UPDATE_PAGE: &str = "/payment/status_update";
#[allow(unused)]
pub const PAYMENT_TL_CALLBACK_PAGE: &str = "/payment/tl_callback";

pub fn payment_scope() -> actix_web::Scope {
    web::scope("/payment")
        .service(web::resource("").to(payment_form))
        .service(web::resource("/status").to(payment_status))
        .service(web::resource("/status_update").to(payment_status_update))
        .service(web::resource("/tl_callback").to(tl_payment_callback))
}
