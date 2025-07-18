// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

pub mod auth_provider;
mod pub_sub;

use std::env;

use config::{Config, File};
pub use pub_sub::*;

use crate::configuration::logging::OESAppSettings;

pub mod logging;

pub fn load_config() -> Config {
    let app_conf_path = env::var("APP_CONF").unwrap_or_else(|_| "config/app".into());

    let cfg = Config::builder();
    cfg.add_source(File::with_name(&app_conf_path).required(false))
        .build()
        .unwrap()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HmiConfig {
    server_port: u16,
    server_host: Option<String>,
    ssl_cert: Option<String>,
    ssl_key: Option<String>,
}

impl Default for HmiConfig {
    fn default() -> Self {
        HmiConfig {
            server_port: 80,
            server_host: Some("0.0.0.0".to_string()),
            ssl_cert: None,
            ssl_key: None,
        }
    }
}

impl HmiConfig {
    pub fn server_host(&self) -> String {
        self.server_host.clone().unwrap_or("0.0.0.0".to_string())
    }
    pub fn server_port(&self) -> u16 {
        self.server_port
    }
    pub fn ssl_cert(&self) -> String {
        self.ssl_cert.clone().unwrap_or("".to_string())
    }
    pub fn ssl_key(&self) -> String {
        self.ssl_key.clone().unwrap_or("".to_string())
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct Configuration {
    pub oes_settings: OESAppSettings,
    pub hmi: HmiConfig,
    pub nats: NatsConfiguration,
}

impl Configuration {
    pub fn new() -> Self {
        let cfg = load_config();
        Self::parse(&cfg).unwrap()
    }

    /// Parse configuration
    fn parse(cfg: &Config) -> Result<Configuration, config::ConfigError> {
        Ok(Configuration {
            oes_settings: cfg.get::<OESAppSettings>("oes-app")?,
            hmi: cfg.get::<HmiConfig>("hmi").unwrap(),
            nats: cfg.get::<NatsConfiguration>("nats").unwrap(),
        })
    }
}
