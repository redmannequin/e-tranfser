pub mod create_payment;
pub mod deposit_payment;
pub mod tl_webhooks;

use db::error::DbError;
use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum PublicError {
    #[error("Something went wrong. Please retry later.")]
    InternalServerError,
    #[error("{0}")]
    Invalid(String),
}

impl From<PublicError> for actix_web::Error {
    #[inline]
    fn from(err: PublicError) -> Self {
        match err {
            err @ PublicError::Invalid(_) => actix_web::error::ErrorBadRequest(err),
            err => actix_web::error::ErrorInternalServerError(err),
        }
    }
}

impl From<DbError> for PublicError {
    #[inline]
    fn from(err: DbError) -> Self {
        println!("\nDbERROR:\n{:?}", err);
        PublicError::InternalServerError
    }
}

pub fn deserialize_body<'de, T>(body: &'de str) -> Result<T, PublicError>
where
    T: Deserialize<'de>,
{
    let jd = &mut serde_json::Deserializer::from_str(body);
    serde_path_to_error::deserialize(jd).map_err(|e| PublicError::Invalid(format!("{e}")))
}
