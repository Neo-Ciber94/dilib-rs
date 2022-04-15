use std::any::TypeId;
use crate::entities::audit_log::LogLevel;
use crate::{AuditLog, AuditLogService};
use actix_web::http::Method;
use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error};
use dilib::resolve;
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::marker::PhantomData;
use crate::utils::Type;

pub struct AuditLogger<T>(PhantomData<T>);
pub fn audit_logger<T: 'static>() -> AuditLogger<T> {
    AuditLogger(PhantomData)
}

impl<T, S, B> Transform<S, ServiceRequest> for AuditLogger<T>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
    T: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuditLoggerMiddleware<S, T>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuditLoggerMiddleware {
            service,
            _marker: PhantomData
        }))
    }
}

pub struct AuditLoggerMiddleware<S, T> {
    service: S,
    _marker: PhantomData<T>,
}

impl<S, B, T> Service<ServiceRequest> for AuditLoggerMiddleware<S, T>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
    T: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().clone();

        // Ignore some requests
        if !can_log_method(&method) {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        let ip = req.peer_addr().map(|addr| addr.ip());
        let message = generate_message::<T>(&req);
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

fn generate_message<T: 'static>(_req: &ServiceRequest) -> Option<String> {
    // We ignore unit type
    if is_unit::<T>() {
        return None;
    }

    let ty = Type::of::<T>();
    let message = format!("On resource: {}", ty.name());
    return Some(message);
}

fn can_log_method(method: &Method) -> bool {
    matches!(method, &Method::POST | &Method::PATCH | &Method::PUT | &Method::DELETE)
}

fn is_unit<T: 'static>() -> bool {
    TypeId::of::<T>() == TypeId::of::<()>()
}