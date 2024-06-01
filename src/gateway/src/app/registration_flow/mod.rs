mod registration_form;

use actix_web::web;
use registration_form::registration_form;

pub fn register_scope() -> actix_web::Scope {
    web::scope("/register").service(web::resource("").to(registration_form))
}
