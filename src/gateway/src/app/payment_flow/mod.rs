mod z01_payment_form;
mod z02_create_payment;
mod z03_tl_payment_callback;
mod z04_payment_status;

use actix_web::web;
use concat_const::concat;
use z01_payment_form::{payment_form, validation::validation_scope};
use z02_create_payment::create_payment;
use z03_tl_payment_callback::tl_payment_callback;
use z04_payment_status::{payment_status, payment_status_update};

use crate::app::APP_ROOT;

pub const PAYMENT_ROOT: &str = "/payment";

pub const PAYMENT_FORM_PAGE: &str = concat!(APP_ROOT, PAYMENT_ROOT);
pub const PAYMENT_CREATE_PAGE: &str = concat!(APP_ROOT, PAYMENT_ROOT, "/create_payout");
pub const PAYMENT_STATUS_PAGE: &str = concat!(APP_ROOT, PAYMENT_ROOT, "/status");
pub const PAYMENT_STATUS_UPDATE_PAGE: &str = concat!(APP_ROOT, PAYMENT_ROOT, "/status_update");
#[allow(unused)]
pub const PAYMENT_TL_CALLBACK_PAGE: &str = concat!(APP_ROOT, PAYMENT_ROOT, "/tl_callback");

pub fn payment_scope() -> actix_web::Scope {
    web::scope("payment")
        .service(web::resource("").get(payment_form))
        .service(web::resource("create_payout").to(create_payment))
        .service(web::resource("status").get(payment_status))
        .service(web::resource("status_update").get(payment_status_update))
        .service(web::resource("tl_callback").to(tl_payment_callback))
        .service(validation_scope())
}
