// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use openidconnect::core::{CoreClient, CoreProviderMetadata};
use openidconnect::reqwest;
use openidconnect::{ClientId, ClientSecret, IssuerUrl, OAuth2TokenResponse};

#[warn(dead_code)]
pub async fn get_token() -> async_nats::Auth {
    let keycloak_url =
        std::env::var("KEYCLOAK_URL").expect("KEYCLOAK_URL environtment variable missing");
    let client_id = std::env::var("KEYCLOAK_CLIENT_ID")
        .expect("KEYCLOAK_CLIENT_ID environtment variable missing");
    let client_secret = std::env::var("KEYCLOAK_CLIENT_SECRET")
        .expect("KEYCLOAK_CLIENT_SECRET environtment variable missing");

    let http_client = reqwest::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    // Use OpenID Connect Discovery to fetch the provider metadata.
    let issuer = match IssuerUrl::new(keycloak_url) {
        Ok(val) => val,
        Err(e) => {
            log::error!("Keycloak Error: IssuerUrl: {:?}", e);
            return async_nats::Auth::new();
        }
    };
    let provider_metadata = CoreProviderMetadata::discover_async(issuer, &http_client).await;
    let provider_metadata = match provider_metadata {
        Ok(val) => val,
        Err(e) => {
            log::error!("Keycloak Error: Provider Metadata: {:?}", e);
            return async_nats::Auth::new();
        }
    };

    let client = CoreClient::from_provider_metadata(
        provider_metadata.clone(),
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
    );

    let token_request = match client.exchange_client_credentials() {
        Ok(val) => val,
        Err(e) => {
            log::error!("Keycloak Error: Client credentials exchange: {:?}", e);
            return async_nats::Auth::new();
        }
    };

    let token_response = match token_request.request_async(&http_client).await {
        Ok(val) => val,
        Err(e) => {
            log::error!("Keycloak Error: Token request: {:?}", e);
            return async_nats::Auth::new();
        }
    };

    let mut auth = async_nats::Auth::new();
    auth.token = Some(token_response.access_token().secret().clone());
    auth
}
