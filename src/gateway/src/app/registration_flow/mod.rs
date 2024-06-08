mod email_code;
mod register;
mod registration_form;

use actix_web::web;
use email_code::email_code_confirm;
use register::register;
use registration_form::{check_email, registration_form};

pub const REGISTER_PAGE: &str = "/app/register";
pub const REGISTER_EMAIL_CHECK_PAGE: &str = "/app/register/check_email";
pub const REGISTER_EMAIL_CODE_PAGE: &str = "/app/register/email_code";

pub fn register_scope() -> actix_web::Scope {
    web::scope("register")
        .service(web::resource("").get(registration_form).post(register))
        .service(web::resource("check_email").post(check_email))
        .service(web::resource("email_code").get(email_code_confirm))
}
