mod register;
mod registration_form;

use actix_web::web;
use register::register;
use registration_form::{check_email, registration_form};

pub fn register_scope() -> actix_web::Scope {
    web::scope("/register")
        .service(web::resource("").get(registration_form).post(register))
        .service(web::resource("/check_email").post(check_email))
}
