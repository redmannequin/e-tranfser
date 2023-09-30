use reqwest_middleware::ClientWithMiddleware;
use tracing::instrument;

pub enum TlEnviorment {
    Sandbox,
    Production,
}

pub struct TlClient {
    client: ClientWithMiddleware,
    enviornment: TlEnviorment,
}

impl TlClient {
    //
    // PAYMENTS V3 API
    //

    pub async fn auth_payments_v3(&self) {
        let endpoint = "http://auth.truelayer.com/connect/token";
        let req = self
            .client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .body(format!(
                r#"
                    {{
                        "grant_type": "client_credentials",
                        "client_id": {},
                        "client_secret": {},
                        "scope": "payments"
                    }}
                "#,
                "clinet_id", "client_secret"
            ))
            .build()
            .unwrap();
        let _res = self.client.execute(req).await.unwrap();
    }

    #[instrument(skip(self))]
    pub async fn create_ma_payment(&self, amount: u32, reference: &str) {
        let endpoint = format!("https://api.{}/v3/payments", "");
        let req = self
            .client
            .post(endpoint)
            .header("Content-Type", "application/json")
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
                                "merchant_account_id": {},
                                "reference": {}
                            }}
                        }},
                        "user": {{
                            "name": "test",
                            "email": "email"
                        }}
                    }}
                "#,
                "amount_in_minor", "merchant_ccount_id", "reference"
            ))
            .build()
            .unwrap();
        let _res = self.client.execute(req).await.unwrap();
    }

    pub async fn create_payout(&self, amount: u32) {
        let endpoint = format!("https://api.{}/v3/payout", "");
        let req = self
            .client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .body(format!(
                r#"
                    {{
                        "amount_in_minor": {},
                        "merchant_account_id": {},
                        "currency": "GBP",
                        "benficiary": {{
                            "type": "external_account",
                            "reference": {},
                            "account_holder_name": {},
                            "account_identifier": {{
                                "type": "iban",
                                "iban": {},
                            }}
                        }}
                    }}
                "#,
                "amount_in_minor",
                "merchant_account_id",
                "reference",
                "account_holder_name",
                "iban"
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
        let endpoint = format!("https://api.{}/data/v1/accounts", "");
    }
}
