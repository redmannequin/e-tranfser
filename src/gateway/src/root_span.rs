use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
};
use tracing::{field, Span};
use tracing_actix_web::{root_span, DefaultRootSpanBuilder, RootSpanBuilder};

pub struct DomainRootSpanBuilder;

impl RootSpanBuilder for DomainRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        root_span!(request, application_id = field::Empty)
    }

    fn on_request_end<B: MessageBody>(
        span: Span,
        response: &Result<ServiceResponse<B>, actix_web::Error>,
    ) {
        DefaultRootSpanBuilder::on_request_end(span, response);
    }
}
