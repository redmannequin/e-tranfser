use std::time::Duration;

use reqwest::ClientBuilder;
use reqwest_middleware::ClientWithMiddleware;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use reqwest_tracing::TracingMiddleware;

use crate::PlaidError;

pub struct PlaidClient {
    client: ClientWithMiddleware,
}

impl PlaidClient {
    const TIMEOUT: u64 = 2500;

    pub async fn new() -> Result<PlaidClient, PlaidError> {
        let raw_client = ClientBuilder::new()
            .timeout(Duration::from_millis(Self::TIMEOUT))
            .build()
            .unwrap();

        let client = reqwest_middleware::ClientBuilder::new(raw_client)
            .with(TracingMiddleware::default())
            .with(RetryTransientMiddleware::new_with_policy(
                ExponentialBackoff::builder()
                    .retry_bounds(Duration::from_millis(300), Duration::from_millis(2000))
                    .build_with_max_retries(3),
            ))
            .build();

        let res = Self { client };

        Ok(res)
    }
}
