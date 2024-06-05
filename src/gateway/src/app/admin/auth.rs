use std::future::{ready, Ready};

use actix_web::{
    body::BoxBody,
    cookie::Cookie,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error, HttpResponse,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use futures::future::LocalBoxFuture;

pub struct AdminAuth;

impl<S> Transform<S, ServiceRequest> for AdminAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AdminAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AdminAuthMiddleware { service }))
    }
}

pub struct AdminAuthMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for AdminAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_flag = req
            .headers()
            .get(header::COOKIE)
            .and_then(|cookie_header| {
                cookie_header.to_str().ok().and_then(|cookie_str| {
                    cookie_str
                        .split(';')
                        .map(|s| Cookie::parse(s.trim()).ok())
                        .find(|x| x.as_ref().map(|x| x.name() == "admin").unwrap_or(false))
                })
            })
            .flatten()
            .as_ref()
            .map(|admin_cookie| admin_cookie.value())
            .map(|auth_value| {
                let admin_hash = PasswordHash::new(auth_value).unwrap();
                Argon2::default()
                    .verify_password(b"admin", &admin_hash)
                    .map_or(false, |_| true)
            })
            .unwrap_or(false);

        if auth_flag {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async {
                Ok(req.into_response(
                    HttpResponse::SeeOther()
                        .insert_header((header::LOCATION, "/admin/unauthorized"))
                        .finish(),
                ))
            })
        }
    }
}
