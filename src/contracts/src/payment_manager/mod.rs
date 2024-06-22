use std::str::FromStr;

use anyhow::Context;
use domain::{PaymentId, PayoutId};
use serde::Deserialize;
use tonic::{
    codegen::{Body, Bytes, StdError},
    transport::{Channel, Uri},
};
use tower::ServiceBuilder;

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
    // pub async fn connect(host: &str, port: u16) -> anyhow::Result<Self> {
    //     Ok(PaymentManagerClient {
    //         inner: protos::PaymentManagerClient::connect((host, port))
    //             .await
    //             .context("connect")?,
    //     })
    // }

    pub async fn connect(host: &str, port: u16) -> anyhow::Result<Self> {
        let uri = Uri::from_str(&format!("http://{}:{}", host, port)).context("test")?;
        let channel = Channel::builder(uri).connect().await?;
        let channel = ServiceBuilder::new().service(channel);
        let client = protos::PaymentManagerClient::new(channel);
        Ok(PaymentManagerClient { inner: client })
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
