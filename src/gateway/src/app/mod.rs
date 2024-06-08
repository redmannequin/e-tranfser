mod component;
mod home;
mod not_found;
mod tl_data_callback;

pub mod admin;
pub mod deposit_flow;
pub mod payment_flow;
pub mod registration_flow;

use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    dev::HttpServiceFactory,
    web, Scope,
};
pub use home::home;
pub use not_found::not_found;
pub use tl_data_callback::tl_data_callback;

pub fn app_scope(secret_key: Key) -> impl HttpServiceFactory + 'static {
    Scope::new("app")
        .wrap(
            SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .session_lifecycle(PersistentSession::default().session_ttl(Duration::minutes(15)))
                .build(),
        )
        .service(web::resource("").get(home))
        .service(payment_flow::payment_scope())
        .service(deposit_flow::deposit_scope())
        .service(registration_flow::register_scope())
}
