// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

#![recursion_limit = "256"]
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::ToSocketAddrs;
use std::sync::Arc;

use tokio::sync::RwLock;

use warp::Filter;

use hmi_server::{
    Publisher, Subscriber,
    auth::*,
    configuration::{Configuration, logging::setup_logger},
    handler::*,
    processor::Processor,
};

#[tokio::main]
async fn main() {
    server_setup().await;
}

async fn server_setup() {
    let configuration = Configuration::new();
    let log_settings = configuration.oes_settings.clone();
    let _ = setup_logger(log_settings);

    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    let mut publisher = Publisher::new(configuration.nats.clone());
    publisher.start().await;

    let processor = Processor::new(publisher, clients.clone());
    let mut subscriber = Subscriber::new(configuration.nats.clone(), processor.clone());

    tokio::spawn(async move {
        if let Err(e) = subscriber.start().await {
            log::error!("Unable to start subscriber: {}", e);
        }
    });

    let login_routes = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(login_handler);

    let user_profile = warp::path("profile")
        .and(warp::get())
        .and(with_auth(Role::Viewer))
        .and_then(profile_handler);

    let get_users = warp::path("get-users")
        .and(warp::get())
        .and(with_auth(Role::Admin))
        .and_then(get_users_handler);

    let delete_user = warp::path("delete-user")
        .and(warp::post())
        .and(with_auth(Role::Admin))
        .and(warp::body::json())
        .and_then(delete_user_handler);

    let update_user = warp::path("update-user")
        .and(warp::post())
        .and(with_auth(Role::Admin))
        .and(warp::body::json())
        .and_then(update_user_handler);

    let create_user = warp::path("create-user")
        .and(warp::post())
        .and(with_auth(Role::Admin))
        .and(warp::body::json())
        .and_then(create_user_handler);

    let save = warp::path("save-diagram");
    let save_routes = save
        .and(warp::post())
        .and(warp::body::json())
        .and_then(save_handler);

    let delete = warp::path("delete-diagram");
    let delete_routes = delete
        .and(warp::post())
        .and(warp::body::json())
        .and_then(delete_handler);

    let list = warp::path("get-diagrams");

    let list_routes = list.and(warp::get()).and_then(list_handler);

    let design = warp::path("get-diagram");
    let design_routes = design
        .and(warp::get())
        .and(warp::query())
        .and_then(diagram_handler);

    let update = warp::path!("update-data")
        .and(warp::body::json())
        .and(with_processor(processor.clone()))
        .and_then(data_handler);

    let data_route = warp::path("data")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and_then(connect_handler);

    let equipment_routes = warp::path("equipment-list")
        .and(warp::get())
        .and(with_auth(Role::Admin))
        .and_then(equipment_handler);

    let delete_equipment = warp::path("delete-equipment")
        .and(warp::post())
        .and(with_auth(Role::Admin))
        .and(warp::body::json())
        .and_then(delete_equipment_handler);

    let update_equipment = warp::path("update-equipment")
        .and(warp::post())
        .and(with_auth(Role::Admin))
        .and(warp::body::json())
        .and_then(update_equipment_handler);

    let create_equipment = warp::path("create-equipment")
        .and(warp::post())
        .and(with_auth(Role::Admin))
        .and(warp::body::json())
        .and_then(create_equipment_handler);

    let cors = warp::cors()
        .allow_methods(vec!["POST", "GET", "OPTIONS"])
        .allow_any_origin()
        .allow_headers(vec![
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "content-type",
            "content-length",
            "upgrade",
            "authorization",
            "Accept",
        ]);

    let static_dir = configuration.hmi.client_dist_dir();

    log::info!("Serving static files from: {}", static_dir);

    let index = format!("{}/index.html", static_dir.clone());

    let static_route = warp::fs::dir(static_dir);
    let is_spa = true;

    // These filters are needed so that when hit "F5" on browser, pages are refreshed correctly
    let home = warp::path("home")
        .and_then(move || async move {
            if is_spa {
                Ok(is_spa)
            } else {
                Err(warp::reject::not_found())
            }
        })
        .and(warp::fs::file(index.clone()))
        .map(|_, file| file);

    let hmi = warp::path("hmi")
        .and_then(move || async move {
            if is_spa {
                Ok(is_spa)
            } else {
                Err(warp::reject::not_found())
            }
        })
        .and(warp::fs::file(index.clone()))
        .map(|_, file| file);

    let diagrams = warp::path("diagrams")
        .and_then(move || async move {
            if is_spa {
                Ok(is_spa)
            } else {
                Err(warp::reject::not_found())
            }
        })
        .and(warp::fs::file(index.clone()))
        .map(|_, file| file);

    let data_connect = warp::path("data-connect")
        .and_then(move || async move {
            if is_spa {
                Ok(is_spa)
            } else {
                Err(warp::reject::not_found())
            }
        })
        .and(warp::fs::file(index.clone()))
        .map(|_, file| file);

    let designer = warp::path("designer")
        .and_then(move || async move {
            if is_spa {
                Ok(is_spa)
            } else {
                Err(warp::reject::not_found())
            }
        })
        .and(warp::fs::file(index.clone()))
        .map(|_, file| file);

    let inspector = warp::path("inspector")
        .and_then(move || async move {
            if is_spa {
                Ok(is_spa)
            } else {
                Err(warp::reject::not_found())
            }
        })
        .and(warp::fs::file(index.clone()))
        .map(|_, file| file);

    let settings = warp::path("settings")
        .and_then(move || async move {
            if is_spa {
                Ok(is_spa)
            } else {
                Err(warp::reject::not_found())
            }
        })
        .and(warp::fs::file(index.clone()))
        .map(|_, file| file);

    let sessions = warp::path("sessions")
        .and_then(move || async move {
            if is_spa {
                Ok(is_spa)
            } else {
                Err(warp::reject::not_found())
            }
        })
        .and(warp::fs::file(index.clone()))
        .map(|_, file| file);

    let routes = static_route
        .or(home)
        .or(hmi)
        .or(diagrams)
        .or(data_connect)
        .or(designer)
        .or(inspector)
        .or(settings)
        .or(sessions)
        .or(login_routes)
        .or(user_profile)
        .or(get_users)
        .or(delete_user)
        .or(update_user)
        .or(create_user)
        .or(save_routes)
        .or(delete_routes)
        .or(list_routes)
        .or(equipment_routes)
        .or(delete_equipment)
        .or(update_equipment)
        .or(create_equipment)
        .or(design_routes)
        .or(data_route)
        .or(update)
        .with(cors)
        .with(warp::log("warp::server"));

    let host = configuration.hmi.server_host();

    let ssl_cert = configuration.hmi.ssl_cert();
    let ssl_key = configuration.hmi.ssl_key();

    if !ssl_cert.is_empty() && !ssl_key.is_empty() {
        let port = configuration.hmi.server_port();
        let server_uri = format!("{}:{}", host, port);

        warp::serve(routes)
            .tls()
            .cert_path(ssl_cert)
            .key_path(ssl_key)
            .run(server_uri.to_socket_addrs().unwrap().next().unwrap())
            .await;
    } else {
        let port = configuration.hmi.server_port();
        let server_uri = format!("{}:{}", host, port);

        warp::serve(routes)
            .run(server_uri.to_socket_addrs().unwrap().next().unwrap())
            .await;
    }
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

fn with_processor(
    process: Processor,
) -> impl Filter<Extract = (Processor,), Error = Infallible> + Clone {
    warp::any().map(move || process.clone())
}
