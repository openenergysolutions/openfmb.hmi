// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use std::time::Duration;

use async_nats::Event;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::configuration::auth_provider::auth_type::AuthType;
use crate::configuration::auth_provider::keycloak;

/// NATS message
#[derive(Debug, Clone)]
pub struct SubMessage(pub Arc<async_nats::Message>);

#[derive(Debug, Clone)]
pub struct PubMessage(pub PublishingMessage);

/// A message to be published.
#[derive(Clone, Debug)]
pub struct PublishingMessage {
    /// The subject this message publishes to.
    pub subject: String,

    /// The message contents.
    pub data: Vec<u8>,
}

/// A callback function that calculates the backoff delay for the next retry.
/// The function takes the current attempt number and the previous delay as input.
/// It returns the next delay to use for the next retry.
pub type BackoffCalculationCallback = Box<dyn Fn(i32, Duration) -> Duration + Send + Sync>;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NatsConfiguration {
    pub connection_url: String,
    pub creds: String,
    pub auth: AuthType,
}

pub struct NatsConnector {
    connection_url: String,
    creds: String,
    auth: AuthType,
    backoff_callback: Option<BackoffCalculationCallback>,
    max_attempts: Option<i32>,
}

impl NatsConnector {
    pub fn new(pub_sub: &NatsConfiguration) -> Self {
        Self {
            connection_url: pub_sub.connection_url.clone(),
            creds: pub_sub.creds.clone(),
            auth: pub_sub.auth.clone(),
            max_attempts: None,
            backoff_callback: None,
        }
    }

    pub fn options(&self) -> async_nats::ConnectOptions {
        if !self.creds.is_empty() {
            log::debug!("Create nats option with cred file: {}", self.creds);
            return async_nats::ConnectOptions::with_credentials(&self.creds)
                .expect("error getting nats connection options with credentials");
        }

        match self.auth {
            AuthType::NIL => {} // Do nothing, intentional
            AuthType::KEYCLOAK => {
                return async_nats::ConnectOptions::with_auth_callback(move |_| async move {
                    let auth = keycloak::get_token().await;
                    Ok(auth)
                });
            }
        }

        async_nats::ConnectOptions::new()
    }

    pub fn max_attempts(mut self, attempts: i32) -> Self {
        self.max_attempts = Some(attempts);
        self
    }

    pub fn backoff<F>(mut self, f: F) -> Self
    where
        F: Fn(i32, Duration) -> Duration + Send + Sync + 'static,
    {
        self.backoff_callback = Some(Box::new(f));
        self
    }

    /// Connects to NATS with retry attempts and exponential backoff + jitter, based on backoff_callback.
    ///
    /// - Retries the connection attempt with growing delay: `1s → 2s → 4s ...` (max 30s).
    /// - Adds up to 1s of random jitter to avoid thundering herd issues.
    /// - Optionally stops retrying after `max_attempts` (use `None` to retry forever).
    /// - Logs all retry attempts and delays.
    ///
    /// Use this when:
    ///   - You want graceful and adaptive retry logic,
    ///   - You want to avoid retry storms in large systems,
    ///   - You may want to cap retry attempts.
    ///
    /// Internally:
    ///   - Implements exponential backoff with capped max delay.
    ///   - Uses jitter to spread out connection attempts across multiple clients.    
    pub async fn connect(&self) -> Result<async_nats::Client, async_nats::Error> {
        let mut attempt = 0;
        let mut delay = Duration::from_secs(1);
        let max_delay = Duration::from_secs(30);

        loop {
            attempt += 1;

            match self
                .options()
                .event_callback(|event| async move {
                    match event {
                        Event::Connected => {
                            log::info!("Successfully connected to NATS!");
                        }
                        Event::Disconnected => {
                            log::info!("Connection to NATS is lost!");
                        }
                        Event::LameDuckMode => {
                            log::info!("Connection to NATS in Lame Duck mode");
                        }
                        Event::SlowConsumer(sid) => {
                            log::warn!("Slow consumers for NATS subscription {sid}");
                        }
                        Event::ServerError(e) => {
                            log::error!("NATS server error: {e}");
                        }
                        Event::ClientError(e) => {
                            log::error!("NATS client error: {e}");
                        }
                        Event::Draining => {
                            log::info!("NATS connection draining");
                        }
                        Event::Closed => {
                            log::info!("NATS connection closed");
                        }
                    }
                })
                .connect(&self.connection_url)
                .await
            {
                Ok(client) => {
                    log::debug!("Connected to NATS on attempt {}", attempt);
                    return Ok(client);
                }
                Err(e) => {
                    log::warn!("Attempt {}: Error connecting to NATS: {}", attempt, e);

                    if let Some(max) = self.max_attempts {
                        if attempt >= max {
                            log::error!("Exceeded max retry attempts.");
                            return Err(e.into());
                        }
                    }

                    delay = match &self.backoff_callback {
                        Some(cb) => {
                            let d = cb(attempt, delay);
                            log::info!("Retrying in {:?}...", d);
                            tokio::time::sleep(d).await;
                            d
                        }
                        None => {
                            let jitter_ms = rand::rng().random_range(0..=1000);
                            let delay_with_jitter = delay + Duration::from_millis(jitter_ms);

                            // Ensure we don’t exceed the max_delay
                            let capped_delay = std::cmp::min(delay_with_jitter, max_delay);
                            log::info!("Retrying in {:?}...", capped_delay);
                            tokio::time::sleep(capped_delay).await;
                            std::cmp::min(delay * 2, max_delay)
                        }
                    };
                }
            }
        }
    }
}

impl Default for NatsConfiguration {
    fn default() -> Self {
        let platform_node_ip =
            std::env::var("PLATFORM_NODE_IP").unwrap_or_else(|_| "nats://localhost".to_string());
        let platform_nats_port =
            std::env::var("PLATFORM_NATS_PORT").unwrap_or_else(|_| "4222".to_string());
        let connection_url = format!("{platform_node_ip}:{platform_nats_port}");

        NatsConfiguration {
            connection_url,
            creds: "".to_string(),
            auth: AuthType::NIL,
        }
    }
}

impl NatsConfiguration {
    fn parse(cfg: &config::Config) -> Result<NatsConfiguration, config::ConfigError> {
        cfg.clone().get::<NatsConfiguration>("nats")
    }

    pub fn new(config: &config::Config) -> NatsConfiguration {
        NatsConfiguration::parse(config).unwrap()
    }

    #[deprecated(since = "0.1.1", note = "please use `connector().options()` instead")]
    pub fn options(&self) -> async_nats::ConnectOptions {
        if !self.creds.is_empty() {
            log::debug!("Create nats option with cred file: {}", self.creds);
            return async_nats::ConnectOptions::with_credentials(&self.creds)
                .expect("error getting nats connection options with credentials");
        }

        match self.auth {
            AuthType::NIL => {} // Do nothing, intentional
            AuthType::KEYCLOAK => {
                return async_nats::ConnectOptions::with_auth_callback(move |_| async move {
                    let auth = keycloak::get_token().await;
                    Ok(auth)
                });
            }
        }

        async_nats::ConnectOptions::new()
    }

    pub fn connector(&self) -> NatsConnector {
        NatsConnector::new(self)
    }

    /// Attempts to connect to NATS using `retry_on_initial_connect`.
    ///
    /// - Automatically retries in the background if the initial connection fails.
    /// - Does **not block** or return an error immediately if the NATS server is unreachable.
    /// - Returns immediately with a client instance that will retry silently in the background.
    /// - Uses a 10-second request timeout and sets up an event callback for connection state changes.
    ///
    /// Note:
    ///   - This may appear to "succeed" even if the NATS server is down at the time of calling.
    ///   - Use this when you want fire-and-forget connection setup (e.g., for daemons or long-running services).
    #[deprecated(since = "0.1.1", note = "please use `connector().connect()` instead")]
    #[allow(deprecated)]
    pub async fn connect(&self) -> Result<async_nats::Client, async_nats::Error> {
        log::info!("Connecting to nats...{}", self.connection_url);

        Ok(self
            .options()
            .retry_on_initial_connect()
            .request_timeout(Some(Duration::from_secs(10)))
            .event_callback(|event| async move {
                match event {
                    Event::Connected => {
                        log::info!("Successfully connected to NATS!");
                    }
                    Event::Disconnected => {
                        log::info!("Connection to NATS is lost!");
                    }
                    Event::LameDuckMode => {
                        log::info!("Connection to NATS in Lame Duck mode");
                    }
                    Event::SlowConsumer(sid) => {
                        log::warn!("Slow consumers for NATS subscription {sid}");
                    }
                    Event::ServerError(e) => {
                        log::error!("NATS server error: {e}");
                    }
                    Event::ClientError(e) => {
                        log::error!("NATS client error: {e}");
                    }
                    Event::Draining => {
                        log::info!("NATS connection draining");
                    }
                    Event::Closed => {
                        log::info!("NATS connection closed");
                    }
                }
            })
            .connect(self.connection_url.clone())
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::configuration::pub_sub::NatsConfiguration;

    #[ignore = "reason: this test requires a running nats server"]
    #[tokio::test]
    // This test will pass even if the nats server is not running
    // because of `retry_on_initial_connect()` option
    async fn test_legacy_connection() {
        env_logger::init();
        let pub_sub = NatsConfiguration::default();
        match pub_sub
            .connector()
            .backoff(|attempt, prev| {
                let delay = std::cmp::min(prev * 1, std::time::Duration::from_secs(30));
                log::info!("Custom backoff: attempt={}, delay={:?}", attempt, delay);
                delay
            })
            .connect()
            .await
        {
            Ok(_client) => {
                log::info!("Connected to nats");
            }
            Err(e) => {
                panic!("Error connecting to nats: {}", e);
            }
        }
    }

    #[ignore = "reason: this test requires a running nats server"]
    #[tokio::test]
    async fn test_default_backoff_connection() {
        env_logger::init();
        let config = NatsConfiguration::default();

        match config.connector().max_attempts(5).connect().await {
            Ok(_client) => {
                log::info!("Connected to nats");
            }
            Err(e) => {
                panic!("Error connecting to nats: {}", e);
            }
        }
    }

    #[ignore = "reason: this test requires a running nats server"]
    #[tokio::test]
    async fn test_custom_backoff_connection() {
        env_logger::init();
        let config = NatsConfiguration::default();

        match config
            .connector()
            .backoff(|attempt, prev| {
                let delay = std::cmp::min(prev * 2, std::time::Duration::from_secs(30));
                log::info!("Custom backoff: attempt={}, delay={:?}", attempt, delay);
                delay
            })
            .max_attempts(5)
            .connect()
            .await
        {
            Ok(_client) => {
                log::info!("Connected to nats");
            }
            Err(e) => {
                panic!("Error connecting to nats: {}", e);
            }
        }
    }
}
