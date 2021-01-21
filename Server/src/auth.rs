use crate::{error::Error};
use std::sync::{Arc};
use tokio::sync::{RwLock};
use std::collections::HashMap;
use chrono::prelude::*;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::fmt;
use warp::{
    http::StatusCode,    
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, Filter, Rejection, Reply, reply
};

pub type Result<T> = std::result::Result<T, Rejection>;

const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"openfmbsecrete2@2@";

pub type Users = Arc<RwLock<HashMap<String, User>>>;

#[derive(Clone, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,    
    pub pwd: String,
    pub email: String,
    pub name: String,
    pub role: String,
}

impl User {
    fn empty() -> User {
        User {
            id: String::from(""),
            username: String::from(""),
            pwd: String::from(""),
            email: String::from(""),
            name: String::from(""),
            role: String::from("")
        }
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub pwd: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User
}

#[derive(Clone, PartialEq)]
pub enum Role {    
    Admin,
    Engineer,
    Viewer
}

impl Role {
    pub fn from_str(role: &str) -> Role {
        match role {
            "Admin" => Role::Admin,
            "Engineer" => Role::Engineer,
            _ => Role::Viewer,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Admin => write!(f, "Admin"),
            Role::Engineer => write!(f, "Engineer"),
            Role::Viewer => write!(f, "Viewer"),            
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    name: String,
    role: String,
    exp: usize,
}

pub fn with_auth(role: Role) -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| (role.clone(), headers))
        .and_then(authorize)
}

pub fn create_jwt(uid: &str, name: &str, role: &Role) -> std::result::Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uid.to_owned(),
        name: name.to_owned(),
        role: role.to_string(),
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| Error::JWTTokenCreationError)
}

async fn authorize((role, headers): (Role, HeaderMap<HeaderValue>)) -> std::result::Result<String, Rejection> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(JWT_SECRET),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|_| reject::custom(Error::JWTTokenError))?;

            if role == Role::Admin && Role::from_str(&decoded.claims.role) != Role::Admin {
                return Err(reject::custom(Error::NoPermissionError));
            }

            Ok(decoded.claims.sub)
        }
        Err(e) => return Err(reject::custom(e)),
    }
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> std::result::Result<String, Error> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(Error::NoAuthHeaderError),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(Error::NoAuthHeaderError),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(Error::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}

pub async fn login_handler(body: LoginRequest, users: Users) -> Result<impl Reply> {

    let mut token = String::from("");

    let mut usr = User::empty();

    users
        .read()
        .await
        .iter()
        .filter(|(_, user)| user.username == body.username)
        .filter(|(_, user)| user.pwd == body.pwd)
        .for_each(|(uid, user)| {
            token = create_jwt(&uid, &user.name, &Role::from_str(&user.role))
                .map_err(|e| reject::custom(e)).unwrap(); 
            usr = user.clone();                        
        });

    if token.len() > 0 {
        // Delete password
        usr.pwd = String::from("");
        return Ok(reply::json(&LoginResponse { token: token, user: usr }));
    } 
    
    Err(reject::custom(Error::WrongCredentialsError))
}

pub async fn profile_handler(_id: String) -> Result<impl Reply> {
    Ok(StatusCode::OK)
}
