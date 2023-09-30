use std::time::Duration;

use reqwest::{ClientBuilder, StatusCode};
use reqwest_middleware::ClientWithMiddleware;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use reqwest_tracing::TracingMiddleware;
use tracing::instrument;
use uuid::Uuid;

use crate::{TlConfig, TlEnviorment};

use super::{model::AuthResponse, CreatePaymentResponse, TlError};

pub struct TlClient {
    client: ClientWithMiddleware,
    enviornment: TlEnviorment,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    merchant_account_id: Uuid,
}

impl TlClient {
    const TIMEOUT: u64 = 2500;

    pub fn new(tl_config: TlConfig) -> Self {
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

        Self {
            client,
            enviornment: tl_config.enviornment,
            client_id: tl_config.client_id,
            client_secret: tl_config.client_secret,
            redirect_uri: tl_config.redirect_uri,
            merchant_account_id: tl_config.merchant_account_id,
        }
    }

    pub fn return_uri(&self) -> &str {
        &self.redirect_uri
    }

    //
    // PAYMENTS V3 API
    //

    pub async fn auth_payments_v3(&self) -> Result<AuthResponse, TlError> {
        let endpoint = format!("http://auth.{}/connect/token", self.enviornment.uri());
        let req = self
            .client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .body(format!(
                r#"
                    {{
                        "grant_type": "client_credentials",
                        "client_id": "{}",
                        "client_secret": "{}",
                        "scope": "payments"
                    }}
                "#,
                self.client_id, self.client_secret
            ))
            .build()
            .map_err(reqwest_middleware::Error::Reqwest)?;
        let res = self.client.execute(req).await?;
        match res.status() {
            StatusCode::OK => res.json().await.map_err(TlError::Response),
            StatusCode::BAD_REQUEST | StatusCode::INTERNAL_SERVER_ERROR => todo!(),
            _ => todo!(),
        }
    }

    #[instrument(skip(self))]
    pub async fn create_ma_payment(
        &self,
        payer_full_name: &str,
        payer_email: &str,
        payer_phonenumber: Option<&str>,
        amount: u32,
        reference: &str,
    ) -> Result<CreatePaymentResponse, TlError> {
        let endpoint = format!("http://api.{}/v3/payments", self.enviornment.uri());
        let req = self
            .client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .header("Authorization", self.client_id.clone())
            .body(format!(
                r#"
                    {{
                        "amount_in_minor": {},
                        "currency": "GBP",
                        "payment_method": {{
                            "type": "bank_transfer",
                            "provider_selection": {{
                                "type": "user_selected",
                                "scheme_selection": {{
                                    "type": "instant_only",
                                    "allow_remitter_fee": false
                                }}
                            }},
                            "beneficiary": {{
                                "type": "merchant_account",
                                "merchant_account_id": "{}",
                                "reference": "{}"
                            }}
                        }},
                        "user": {{
                            "name": "{}",
                            "email": "{}"
                            {}
                        }}
                    }}
                "#,
                amount,
                self.merchant_account_id,
                reference,
                payer_full_name,
                payer_email,
                payer_phonenumber
                    .map(|pn| format!(r#","phone": "{}""#, pn))
                    .unwrap_or_default()
            ))
            .build()
            .map_err(reqwest_middleware::Error::Reqwest)?;
        let res = self.client.execute(req).await?;
        match res.status() {
            StatusCode::CREATED => res
                .json::<CreatePaymentResponse>()
                .await
                .map_err(TlError::Response),
            _ => todo!(),
        }
    }

    #[instrument(skip(self))]
    pub async fn create_payout(
        &self,
        payee_full_name: &str,
        payee_iban: &str,
        amount: u32,
        reference: &str,
    ) {
        let endpoint = format!("https://api.{}/v3/payout", "");
        let req = self
            .client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .body(format!(
                r#"
                    {{
                        "amount_in_minor": {},
                        "merchant_account_id": "{}",
                        "currency": "GBP",
                        "benficiary": {{
                            "type": "external_account",
                            "reference": "{}",
                            "account_holder_name": "{}",
                            "account_identifier": {{
                                "type": "iban",
                                "iban": "{}",
                            }}
                        }}
                    }}
                "#,
                amount, self.merchant_account_id, reference, payee_full_name, payee_iban
            ))
            .build()
            .unwrap();
        let _res = self.client.execute(req).await.unwrap();
    }

    //
    //  DATA API
    //

    pub async fn auth_data(&self) {
        let endpoint = "https://auth.turelayer.com/connect/token";
        let req = self
            .client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .body(format!(
                r#"
                    {{
                        "grant_type": "authoriztion_code",
                        "client_id": {},
                        "client_secret": {},
                        "code": {},
                        "redirect_uri": {}
                    }}
                "#,
                "clinet_id", "client_secret", "code", "redirect_uri"
            ))
            .build()
            .unwrap();
        let _res = self.client.execute(req).await.unwrap();
    }

    pub async fn get_accounts(&self) {
        let _endpoint = format!("https://api.{}/data/v1/accounts", "");
    }
}
