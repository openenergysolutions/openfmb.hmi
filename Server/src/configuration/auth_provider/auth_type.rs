// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub enum AuthType {
    #[default]
    NIL,
    KEYCLOAK,
}

impl From<Option<String>> for AuthType {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(val) => match val.to_lowercase().as_str() {
                "keycloak" => AuthType::KEYCLOAK,
                _ => AuthType::NIL,
            },
            None => AuthType::NIL,
        }
    }
}
