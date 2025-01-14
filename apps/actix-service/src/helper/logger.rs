use serde_json::json;
use chrono::Utc;
use reqwest::Client;
use once_cell::sync::Lazy;
use std::env;
use tracing::{info, error, warn, debug};

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

#[derive(Clone)]
pub struct Logger {
    source: String,
}


#[allow(dead_code)]
impl Logger {
    pub fn new(source: String) -> Self {
        Self { source }
    }

    async fn send_to_logtail(&self, level: &str, message: String) {
        if let Ok(api_key) = env::var("LOGTAIL_API_KEY") {
            let timestamp = Utc::now().to_rfc3339();

            let payload = json!({
                "dt": timestamp,
                "level": level,
                "message": message,
                "source": self.source,
            });

            let _ = CLIENT
                .post("https://in.logtail.com")
                .header("Authorization", format!("Bearer {}", api_key))
                .header("Content-Type", "application/json")
                .json(&payload)
                .send()
                .await;
        } else {
            error!("LOGTAIL_API_KEY is not set.");
        }
    }

    #[allow(dead_code)]
    pub async fn info<T: std::fmt::Display>(&self, message: T) {
        let msg = message.to_string();
        info!("{}", msg);
        self.send_to_logtail("info", msg).await;
    }

    #[allow(dead_code)]
    pub async fn error<T: std::fmt::Display>(&self, message: T) {
        let msg = message.to_string();
        error!("{}", msg);
        self.send_to_logtail("error", msg).await;
    }

    #[allow(dead_code)]
    pub async fn warn<T: std::fmt::Display>(&self, message: T) {
        let msg = message.to_string();
        warn!("{}", msg);
        self.send_to_logtail("warn", msg).await;
    }

    #[allow(dead_code)]
    pub async fn debug<T: std::fmt::Display>(&self, message: T) {
        let msg = message.to_string();
        debug!("{}", msg);
        self.send_to_logtail("debug", msg).await;
    }
}

// Global logger instance
static LOGGER: Lazy<Logger> = Lazy::new(|| {
    Logger::new("your-service-name".to_string())
});

// Public interface
#[allow(dead_code)]
pub async fn info<T: std::fmt::Display>(message: T) {
    LOGGER.info(message).await;
}

#[allow(dead_code)]
pub async fn error<T: std::fmt::Display>(message: T) {
    LOGGER.error(message).await;
}

#[allow(dead_code)]
pub async fn warn<T: std::fmt::Display>(message: T) {
    LOGGER.warn(message).await;
}

#[allow(dead_code)]
pub async fn debug<T: std::fmt::Display>(message: T) {
    LOGGER.debug(message).await;
}
