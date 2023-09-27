pub mod create_payment;

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

pub fn deserialize_body<'de, T>(body: &'de str) -> Result<T, PublicError>
where
    T: Deserialize<'de>,
{
    let jd = &mut serde_json::Deserializer::from_str(body);
    serde_path_to_error::deserialize(jd).map_err(|e| PublicError::Invalid(format!("{e}")))
}
