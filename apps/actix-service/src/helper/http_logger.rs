use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use serde_json::{json, to_string};
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
    fn headers_to_string(headers: &actix_web::http::header::HeaderMap) -> String {
        headers
            .iter()
            .map(|(name, value)| format!("{}: {}", name, value.to_str().unwrap_or("")))
            .collect::<Vec<String>>()
            .join(", ")
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
        let headers = Self::headers_to_string(req.headers());
        let logger = self.logger.clone();

        let fut = self.service.call(req);

        Box::pin(async move {
            // Log request information inside the async block
            let request_log = format!(
                "Request | Method: {} | Headers: {} | URL: {}",
                method.to_string(),
                headers,
                uri.to_string()
            );

            // Send formatted request log
            logger.info(request_log).await;

            let result = fut.await;
            let duration = start.elapsed();

            match result {
                Ok(response) => {
                    // Format duration to milliseconds
                    let duration_ms = duration.as_secs_f64() * 1000.0; // Convert to milliseconds
                    let response_log = format!(
                        "Response | Method: {} | URL: {} | Status: {} | Duration: {:.2} ms",
                        method.to_string(),
                        uri.to_string(),
                        response.status().as_u16(),
                        duration_ms
                    );

                    // Send formatted response log
                    logger.info(response_log).await;

                    Ok(response)
                }
                Err(e) => {
                    let error_log = json!( {
                        "method": method.to_string(),
                        "url": uri.to_string(),
                        "error": format!("{:?}", e),
                        "duration": duration.as_millis(),
                    });

                    // Convert error_log to string
                    let error_log_str = to_string(&error_log).unwrap_or_else(|e| format!("Error serializing error: {}", e));
                    logger.error(format!("Request failed | {}", error_log_str)).await;

                    Err(e)
                }
            }
        })
    }
}
