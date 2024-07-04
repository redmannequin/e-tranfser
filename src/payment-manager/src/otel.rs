use std::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use http::{Request, Response};
use tonic::{body::BoxBody, transport::Body};
use tower::{Layer, Service};
use tracing::Span;

type ServiceRequest = Request<Body>;
type ServiceResponse = Response<BoxBody>;

#[derive(Debug, Clone, Default)]
pub struct TracingLoggerLayer<RootSpan> {
    _root_span_type: PhantomData<RootSpan>,
}

impl<RootSpan> TracingLoggerLayer<RootSpan>
where
    RootSpan: GrpcRootSpanBuilder,
{
    pub fn new() -> Self {
        TracingLoggerLayer {
            _root_span_type: PhantomData,
        }
    }
}

pub trait GrpcRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span;
    fn on_request_end(span: Span, response: &ServiceResponse);
}

#[derive(Debug, Clone)]
pub struct TracingLoggerMiddleware<S, RootSpanType> {
    service: S,
    _root_span_type: PhantomData<RootSpanType>,
}

impl<S, RootSpanType> Layer<S> for TracingLoggerLayer<RootSpanType> {
    type Service = TracingLoggerMiddleware<S, RootSpanType>;

    fn layer(&self, service: S) -> Self::Service {
        TracingLoggerMiddleware {
            service,
            _root_span_type: PhantomData,
        }
    }
}

impl<S, RootSpanType> Service<ServiceRequest> for TracingLoggerMiddleware<S, RootSpanType>
where
    S: Service<ServiceRequest, Response = ServiceResponse> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Send,
    RootSpanType: GrpcRootSpanBuilder,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = TracingResponse<S::Future, RootSpanType>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let root_span = RootSpanType::on_request_start(&req);
        let root_span_wrapper = root_span.clone();
        req.extensions_mut().insert(root_span_wrapper);

        let future = root_span.in_scope(|| self.service.call(req));

        TracingResponse {
            future,
            root_span,
            _root_span_type: PhantomData,
        }
    }
}

#[pin_project::pin_project]
pub struct TracingResponse<F, RootSpan> {
    #[pin]
    future: F,
    root_span: Span,
    _root_span_type: PhantomData<RootSpan>,
}

impl<Fut, E, RootSpan> Future for TracingResponse<Fut, RootSpan>
where
    Fut: Future<Output = Result<ServiceResponse, E>>,
    RootSpan: GrpcRootSpanBuilder,
{
    type Output = Result<ServiceResponse, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let future = this.future;
        let root_span = this.root_span;

        root_span.in_scope(|| match future.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(outcome) => {
                match outcome {
                    Ok(ref outcome) => RootSpan::on_request_end(Span::current(), outcome),
                    _ => todo!(),
                }
                Poll::Ready(outcome)
            }
        })
    }
}
