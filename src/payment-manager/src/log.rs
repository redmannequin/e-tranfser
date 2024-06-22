use http::{Request, Response};
use tonic::{body::BoxBody, transport::Body};
use tracing::Span;

use crate::otel::GrpcRootSpanBuilder;

#[derive(Debug, Clone, Copy)]
pub struct AppRootSpan;

impl GrpcRootSpanBuilder for AppRootSpan {
    fn on_request_start(_request: &Request<Body>) -> Span {
        let service_name = "name";
        let service_method = "method";

        tracing::span!(
            tracing::Level::INFO,
            "gRCP request",
            rpc.system = "grpc",
            rpc.service = %service_name,
            rpc.method = %service_method,
            rpc.grpc.status_code = tracing::field::Empty,
            otel.name = %format!("gRPC {}/{}", service_name, service_method),
            otel.kind = "server",
            otel.status_code = tracing::field::Empty,
        )
    }

    fn on_request_end(_span: Span, _response: &Response<BoxBody>) {
        todo!()
    }
}
