// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::convert::TryFrom;
use std::str::FromStr;
use std::sync::RwLock;
use config::Config;
use lazy_static::lazy_static;
use std::fmt;
use serde::{Deserialize, Serialize};
use log::info;

lazy_static! {
	static ref SETTINGS: RwLock<Config> = RwLock::new(riker::load_config());    
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Environment {
    Dev = 0,
    Prod = 1
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
    type Err  = ();

    fn from_str(input: &str) -> Result<Environment, Self::Err> {
        match input {
            "dev"  => Ok(Environment::Dev),
            "prod"  => Ok(Environment::Prod),            
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
    type Err  = ();

    fn from_str(input: &str) -> Result<Authentication, Self::Err> {
        match input {
            "none"  => Ok(Authentication::None),
            "user_pwd"  => Ok(Authentication::UserPwd), 
            "token"  => Ok(Authentication::Token),
            "creds"  => Ok(Authentication::Creds),                       
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
    type Err  = ();

    fn from_str(input: &str) -> Result<Security, Self::Err> {
        match input {
            "none"  => Ok(Security::None),
            "tls_server"  => Ok(Security::TlsServer), 
            "tls_mutual"  => Ok(Security::TlsMutual),                                  
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
pub struct PubSubStatus {
    pub env: Environment,
    pub connected: bool,
    pub server_id: String,
    pub send_status_update: bool,
}

#[derive(Clone, Debug)]
pub struct PubSubOptions {
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

impl PubSubOptions {

    pub fn new() -> PubSubOptions {
        
        let env = Environment::from_str(SETTINGS.read().unwrap().get_str("coordinator.environment").unwrap().as_str()).unwrap();
        let _ = SETTINGS.write().unwrap().set_default("openfmb_nats_subscriber.connected", true);

        PubSubOptions::get_options(env)
    }

    pub fn get_options(env: Environment) -> PubSubOptions {        
        
        let nats_server_uri = match env {
            Environment::Prod => {
                let _ = SETTINGS.write().unwrap().set("coordinator.environment", "prod");
                SETTINGS.read().unwrap().get_str("openfmb_nats_subscriber.prod_uri").unwrap()
            },
            Environment::Dev => {
                let _ = SETTINGS.write().unwrap().set("coordinator.environment", "dev");
                SETTINGS.read().unwrap().get_str("openfmb_nats_subscriber.dev_uri").unwrap()
            }
        };   
        
        let mut authentication_type = Authentication::None;

        if let Ok(s) = SETTINGS.read().unwrap().get_str("openfmb_nats_subscriber.authentication_type") {
            if let Ok(sec) = Authentication::from_str(&s) {
                authentication_type = sec;
            }
            else {
                log::error!("Invalid authentication type {}", s);
            }
        }

        let mut security_type = Security::None;

        if let Ok(s) = SETTINGS.read().unwrap().get_str("openfmb_nats_subscriber.security_type") {
            if let Ok(sec) = Security::from_str(&s) {
                security_type = sec;
            }
            else {
                log::error!("Invalid security type {}", s);
            }
        }

        let mut options = PubSubOptions {
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

        if let Ok(s) = SETTINGS.read().unwrap().get_str("openfmb_nats_subscriber.token") {
            if s.len() > 0 {
                options.token = Some(s);
            }
        }
        if let Ok(s) = SETTINGS.read().unwrap().get_str("openfmb_nats_subscriber.creds") {
            if s.len() > 0 {
                options.creds_file = Some(s);
            }
        }
        if let Ok(s) = SETTINGS.read().unwrap().get_str("openfmb_nats_subscriber.username") {
            if s.len() > 0 {
                options.username = Some(s);
            }
        }
        if let Ok(s) = SETTINGS.read().unwrap().get_str("openfmb_nats_subscriber.password") {
            if s.len() > 0 {
                options.password = Some(s);
            }
        }        
        if let Ok(s) = SETTINGS.read().unwrap().get_str("openfmb_nats_subscriber.client_cert") {
            if s.len() > 0 {
                options.client_cert = Some(s);
            }
        }
        if let Ok(s) = SETTINGS.read().unwrap().get_str("openfmb_nats_subscriber.client_key") {
            if s.len() > 0 {
                options.client_key = Some(s);
            }
        }
        if let Ok(s) = SETTINGS.read().unwrap().get_str("openfmb_nats_subscriber.root_cert") {
            if s.len() > 0 {
                options.root_cert = Some(s);
            }
        }

        return options;
    }

    pub fn toggle_environment() -> PubSubOptions {
        let mut env = Environment::from_str(SETTINGS.read().unwrap().get_str("coordinator.environment").unwrap().as_str()).unwrap();
        if env == Environment::Prod {
            env = Environment::Dev;
        }
        else {
            env = Environment::Prod;
        }

        PubSubOptions::get_options(env)
    }

    pub fn connect(&self) -> std::io::Result<nats::Connection> {
        let options: nats::Options;

        match self.authentication_type {
            Authentication::UserPwd => {
                info!("NATS with default username/pwd");
                options = nats::Options::with_user_pass(self.username.as_ref().unwrap(), self.password.as_ref().unwrap());
            }
            Authentication::Token => {
                info!("NATS options with token: {:?}", self.token);
                options = nats::Options::with_token(self.token.as_ref().unwrap());
            }
            Authentication::Creds => {
                info!("NATS options with credential file: {:?}", self.creds_file);
                options = nats::Options::with_credentials(self.creds_file.as_ref().unwrap());
            }            
            Authentication::None => {
                info!("NATS with default option");
                options = nats::Options::new();
            }
        }

        match self.security_type {
            Security::TlsServer => {
                options
                    .add_root_certificate(self.root_cert.as_ref().unwrap())
                    .disconnect_callback(|| PubSubOptions::on_disconnect())
                    .reconnect_callback(|| PubSubOptions::on_reconnect())
                    .connect(&self.connection_url)
            }
            Security::TlsMutual => {
                options
                    .add_root_certificate(self.root_cert.as_ref().unwrap())
                    .client_cert(self.client_cert.as_ref().unwrap(), self.client_key.as_ref().unwrap())
                    .disconnect_callback(|| PubSubOptions::on_disconnect())
                    .reconnect_callback(|| PubSubOptions::on_reconnect())
                    .connect(&self.connection_url)
            }
            Security::None => {
                options
                    .disconnect_callback(|| PubSubOptions::on_disconnect())
                    .reconnect_callback(|| PubSubOptions::on_reconnect())
                    .connect(&self.connection_url)
            }
        }                
    }

    pub fn current_status() -> PubSubStatus {
        let mut send_update_status = match SETTINGS.read().unwrap().get_bool("openfmb_nats_subscriber.send_status_update") {
            Ok(b) => b,
            _ => false
        };

        let server_id = match SETTINGS.read().unwrap().get_str("openfmb_nats_subscriber.server_id") {
            Ok(b) => b,
            _ => {
                if send_update_status {
                    log::error!("Missing server id.  Will not send pub/sub status updates.");
                    send_update_status = false;
                }
                "".to_string()
            }
        };        

        PubSubStatus {
            env: Environment::from_str(SETTINGS.read().unwrap().get_str("coordinator.environment").unwrap().as_str()).unwrap(),
            connected: SETTINGS.read().unwrap().get_bool("openfmb_nats_subscriber.connected").unwrap(),
            send_status_update: send_update_status,
            server_id: server_id
        }
    }

    fn on_disconnect() {
        log::debug!("Connection to pub/sub broker has lost!");
        let _ = SETTINGS.write().unwrap().set("openfmb_nats_subscriber.connected", false);
    }

    fn on_reconnect() {
        log::debug!("Reconnected to pub/sub broker.");
        let _ = SETTINGS.write().unwrap().set("openfmb_nats_subscriber.connected", true);
    }
}