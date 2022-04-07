use crate::entities::audit_log::LogLevel;
use crate::{AuditLog, AuditLogService};
use actix_web::web::Data;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use dilib::Container;
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use actix_web::http::Method;
pub struct AuditLogger;

impl<S, B> Transform<S, ServiceRequest> for AuditLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuditLoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuditLoggerMiddleware { service }))
    }
}

pub struct AuditLoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuditLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().clone();

        // Ignore get requests
        if method == Method::GET {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        let container = req.app_data::<Data<Container<'static>>>().unwrap().clone();
        let ip = req.peer_addr().map(|addr| addr.ip());
        let message = generate_message(&req);
        let route = req.match_info().as_str().to_owned();
        let user_agent = req
            .headers()
            .get("user-agent")
            .and_then(|s| s.to_str().ok())
            .map(|s| s.to_owned());

        let fut = self.service.call(req);

        Box::pin(async move {
            let audit_log_service = container.get::<AuditLogService>().unwrap();
            let start_time = std::time::Instant::now();
            let res: Self::Response = fut.await?;
            let end_time = std::time::Instant::now();

            let duration = end_time.duration_since(start_time);
            let duration_ms = duration.as_millis();
            let level = match res.status().as_u16() {
                status if status >= 400 => LogLevel::Error,
                _ => LogLevel::Info,
            };

            let audit_log = AuditLog::builder()
                .ip(ip)
                .message(message)
                .method(method.into())
                .route(route)
                .user_agent(user_agent)
                .duration_ms(duration_ms)
                .level(level)
                .build();

            audit_log_service.log(audit_log).await;

            Ok(res)
        })
    }
}

pub fn generate_message(req: &ServiceRequest) -> Option<String> {
    match *req.method() {
        Method::GET => Some("Get".to_owned()),
        Method::POST => Some("Create".to_owned()),
        Method::PATCH | Method::PUT => Some("Update".to_owned()),
        Method::DELETE => Some("Delete".to_owned()),
        _ => None,
    }
}