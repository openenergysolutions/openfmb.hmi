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

use hmi_server::coordinator::StartProcessingMessages;
use hmi_server::logs::{setup_logger, SystemEventLog};

use hmi_server::hmi::{
    coordinator::*, hmi::*, hmi_publisher::*, hmi_subscriber::*, monitor::*, processor::*,
};
use hmi_server::{auth::*, handler::*};

use riker::actor::Tell;
use riker::actor::{ActorRef, ActorRefFactory};
use riker::system::ActorSystem;

use config::Config;

#[tokio::main]
async fn main() {
    server_setup().await;
}

async fn server_setup() {
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    let config = riker::load_config();

    setup_logger(&config).unwrap();

    //Create the actor system that will manage all of the actors instantiate during runtime
    let sys = ActorSystem::with_config("coordinator", config.clone()).unwrap();

    // System event logger before other actors are started
    sys.actor_of::<SystemEventLog>("SysEventLogger").unwrap();

    // Start Hmi related

    let publisher = sys
        .actor_of_args::<HmiPublisher, Config>("HmiPublisher", sys.config().clone())
        .unwrap();

    let processor = sys
        .actor_of_args::<Processor, (ActorRef<HmiPublisherMsg>, Clients)>(
            "HmiProcessor",
            (publisher.clone(), clients.clone()),
        )
        .unwrap();

    let subscriber = sys
        .actor_of_args::<HmiSubscriber, ActorRef<ProcessorMsg>>("HmiSubscriber", processor.clone())
        .unwrap();
    if let Ok(send_status_update) = config.get_bool("nats.send_status_update") {
        if send_status_update {
            let _monitor = sys
                .actor_of_args::<Monitor, ActorRef<ProcessorMsg>>("HmiMonitor", processor.clone())
                .unwrap();
        }
    }

    let hmi_actor = sys
        .actor_of_args::<Hmi, (ActorRef<HmiPublisherMsg>, ActorRef<HmiSubscriberMsg>)>(
            "HMI",
            (publisher.clone(), subscriber.clone()),
        )
        .unwrap();

    let run_mode = StartProcessingMessages {
        pubsub_options: CoordinatorOptions::new(),
    };

    let start_processing_msg: HmiMsg = run_mode.into();
    hmi_actor.tell(start_processing_msg, Some(sys.user_root().clone()));

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
        .and(with_hmi(hmi_actor.clone()))
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

    let static_dir = "Client/dist/openfmb-hmi/";
    let index = "Client/dist/openfmb-hmi/index.html";

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
        .and(warp::fs::file(index))
        .map(|_, file| file);

    let hmi = warp::path("hmi")
        .and_then(move || async move {
            if is_spa {
                Ok(is_spa)
            } else {
                Err(warp::reject::not_found())
            }
        })
        .and(warp::fs::file(index))
        .map(|_, file| file);

    let diagrams = warp::path("diagrams")
        .and_then(move || async move {
            if is_spa {
                Ok(is_spa)
            } else {
                Err(warp::reject::not_found())
            }
        })
        .and(warp::fs::file(index))
        .map(|_, file| file);

    let data_connect = warp::path("data-connect")
        .and_then(move || async move {
            if is_spa {
                Ok(is_spa)
            } else {
                Err(warp::reject::not_found())
            }
        })
        .and(warp::fs::file(index))
        .map(|_, file| file);

    let designer = warp::path("designer")
        .and_then(move || async move {
            if is_spa {
                Ok(is_spa)
            } else {
                Err(warp::reject::not_found())
            }
        })
        .and(warp::fs::file(index))
        .map(|_, file| file);

    let inspector = warp::path("inspector")
        .and_then(move || async move {
            if is_spa {
                Ok(is_spa)
            } else {
                Err(warp::reject::not_found())
            }
        })
        .and(warp::fs::file(index))
        .map(|_, file| file);

    let settings = warp::path("settings")
        .and_then(move || async move {
            if is_spa {
                Ok(is_spa)
            } else {
                Err(warp::reject::not_found())
            }
        })
        .and(warp::fs::file(index))
        .map(|_, file| file);

    let sessions = warp::path("sessions")
        .and_then(move || async move {
            if is_spa {
                Ok(is_spa)
            } else {
                Err(warp::reject::not_found())
            }
        })
        .and(warp::fs::file(index))
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

    let host = config
        .get_str("hmi.server_host")
        .unwrap_or("0.0.0.0".to_string());

    let ssl_cert = config.get_str("hmi.ssl_cert").unwrap_or("".to_string());
    let ssl_key = config.get_str("hmi.ssl_key").unwrap_or("".to_string());

    if ssl_cert.len() > 0 && ssl_key.len() > 0 {
        let port = config.get_int("hmi.server_port").unwrap_or(443);
        let server_uri = format!("{}:{}", host, port);

        warp::serve(routes)
            .tls()
            .cert_path(ssl_cert)
            .key_path(ssl_key)
            .run(server_uri.to_socket_addrs().unwrap().next().unwrap())
            .await;
    } else {
        let port = config.get_int("hmi.server_port").unwrap_or(80);
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
    process: ActorRef<ProcessorMsg>,
) -> impl Filter<Extract = (ActorRef<ProcessorMsg>,), Error = Infallible> + Clone {
    warp::any().map(move || process.clone())
}

fn with_hmi(
    hmi: ActorRef<HmiMsg>,
) -> impl Filter<Extract = (ActorRef<HmiMsg>,), Error = Infallible> + Clone {
    warp::any().map(move || hmi.clone())
}
