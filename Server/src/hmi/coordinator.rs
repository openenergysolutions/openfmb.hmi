// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use config::Config;
use lazy_static::lazy_static;
use log::info;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;
use std::sync::RwLock;
use std::time::Duration;

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(riker::load_config());
}

#[derive(Clone, Debug)]
pub struct StartProcessingMessages {
    pub pubsub_options: CoordinatorOptions,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Environment {
    Dev = 0,
    Prod = 1,
}

impl TryFrom<i32> for Environment {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Environment::Dev as i32 => Ok(Environment::Dev),
            x if x == Environment::Prod as i32 => Ok(Environment::Prod),
            _ => Err(()),
        }
    }
}

impl FromStr for Environment {
    type Err = ();

    fn from_str(input: &str) -> Result<Environment, Self::Err> {
        match input {
            "dev" => Ok(Environment::Dev),
            "prod" => Ok(Environment::Prod),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Authentication {
    None = 0,
    UserPwd = 1,
    Token = 2,
    Creds = 3,
}

impl TryFrom<i32> for Authentication {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Authentication::None as i32 => Ok(Authentication::None),
            x if x == Authentication::UserPwd as i32 => Ok(Authentication::UserPwd),
            x if x == Authentication::Token as i32 => Ok(Authentication::Token),
            x if x == Authentication::Creds as i32 => Ok(Authentication::Creds),
            _ => Err(()),
        }
    }
}

impl FromStr for Authentication {
    type Err = ();

    fn from_str(input: &str) -> Result<Authentication, Self::Err> {
        match input {
            "none" => Ok(Authentication::None),
            "user_pwd" => Ok(Authentication::UserPwd),
            "token" => Ok(Authentication::Token),
            "creds" => Ok(Authentication::Creds),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Authentication {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Security {
    None = 0,
    TlsServer = 1,
    TlsMutual = 2,
}

impl TryFrom<i32> for Security {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Security::None as i32 => Ok(Security::None),
            x if x == Security::TlsServer as i32 => Ok(Security::TlsServer),
            x if x == Security::TlsMutual as i32 => Ok(Security::TlsMutual),
            _ => Err(()),
        }
    }
}

impl FromStr for Security {
    type Err = ();

    fn from_str(input: &str) -> Result<Security, Self::Err> {
        match input {
            "none" => Ok(Security::None),
            "tls_server" => Ok(Security::TlsServer),
            "tls_mutual" => Ok(Security::TlsMutual),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Security {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoordinatorStatus {
    pub env: Environment,
    pub connected: bool,
    pub coordinator_active: bool,
    pub comm_ok: bool,
    pub server_id: String,
    pub send_status_update: bool,
}

#[derive(Clone, Debug)]
pub struct CoordinatorOptions {
    pub connection_url: String,
    pub authentication_type: Authentication,
    pub env: Environment,
    pub token: Option<String>,
    pub creds_file: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub security_type: Security,
    pub root_cert: Option<String>,
    pub client_cert: Option<String>,
    pub client_key: Option<String>,
}

impl CoordinatorOptions {
    pub fn new() -> CoordinatorOptions {
        let env = Environment::from_str(
            SETTINGS
                .read()
                .unwrap()
                .get_str("hmi.environment")
                .unwrap()
                .as_str(),
        )
        .unwrap();
        let _ = SETTINGS
            .write()
            .unwrap()
            .set_default("nats.connected", true);

        CoordinatorOptions::get_options(env)
    }

    fn get_options(env: Environment) -> CoordinatorOptions {
        let nats_server_uri = match env {
            Environment::Prod => {
                let _ = SETTINGS.write().unwrap().set("hmi.environment", "prod");
                SETTINGS.read().unwrap().get_str("nats.prod_uri").unwrap()
            }
            Environment::Dev => {
                let _ = SETTINGS.write().unwrap().set("hmi.environment", "dev");
                SETTINGS.read().unwrap().get_str("nats.dev_uri").unwrap()
            }
        };

        let mut authentication_type = Authentication::None;

        if let Ok(s) = SETTINGS.read().unwrap().get_str("nats.authentication_type") {
            if let Ok(sec) = Authentication::from_str(&s) {
                authentication_type = sec;
            } else {
                log::error!("Invalid authentication type {}", s);
            }
        }

        let mut security_type = Security::None;

        if let Ok(s) = SETTINGS.read().unwrap().get_str("nats.security_type") {
            if let Ok(sec) = Security::from_str(&s) {
                security_type = sec;
            } else {
                log::error!("Invalid security type {}", s);
            }
        }

        let mut options = CoordinatorOptions {
            connection_url: nats_server_uri,
            authentication_type: authentication_type.clone(),
            env: env,
            token: None,
            creds_file: None,
            username: None,
            password: None,
            security_type: security_type,
            client_cert: None,
            client_key: None,
            root_cert: None,
        };

        if let Ok(s) = SETTINGS.read().unwrap().get_str("nats.token") {
            if s.len() > 0 {
                options.token = Some(s);
            }
        }
        if let Ok(s) = SETTINGS.read().unwrap().get_str("nats.creds") {
            if s.len() > 0 {
                options.creds_file = Some(s);
            }
        }
        if let Ok(s) = SETTINGS.read().unwrap().get_str("nats.username") {
            if s.len() > 0 {
                options.username = Some(s);
            }
        }
        if let Ok(s) = SETTINGS.read().unwrap().get_str("nats.password") {
            if s.len() > 0 {
                options.password = Some(s);
            }
        }
        if let Ok(s) = SETTINGS.read().unwrap().get_str("nats.client_cert") {
            if s.len() > 0 {
                options.client_cert = Some(s);
            }
        }
        if let Ok(s) = SETTINGS.read().unwrap().get_str("nats.client_key") {
            if s.len() > 0 {
                options.client_key = Some(s);
            }
        }
        if let Ok(s) = SETTINGS.read().unwrap().get_str("nats.root_cert") {
            if s.len() > 0 {
                options.root_cert = Some(s);
            }
        }

        return options;
    }

    pub fn toggle_environment() -> CoordinatorOptions {
        let mut env = Environment::from_str(
            SETTINGS
                .read()
                .unwrap()
                .get_str("hmi.environment")
                .unwrap()
                .as_str(),
        )
        .unwrap();
        if env == Environment::Prod {
            env = Environment::Dev;
        } else {
            env = Environment::Prod;
        }

        CoordinatorOptions::get_options(env)
    }

    pub fn options(&self) -> std::io::Result<nats::Options> {
        let options = match self.authentication_type {
            Authentication::UserPwd => {
                info!("NATS with default username/pwd");
                nats::Options::with_user_pass(
                    self.username.as_ref().unwrap(),
                    self.password.as_ref().unwrap(),
                )
            }
            Authentication::Token => {
                info!("NATS options with token: {:?}", self.token);
                nats::Options::with_token(self.token.as_ref().unwrap())
            }
            Authentication::Creds => {
                info!("NATS options with credential file: {:?}", self.creds_file);
                nats::Options::with_credentials(self.creds_file.as_ref().unwrap())
            }
            Authentication::None => {
                info!("NATS with default option");
                nats::Options::new()
            }
        };

        match self.security_type {
            Security::TlsServer => {
                Ok(options.add_root_certificate(self.root_cert.as_ref().unwrap()))
            }
            Security::TlsMutual => Ok(options
                .add_root_certificate(self.root_cert.as_ref().unwrap())
                .client_cert(
                    self.client_cert.as_ref().unwrap(),
                    self.client_key.as_ref().unwrap(),
                )),
            Security::None => Ok(options),
        }
    }

    pub fn current_status() -> CoordinatorStatus {
        let mut send_update_status =
            match SETTINGS.read().unwrap().get_bool("nats.send_status_update") {
                Ok(b) => b,
                _ => false,
            };

        let server_id = match SETTINGS.read().unwrap().get_str("nats.server_id") {
            Ok(b) => b,
            _ => {
                if send_update_status {
                    log::error!("Missing server id.  Will not send pub/sub status updates.");
                    send_update_status = false;
                }
                "".to_string()
            }
        };

        CoordinatorStatus {
            env: Environment::from_str(
                SETTINGS
                    .read()
                    .unwrap()
                    .get_str("hmi.environment")
                    .unwrap()
                    .as_str(),
            )
            .unwrap(),
            connected: SETTINGS.read().unwrap().get_bool("nats.connected").unwrap(),
            send_status_update: send_update_status,
            server_id: server_id,
            coordinator_active: SETTINGS
                .read()
                .unwrap()
                .get_bool("hmi.active")
                .unwrap_or(false),
            comm_ok: SETTINGS
                .read()
                .unwrap()
                .get_bool("hmi.comm_ok")
                .unwrap_or(false),
        }
    }

    pub fn update_coordinator_status(is_active: bool, overall_comm: bool) {
        match SETTINGS.write().unwrap().set("hmi.active", is_active) {
            Ok(_) => {}
            Err(e) => {
                log::error!("Unable to write to configuration: {:?}", e);
            }
        }

        match SETTINGS.write().unwrap().set("hmi.comm_ok", overall_comm) {
            Ok(_) => {}
            Err(e) => {
                log::error!("Unable to write to configuration: {:?}", e);
            }
        }
    }

    pub fn server_id() -> Option<String> {
        match SETTINGS.read().unwrap().get_str("nats.server_id") {
            Ok(b) => Some(b),
            _ => None,
        }
    }

    pub fn on_disconnect() {
        log::error!("Connection to pub/sub broker has lost!");
        let _ = SETTINGS.write().unwrap().set("nats.connected", false);
    }

    pub fn on_reconnect() {
        log::info!("Reconnected to pub/sub broker.");
        let _ = SETTINGS.write().unwrap().set("nats.connected", true);
    }

    pub fn on_delay_reconnect(c: usize) -> Duration {
        Duration::from_millis(std::cmp::min((c * 100) as u64, 10000))
    }

    pub fn on_error(_err: std::io::Error) {
        log::error!("Connection to pub/sub broker throws error");
    }

    pub fn on_closed() {
        log::info!("Connection to pub/sub broker has been closed!");
    }
}
