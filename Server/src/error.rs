// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("wrong credentials")]
    WrongCredentialsError,
    #[error("failed to get jwks")]
    GetJWKError,
    #[error("jwk parse error")]
    ParseJWKError,
    #[error("could not extract kid from jwk")]
    ExtractJWKKidError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("jwt token creation error")]
    JWTTokenCreationError,
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("no permission")]
    NoPermissionError,
    #[error("add user failed")]
    AddUserError,
    #[error("add device failed")]
    AddDeviceError,
}

impl warp::reject::Reject for Error {}
