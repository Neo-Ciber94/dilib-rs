use crate::entities::audit_log::LogLevel;
use crate::{AuditLog, AuditLogService};
use actix_web::http::Method;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use dilib::resolve;
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
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
        if !can_log_method(&method) {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

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
            let audit_log_service = resolve!(AuditLogService).unwrap();
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

fn generate_message(_req: &ServiceRequest) -> Option<String> {
    None
}

fn can_log_method(method: &Method) -> bool {
    matches!(method, &Method::POST | &Method::PATCH | &Method::PUT | &Method::DELETE)
}
