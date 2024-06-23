use std::sync::Arc;

use contracts::payment_manager::server::{
    CreatePaymentRequest, CreatePaymentResponse, CreatePayoutRequest, CreatePayoutResponse,
    PaymentManager,
};
use domain::PaymentId;

use crate::{
    handlers::{self, PaymentInfo},
    AppContext,
};

pub struct MyPaymentManager {
    app: Arc<AppContext>,
}

impl MyPaymentManager {
    pub fn new(app: Arc<AppContext>) -> Self {
        MyPaymentManager { app }
    }
}

#[tonic::async_trait]
impl PaymentManager for MyPaymentManager {
    async fn create_payout(
        &self,
        request: tonic::Request<CreatePayoutRequest>,
    ) -> Result<tonic::Response<CreatePayoutResponse>, tonic::Status> {
        let request = request.into_inner();

        let payment_id = PaymentId::parse_str(&request.payment_id)
            .map_err(|_| tonic::Status::invalid_argument("Invalid Payment ID"))?;
        let iban = &request.payee_iban;
        let reference = &request.reference;

        let payout_id = handlers::create_payout(&self.app, payment_id, iban, reference).await;

        Ok(tonic::Response::new(CreatePayoutResponse {
            payout_id: payout_id.to_string(),
        }))
    }

    async fn create_payment(
        &self,
        request: tonic::Request<CreatePaymentRequest>,
    ) -> std::result::Result<tonic::Response<CreatePaymentResponse>, tonic::Status> {
        let request = request.into_inner();

        let payment_info = PaymentInfo {
            payer_full_name: &request.payer_full_name,
            payer_email: &request.payer_email,
            payee_full_name: &request.payee_full_name,
            payee_email: &request.payee_email,
            amount: request.amount,
            security_question: &request.security_question,
            security_answer: &request.security_answer,
        };

        let (payment_id, resource_token) = handlers::create_payment(&self.app, payment_info).await;

        Ok(tonic::Response::new(CreatePaymentResponse {
            payment_id: payment_id.to_string(),
            resource_token,
        }))
    }
}
