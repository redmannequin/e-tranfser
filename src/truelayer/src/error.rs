#[derive(thiserror::Error, Debug)]
pub enum TlError {
    #[error("Unable to parse response: {0}")]
    Response(#[from] reqwest::Error),
    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest_middleware::Error),
}
