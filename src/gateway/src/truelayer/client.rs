use std::{sync::Arc, time::Duration};

use actix_web::http::header;
use reqwest::{ClientBuilder, StatusCode};
use reqwest_middleware::ClientWithMiddleware;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use reqwest_tracing::TracingMiddleware;
use tokio::sync::Mutex;
use tracing::instrument;
use truelayer_signing::Method;
use uuid::Uuid;

use crate::{TlConfig, TlEnviorment};

use super::{
    model::{AccountBalance, AuthResponse, CreatePayoutResponse, GetAccountBalance, GetAccounts},
    CreatePaymentResponse, TlError,
};

pub struct TlClient {
    client: ClientWithMiddleware,
    pub enviornment: TlEnviorment,
    pub client_id: String,
    pub client_secret: String,
    pub kid: String,
    private_key: String,
    access_token: Arc<Mutex<Option<String>>>,
    pub redirect_uri: String,
    pub data_redirect_uri: String,
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
            kid: tl_config.kid,
            private_key: tl_config.private_key,
            access_token: Arc::new(Mutex::new(None)),
            redirect_uri: tl_config.redirect_uri,
            data_redirect_uri: tl_config.data_redirect_uri,
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
        let endpoint = format!("https://auth.{}/connect/token", self.enviornment.uri());

        let body = format!(
            r#"
                    {{
                        "grant_type": "client_credentials",
                        "client_secret": "{}",
                        "client_id": "{}",
                        "scope": "payments"
                    }}
                "#,
            self.client_secret, self.client_id
        );

        let req = self
            .client
            .post(endpoint)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .body(body)
            .build()
            .map_err(reqwest_middleware::Error::Reqwest)?;

        let res = self.client.execute(req).await?;
        match res.status() {
            StatusCode::OK => res.json().await.map_err(TlError::Response),
            StatusCode::BAD_REQUEST | StatusCode::INTERNAL_SERVER_ERROR => {
                let err: serde_json::Value = res.json().await.map_err(TlError::Response)?;
                println!("\nTlClient::Auth ERROR:\n{:?}", err);
                unimplemented!()
            }
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
        let endpoint = format!("https://api.{}/v3/payments", self.enviornment.uri());

        let access_token = {
            let mut access_token = self.access_token.lock().await;
            if access_token.is_none() {
                *access_token = Some(self.auth_payments_v3().await?.access_token);
            }
            access_token.to_owned().unwrap()
        };

        let idempotency_key = Uuid::new_v4().to_string();
        let body = format!(
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
        );

        let tl_signature =
            truelayer_signing::sign_with_pem(self.kid.as_str(), self.private_key.as_bytes())
                .method(Method::Post)
                .path("/v3/payments")
                .header("Idempotency-Key", idempotency_key.as_bytes())
                .body(body.as_bytes())
                .build_signer()
                .sign()
                .unwrap();

        let req = self
            .client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Idempotency-Key", idempotency_key)
            .header("Tl-Signature", tl_signature)
            .body(body)
            .build()
            .map_err(reqwest_middleware::Error::Reqwest)?;

        let res = self.client.execute(req).await?;
        match res.status() {
            StatusCode::CREATED => res
                .json::<CreatePaymentResponse>()
                .await
                .map_err(TlError::Response),
            _ => {
                let err: serde_json::Value = res.json().await.map_err(TlError::Response)?;
                println!("\nTlClient::Payment Error:\n{}", err);
                unimplemented!()
            }
        }
    }

    #[instrument(skip(self))]
    pub async fn create_payout(
        &self,
        payee_full_name: &str,
        payee_iban: &str,
        amount: u32,
        reference: &str,
    ) -> Result<CreatePayoutResponse, TlError> {
        let endpoint = format!("http://api.{}/v3/payouts", self.enviornment.uri());
        let req = self
            .client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .header("Authorization", self.client_id.clone())
            .body(format!(
                r#"
                    {{
                        "amount_in_minor": {},
                        "merchant_account_id": "{}",
                        "currency": "GBP",
                        "beneficiary": {{
                            "type": "external_account",
                            "reference": "{}",
                            "account_holder_name": "{}",
                            "account_identifier": {{
                                "type": "iban",
                                "iban": "{}"
                            }}
                        }}
                    }}
                "#,
                amount, self.merchant_account_id, reference, payee_full_name, payee_iban
            ))
            .build()
            .unwrap();
        let res = self.client.execute(req).await?;
        match res.status() {
            StatusCode::CREATED => res.json().await.map_err(TlError::Response),
            _ => {
                unimplemented!()
            }
        }
    }

    //
    //  DATA API
    //

    pub async fn auth_data(&self, code: &str) -> Result<AuthResponse, TlError> {
        let endpoint = format!("https://auth.{}/connect/token", self.enviornment.uri());
        let req = self
            .client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .body(format!(
                r#"
                    {{
                        "client_id": "{}",
                        "client_secret": "{}",
                        "code": "{}",
                        "grant_type": "authorization_code",
                        "redirect_uri": "{}"
                    }}
                "#,
                self.client_id, self.client_secret, code, self.data_redirect_uri
            ))
            .build()
            .unwrap();
        let res = self.client.execute(req).await?;
        match res.status() {
            StatusCode::OK => res.json().await.map_err(TlError::Response),
            _ => {
                let err: serde_json::Value = res.json().await.map_err(TlError::Response)?;
                println!("\nTlClient::DataAuth Error:\n{}", err);
                unimplemented!()
            }
        }
    }

    pub async fn get_accounts(&self, access_token: &str) -> Result<GetAccounts, TlError> {
        let endpoint = format!("https://api.{}/data/v1/accounts", self.enviornment.uri());
        let req = self
            .client
            .get(endpoint)
            .header(header::ACCEPT.as_str(), "application/json")
            .header(
                header::AUTHORIZATION.as_str(),
                format!("Bearer {}", access_token),
            )
            .build()
            .unwrap();
        let res = self.client.execute(req).await?;
        match res.status() {
            StatusCode::OK => res.json().await.map_err(TlError::Response),
            _ => {
                let err: serde_json::Value = res.json().await.map_err(TlError::Response)?;
                println!("\nTlClient::GetAccounts Error:\n{}", err);
                unimplemented!()
            }
        }
    }

    pub async fn get_account_balance(
        &self,
        access_token: &str,
        account_id: String,
    ) -> Result<AccountBalance, TlError> {
        let endpoint = format!(
            "https://api.{}/data/v1/accounts/{}/balance",
            self.enviornment.uri(),
            account_id
        );
        let req = self
            .client
            .get(endpoint)
            .header(header::ACCEPT.as_str(), "application/json")
            .header(
                header::AUTHORIZATION.as_str(),
                format!("Bearer {}", access_token),
            )
            .build()
            .unwrap();
        let res = self.client.execute(req).await?;
        match res.status() {
            StatusCode::OK => res
                .json::<GetAccountBalance>()
                .await
                .map(|mut a| a.results.pop().unwrap())
                .map_err(TlError::Response),
            _ => {
                let err: serde_json::Value = res.json().await.map_err(TlError::Response)?;
                println!("\nTlClient::GetAccounts Error:\n{}", err);
                unimplemented!()
            }
        }
    }
}
