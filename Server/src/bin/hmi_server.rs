

use log::Level;

use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{RwLock};
use warp::{Filter};

use hmi_server::handler::*;
use static_dir::static_dir;
use warp::http::Method;

const ASSETS: inpm::Dir = inpm::include_package!("Client/dist/openfmb-hmi/");



#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    use static_dir::static_dir;
    use warp::Filter;
    //let static_route = warp::path("static").and(static_dir!("../Client/dist/openfmb-hmi/"));
    let static_route = inpm::warp::embedded(ASSETS);
    let clients = Arc::new(RwLock::new(HashMap::new()));    

    let cors = warp::cors()        
        .allow_methods(vec!["POST", "GET", "DELETE"])        
        .allow_any_origin()
        .allow_headers(vec!["User-Agent", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers", "content-type", "authorization"]);

    let save = warp::path("save");
    let save_routes = save
        .and(warp::post())
        .and(warp::body::json())
        .and_then(save_handler);

    let list = warp::path("diagrams");
    let list_routes = list
        .and(warp::get())
        .and_then(list_handler);

    let design = warp::path("diagram");
    let design_routes = design
        //.and(warp::path::param::<String>())
        .and(warp::get())        
        .and_then(design_handler);

    let update = warp::path!("update-data")
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(data_handler);

    let hmi_route = warp::path("hmi")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and_then(hmi_handler);

    let routes = save_routes
        .or(static_route)
        .or(list_routes)
        .or(design_routes)
        .or(hmi_route)
        .or(update)
        .with(cors);



    warp::serve(routes).run(([127, 0, 0, 1], 32771)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
