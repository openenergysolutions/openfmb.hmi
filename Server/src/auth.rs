// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0
extern crate alcoholic_jwt;
use crate::error::Error;
use alcoholic_jwt::{token_kid, validate, JWKS};
use chrono::prelude::*;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use log::error;
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    http::StatusCode,
    reject, reply,
    reply::json,
    Filter, Rejection, Reply,
};

pub type JsonValue = serde_json::Value;

pub type Result<T> = std::result::Result<T, Rejection>;

const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"openfmbsecrete2@2@";

pub type Users = Arc<RwLock<HashMap<String, User>>>;

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub pwd: String,
    pub displayname: String,
    pub role: String,
}

impl User {
    fn empty() -> User {
        User {
            id: String::from(""),
            username: String::from(""),
            pwd: String::from(""),
            displayname: String::from(""),
            role: String::from(""),
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
    pub user: User,
}

#[derive(Clone, PartialEq)]
pub enum Role {
    SuperUser,
    Engineer,
    Viewer,
}

impl Role {
    pub fn from_str(role: &str) -> Role {
        match role {
            "SuperUser" => Role::SuperUser,
            "Engineer" => Role::Engineer,
            _ => Role::Viewer,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::SuperUser => write!(f, "SuperUser"),
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

async fn authorize(
    (role, headers): (Role, HeaderMap<HeaderValue>),
) -> std::result::Result<String, Rejection> {
    match jwt_from_header(&headers) {
        Ok(jwt) => valid_jwt(&jwt, role),
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

fn hash_password(password: &str) -> String {
    match bcrypt::hash(password) {
        Ok(s) => s,
        Err(e) => {
            panic!("Unable to hash password: {}", e)
        }
    }
}

fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash)
}

fn get_jwks(uri: &str) -> Result<JWKS> {
    match reqwest::get(uri) {
        Ok(mut res) => {
            let v = res.json::<JWKS>();

            if let Err(_) = v {
                return Err(reject::custom(Error::ParseJWKError));
            }

            Ok(v.unwrap())
        }
        Err(_) => Err(reject::custom(Error::GetJWKError)),
    }
}

fn extract_kid(jwt: &str) -> Result<String> {
    match token_kid(&jwt) {
        Ok(kid) if kid.is_some() => Ok(kid.unwrap()),
        _ => Err(reject::custom(Error::ExtractJWKKidError)),
    }
}

fn check_role(mut claims: JsonValue, role: Role) -> Result<String> {
    match claims["http://oes.com//roles"].as_array_mut() {
        Some(v) => {
            let roles: Vec<Role> = v
                .iter()
                .map(|v| Role::from_str(v.as_str().unwrap()))
                .collect();

            if role == Role::SuperUser && !roles.contains(&Role::SuperUser) {
                return Err(reject::custom(Error::NoPermissionError));
            }

            if let Some(sub) = claims["sub"].as_str() {
                Ok(String::from(sub))
            } else {
                Err(reject::custom(Error::JWTTokenError))
            }
        }
        None => {
            return Err(reject::custom(Error::JWTTokenError));
        }
    }
}

fn valid_jwt(jwt: &str, role: Role) -> Result<String> {
    let auth = env::var("AUTHORITY").expect("AUTHORITY must be set!");
    let aud = env::var("AUDIENCE").expect("AUDIENCE must be set!");

    let jwks = get_jwks(&format!("{}{}", auth.as_str(), ".well-known/jwks.json"))?;

    let validations = vec![
        alcoholic_jwt::Validation::Issuer(auth),
        alcoholic_jwt::Validation::Audience(aud),
        alcoholic_jwt::Validation::SubjectPresent,
    ];

    let kid = extract_kid(&jwt)?;
    let jwk = jwks
        .find(&kid)
        .ok_or(reject::custom(Error::WrongCredentialsError))?;

    match validate(&jwt, jwk, validations) {
        Ok(jwt) => check_role(jwt.claims, role),
        _ => Err(reject::custom(Error::JWTTokenError)),
    }
}

pub async fn login_handler(body: LoginRequest) -> Result<impl Reply> {
    let users = Arc::new(RwLock::new(init_users()));

    let mut token = String::from("");

    let mut usr = User::empty();

    users
        .read()
        .await
        .iter()
        .filter(|(_, user)| user.username == body.username)
        .filter(|(_, user)| verify_password(&body.pwd, &user.pwd))
        .for_each(|(uid, user)| {
            token = create_jwt(&uid, &user.displayname, &Role::from_str(&user.role))
                .map_err(|e| reject::custom(e))
                .unwrap();
            usr = user.clone();
        });

    if token.len() > 0 {
        // Delete password
        usr.pwd = String::from("");
        return Ok(reply::json(&LoginResponse {
            token: token,
            user: usr,
        }));
    }

    Err(reject::custom(Error::WrongCredentialsError))
}

pub async fn profile_handler(_id: String) -> Result<impl Reply> {
    Ok(StatusCode::OK)
}

fn get_user_file() -> String {
    let app_dir = std::env::var("APP_DIR_NAME").unwrap_or_else(|_| "".into());
    if app_dir != "" {
        return format!("/{}/users.json", app_dir);
    }
    "users.json".to_string()
}

pub fn init_users() -> HashMap<String, User> {
    let file = get_user_file();

    if !Path::new(&file).exists() {
        let mut users: Vec<User> = vec![];

        users.push(User {
            id: String::from("e2a1eaff-c4ea-4f28-bd59-d88fc2882f39"),
            username: String::from("admin"),
            pwd: hash_password("hm1admin"),
            displayname: String::from("Administrator"),
            role: String::from("Admin"),
        });

        let _ = save_user_list(file.clone(), &users);
    }
    load_users(file).unwrap()
}

fn load_users(file_path: String) -> std::io::Result<HashMap<String, User>> {
    let mut map: HashMap<String, User> = HashMap::new();
    let users: Vec<User> = get_user_list(file_path).unwrap();

    for usr in users.iter() {
        map.insert(usr.id.clone(), usr.clone());
    }

    Ok(map)
}

fn save_user_list(file_path: String, users: &Vec<User>) -> std::io::Result<()> {
    let json = serde_json::to_string(&users).unwrap();
    fs::write(file_path, json).expect("Unable to write file");

    Ok(())
}

fn get_user_list(file_path: String) -> std::io::Result<Vec<User>> {
    let map: Vec<User> = vec![];

    if let Ok(mut file) = File::open(file_path.clone()) {
        let mut contents = String::new();
        if let Ok(_) = file.read_to_string(&mut contents) {
            let users: Vec<User> =
                serde_json::from_str(&contents).expect("User json file was not well-formatted");
            return Ok(users);
        } else {
            error!("Unable to read user file: {}", file_path);
        }
    } else {
        error!("Unable to open user file: {}", file_path);
    }

    Ok(map)
}

pub async fn get_users_handler(_id: String) -> Result<impl Reply> {
    let mut list = get_user_list(get_user_file()).unwrap();
    for usr in list.iter_mut() {
        usr.pwd.clear();
    }

    Ok(json(&list))
}

pub async fn delete_user_handler(_id: String, user: User) -> Result<impl Reply> {
    let mut list = get_user_list(get_user_file()).unwrap();

    if let Some(pos) = list.iter().position(|x| *x.id == user.id) {
        list.remove(pos);

        let _ = save_user_list(get_user_file(), &list);
    }
    for usr in list.iter_mut() {
        usr.pwd.clear();
    }
    Ok(json(&list))
}

pub async fn update_user_handler(_id: String, user: User) -> Result<impl Reply> {
    let mut list = get_user_list(get_user_file()).unwrap();

    if let Some(pos) = list.iter().position(|x| *x.id == user.id) {
        let mut usr = list.get_mut(pos).unwrap();
        usr.displayname = user.displayname;
        usr.role = user.role;
        usr.pwd = hash_password(&user.pwd);
        let _ = save_user_list(get_user_file(), &list);
    }
    for usr in list.iter_mut() {
        usr.pwd.clear();
    }
    Ok(json(&list))
}

pub async fn create_user_handler(_id: String, user: User) -> Result<impl Reply> {
    let mut list = get_user_list(get_user_file()).unwrap();
    if let Some(_pos) = list.iter().position(|x| {
        *x.id.to_lowercase() == user.id.to_lowercase()
            || *x.username.to_lowercase() == user.username.to_lowercase()
    }) {
        // same user id/username already exists
        error!(
            "User with same id/username ({}/{}) already exists",
            user.id, user.username
        );

        return Err(reject::custom(Error::AddUserError));
    } else {
        let mut usr = user.clone();
        usr.pwd = hash_password(&user.pwd);
        list.push(usr);
        let _ = save_user_list(get_user_file(), &list);
    }
    for usr in list.iter_mut() {
        usr.pwd.clear();
    }
    Ok(json(&list))
}
