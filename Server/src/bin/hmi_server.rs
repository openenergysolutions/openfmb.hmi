use log::Level;
use log::info;
use riker::actor::Tell;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{RwLock};
use warp::{Filter};
use circuit_segment_manager::actors::{Coordinator};
use hmi_server::handler::*;
use static_dir::static_dir;
use warp::http::Method;
use circuit_segment_manager::util::init::{Error, setup_logger};
use std::thread::sleep;
use futures::executor::block_on;
use riker::system::ActorSystem;
use riker::actor::{ActorRef, ActorRefFactory, ActorSelectionFactory};
use circuit_segment_manager::actors::{SystemEventLog, Publisher, Persistor, Microgrid};
use circuit_segment_manager::util::init::Error::ActorConfigError;
use std::time::Duration;
use nats::Connection;
use config::{ConfigError,Config};
use circuit_segment_manager::actors::coordinator::subscriber::subscriber::Subscriber;
use circuit_segment_manager::actors::PublisherMsg;
use circuit_segment_manager::actors::PersistorMsg;
use circuit_segment_manager::actors::MicrogridMsg;
// use circuit_segment_coordinator::actors::DeviceMsg;
use circuit_segment_manager::messages::{StartProcessing, RequestActorStats};
use circuit_segment_manager::actors::subscriber::subscriber::SubscriberMsg;
use circuit_segment_manager::actors::CoordinatorMsg;
use circuit_segment_manager::actors::openfmb::OpenFMBDeviceMsg;
use circuit_segment_manager::actors::wasm_processor::WASMProcessorMsg;
use riker::actors::actor;
use circuit_segment_manager::messages::OpenFMBMessage;
use circuit_segment_manager::actors::PublisherRefWrap;
use riker::actors::Receive;
use riker::actors::Context;
use circuit_segment_manager::actors::Device;
use circuit_segment_manager::actors::DeviceMsg;

const ASSETS: inpm::Dir = inpm::include_package!("Client/dist/openfmb-hmi/");

// #[actor(RequestActorStats, OpenFMBMessage, PublisherRefWrap)]
// #[derive(Clone, Debug)]
// pub struct Device {
//     message_count: u32,
//     openfmb_device: Option<ActorRef<OpenFMBDeviceMsg>>,
//     wasm_processor: Option<ActorRef<WASMProcessorMsg>>,
//     circuit_segment_coordinator: ActorRef<MicrogridMsg>,
//     publisher: ActorRef<PublisherMsg>,
//     persistor: ActorRef<PersistorMsg>,
// }


#[tokio::main]
async fn main() {
    let future = run(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
}

async fn run() {
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


    hmi_setup();

    warp::serve(routes).run(([127, 0, 0, 1], 32771)).await;

}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

pub fn hmi_setup()  {
    let config = riker::load_config();

    setup_logger(&config).unwrap();

    //Create the actor system that will manage all of the actors instantiate during runtime
    let sys = ActorSystem::with_config("coordinator", config.clone()).unwrap();

    // System event logger before other actors are started
    sys.actor_of::<SystemEventLog>("SysEventLogger").unwrap();

    let nats_server_uri = match config.get_str("coordinator.environment").unwrap().as_str() {
        "prod" => config.get_str("openfmb_nats_subscriber.prod_uri").unwrap(),
        "dev" => config.get_str("openfmb_nats_subscriber.dev_uri").unwrap(),
        err => panic!("unsupported environment name: {}", err),
    };
    let nats_client = nats::connect(&nats_server_uri).unwrap();
    info!("Connected to NATS server");

    let app_name = sys
        .config()
        .get_str("coordinator.app_name").unwrap();
   // std::thread::sleep(Duration::from_millis(1000));
    let publisher = sys
        .actor_of_args::<Publisher, Connection>("Publisher", nats_client.clone())
        .unwrap();
    let persistor = sys.actor_of::<Persistor>("Persistor").unwrap();
    let microgrid = sys
        .actor_of_args::<Microgrid, (Config, ActorRef<PublisherMsg>, ActorRef<PersistorMsg>)>(
            "Microgrid",
            (sys.config().clone(), publisher.clone(), persistor.clone()),
        )
        .unwrap();
    let processor = sys
        .actor_of_args::<Device, (
            ActorRef<PublisherMsg>,
            ActorRef<PersistorMsg>,
            ActorRef<MicrogridMsg>,
        )>(
            "Device",
            (publisher.clone(), persistor.clone(), microgrid.clone()),
        )
        .unwrap();

    let subscriber = sys
        .actor_of_args::<Subscriber, (
            ActorRef<PublisherMsg>,
            ActorRef<DeviceMsg>,
            ActorRef<PersistorMsg>,
            ActorRef<MicrogridMsg>,
            Connection,
        )>(
            "Subscriber",
            (
                publisher.clone(),
                processor.clone(),
                persistor.clone(),
                microgrid.clone(),
                nats_client,
            ),
        )
        .unwrap();
    // let props = Props::new_args(
    //     (publisher, subscriber, persistor, processor, gui),
    // );
    //The Coordinator is the root of our user actor tree, and is the only one we ever instantiate directly
    let coordinator_actor = sys
        .actor_of_args::<Coordinator, (
            ActorRef<PublisherMsg>,
            ActorRef<SubscriberMsg>,
            ActorRef<PersistorMsg>,
            ActorRef<DeviceMsg>,
        )>(&app_name, (publisher, subscriber, persistor, processor))
        .unwrap();

    //    info!("Coordinator name: {}\n", coordinator_actor.path());
    //    let actor: BasicActorRef = coordinator_actor.clone().into();
    let coordinator = sys.select("/user/CircuitSegmentCoordinator").unwrap();
    std::thread::sleep(Duration::from_millis(500));
    let start_processing_msg: CoordinatorMsg = StartProcessing.into();
    let _request_actor_stats_msg: CoordinatorMsg = RequestActorStats.into();
    // std::thread::sleep(Duration::from_millis(2000));
    //    coordinator.try_tell(start_processing_msg, Some(sys.user_root().clone()));
    coordinator_actor.tell(start_processing_msg, Some(sys.user_root().clone()));
    // std::thread::sleep(Duration::from_millis(4000));
    //let basic:BasicActorRef = coordinator_actor.clone().into();
    //basic.try_tell(ActorStats{message_count: 3}, Some(sys.user_root().clone()));
    //    coordinator_actor.tell(ActorStats { message_count: 3 }, Some(sys.user_root().clone()));
    let request_actor_stats_msg: CoordinatorMsg = RequestActorStats.into();
    coordinator.try_tell(request_actor_stats_msg, Some(sys.user_root().clone()));
    ()
    //Ok((sys, coordinator_actor, microgrid));
}
