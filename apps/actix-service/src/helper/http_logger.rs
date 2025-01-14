use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use serde_json::json;
use futures::future::{ok, Ready};
use std::sync::Arc;
use std::time::Instant;

use crate::helper::logger::Logger as LogtailLogger;

#[derive(Clone)]
pub struct HttpLogger {
    logger: Arc<LogtailLogger>,
}

impl HttpLogger {
    pub fn new(logger: LogtailLogger) -> Self {
        Self {
            logger: Arc::new(logger),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for HttpLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = HttpLoggerService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(HttpLoggerService {
            service,
            logger: self.logger.clone(),
        })
    }
}

pub struct HttpLoggerService<S> {
    service: S,
    logger: Arc<LogtailLogger>,
}

impl<S> HttpLoggerService<S> {
    fn headers_to_json(headers: &actix_web::http::header::HeaderMap) -> serde_json::Value {
        headers
            .iter()
            .filter_map(|(name, value)| value.to_str().ok().map(|v| (name.to_string(), json!(v))))
            .collect::<serde_json::Map<_, _>>()
            .into()
    }
}

impl<S, B> Service<ServiceRequest> for HttpLoggerService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let method = req.method().clone();
        let uri = req.uri().clone();
        let headers = Self::headers_to_json(req.headers());
        let logger = self.logger.clone();

        let fut = self.service.call(req);

        Box::pin(async move {
            let result = fut.await;
            let duration = start.elapsed();

            match result {
                Ok(response) => {
                    let response_fields = json!({
                        "method": method.to_string(),
                        "uri": uri.to_string(),
                        "headers": headers,
                        "status": response.status().as_u16(),
                        "duration_ms": duration.as_millis(),
                    });

                    logger
                        .info(format!("Request completed: {}", response_fields))
                        .await;

                    Ok(response)
                }
                Err(e) => {
                    let error_fields = json!({
                        "method": method.to_string(),
                        "uri": uri.to_string(),
                        "headers": headers,
                        "error": format!("{:?}", e),
                        "duration_ms": duration.as_millis(),
                    });

                    logger
                        .error(format!("Request failed: {}", error_fields))
                        .await;

                    Err(e)
                }
            }
        })
    }
}
