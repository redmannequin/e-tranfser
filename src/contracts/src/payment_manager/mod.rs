use anyhow::Context;
use domain::{PaymentId, PayoutId};
use serde::Deserialize;
use tonic::codegen::{Body, Bytes, StdError};

pub mod server {
    pub use crate::protos::payment_manager::{
        payment_manager_server::{PaymentManager, PaymentManagerServer},
        CreatePaymentRequest, CreatePaymentResponse, CreatePayoutRequest, CreatePayoutResponse,
    };
}

mod protos {
    pub use crate::protos::payment_manager::{
        payment_manager_client::PaymentManagerClient, CreatePayoutRequest,
    };
}

pub type BigBoyGrpcChannel = tonic::transport::Channel;

#[derive(Debug, Clone, Deserialize)]
pub struct PaymentManagerConfig {
    pub host: String,
    pub port: u16,
}

pub struct PaymentManagerClient<T = BigBoyGrpcChannel> {
    inner: protos::PaymentManagerClient<T>,
}

impl PaymentManagerClient {
    pub async fn connect() -> anyhow::Result<Self> {
        Ok(PaymentManagerClient {
            inner: protos::PaymentManagerClient::connect("")
                .await
                .context("connect")?,
        })
    }
}

impl<T> PaymentManagerClient<T>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    T: Clone + Send,
{
    pub async fn create_payout(
        &self,
        payment_id: PaymentId,
        iban: String,
        reference: String,
    ) -> Result<PayoutId, tonic::Status> {
        let request = protos::CreatePayoutRequest {
            payment_id: payment_id.to_string(),
            payee_iban: iban,
            reference,
        };

        let mut grpc_client = self.inner.clone();
        grpc_client.create_payout(request).await.and_then(|res| {
            PayoutId::parse_str(&res.into_inner().payout_id).map_err(|err| {
                tonic::Status::unknown(format!("Failed to parse Payout ID: {}", err))
            })
        })
    }
}
