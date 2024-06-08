use std::fmt;

use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
};
use domain::PaymentState;
use tracing::{field, Span};
use tracing_actix_web::{root_span, DefaultRootSpanBuilder, RootSpanBuilder};

const PAYMENT_ID: &str = "payment_id";
const PAYMENT_STATE: &str = "payment_state";
const PAYOUT_ID: &str = "payout_id";
const REFUND_ID: &str = "refund_id";

pub struct DomainRootSpanBuilder;

impl RootSpanBuilder for DomainRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        root_span!(
            request,
            payment_id = field::Empty,
            payout_id = field::Empty,
            payment_state = field::Empty
        )
    }

    fn on_request_end<B: MessageBody>(
        span: Span,
        response: &Result<ServiceResponse<B>, actix_web::Error>,
    ) {
        DefaultRootSpanBuilder::on_request_end(span, response);
    }
}

pub fn set_payment_id(payment_id: impl fmt::Display) {
    tracing::Span::current().record(PAYMENT_ID, payment_id.to_string());
}

pub fn set_payout_id(payout_id: impl fmt::Display) {
    tracing::Span::current().record(PAYOUT_ID, payout_id.to_string());
}

pub fn set_refund_id(refund_id: impl fmt::Display) {
    tracing::Span::current().record(REFUND_ID, refund_id.to_string());
}

pub fn set_payment_state(payment_state: PaymentState) {
    tracing::Span::current().record(PAYMENT_STATE, payment_state.as_str());
}
