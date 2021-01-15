use log::info;
use riker::actor::Tell;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{RwLock};
use warp::{Filter};
use circuit_segment_manager::actors::Coordinator;
use hmi_server::{handler::*, auth::*};
use hmi_server::hmi::{hmi::*, hmi_subscriber::*, hmi_publisher::*, processor::*};
use circuit_segment_manager::util::init::setup_logger;
use futures::executor::block_on;
use riker::system::ActorSystem;
use riker::actor::{ActorRef, ActorRefFactory, ActorSelectionFactory};
use circuit_segment_manager::actors::{SystemEventLog, Publisher, Persistor, Microgrid};
use std::time::Duration;
use std::net::ToSocketAddrs;
use nats::Connection;
use config::Config;
use circuit_segment_manager::actors::coordinator::subscriber::subscriber::Subscriber;
use circuit_segment_manager::actors::PublisherMsg;
use circuit_segment_manager::actors::PersistorMsg;
use circuit_segment_manager::actors::MicrogridMsg;
use circuit_segment_manager::messages::{StartProcessing, RequestActorStats};
use circuit_segment_manager::actors::subscriber::subscriber::SubscriberMsg;
use circuit_segment_manager::actors::CoordinatorMsg;
use circuit_segment_manager::actors::Device;
use circuit_segment_manager::actors::DeviceMsg;

//const ASSETS: inpm::Dir = inpm::include_package!("Client/dist/openfmb-hmi/");

#[tokio::main]
async fn main() {    
        
    let future = server_setup(); 
    block_on(future); 
}

async fn server_setup() {   
    
    let clients : Clients = Arc::new(RwLock::new(HashMap::new()));

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
        .get_str("coordinator.app_name")
        .unwrap();
    std::thread::sleep(Duration::from_millis(1000));
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
                nats_client.clone(),
            ),
        )
        .unwrap();
    
    //The Coordinator is the root of our user actor tree, and is the only one we ever instantiate directly
    let coordinator_actor = sys
        .actor_of_args::<Coordinator, (
            ActorRef<PublisherMsg>,
            ActorRef<SubscriberMsg>,
            ActorRef<PersistorMsg>,
            ActorRef<DeviceMsg>,
        )>(&app_name, (publisher.clone(), subscriber, persistor.clone(), processor))
        .unwrap();
   
    let coordinator = sys.select("/user/CircuitSegmentCoordinator").unwrap();
    std::thread::sleep(Duration::from_millis(500));
    let start_processing_msg: CoordinatorMsg = StartProcessing.into();        
    coordinator_actor.tell(start_processing_msg, Some(sys.user_root().clone()));    
    let request_actor_stats_msg: CoordinatorMsg = RequestActorStats.into();
    coordinator.try_tell(request_actor_stats_msg, Some(sys.user_root().clone()));

    // Start Hmi related 
    
    let publisher = sys
        .actor_of_args::<HmiPublisher, (Connection, Config)>("HmiPublisher", (nats_client.clone(), sys.config().clone()))
        .unwrap();

    let processor = sys
        .actor_of_args::<Processor,( 
            ActorRef<HmiPublisherMsg>, 
            Clients,
        )>(
            "HmiProcessor",
            (
                publisher.clone(),
                clients.clone(),
            )       
        )
        .unwrap();

    let subscriber = sys
        .actor_of_args::<HmiSubscriber, (            
            ActorRef<ProcessorMsg>,            
            Connection,
        )>(
            "HmiSubscriber",
            (                
                processor.clone(),                
                nats_client.clone(),
            ),
        )
        .unwrap();           

    let hmi_actor = sys
        .actor_of_args::<Hmi, (
            ActorRef<HmiPublisherMsg>,
            ActorRef<HmiSubscriberMsg>                                                   
        )>("HMI", (publisher.clone(), subscriber.clone()))
        .unwrap();

    let start_processing_msg: HmiMsg = StartProcessing.into();
    hmi_actor.tell(start_processing_msg, Some(sys.user_root().clone()));

    //let static_route = warp::path("static").and(static_dir!("../Client/dist/openfmb-hmi/"));
    //let static_route = inpm::warp::embedded(ASSETS);        

    let users = Arc::new(RwLock::new(init_users()));

    let login_routes = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_users(users.clone()))        
        .and_then(login_handler);

    let user_profile = warp::path("profile")
        .and(warp::get()) 
        .and(with_auth(Role::User))                     
        .and_then(profile_handler);    

    let save = warp::path("save");
    let save_routes = save
        .and(warp::post())
        .and(warp::body::json())        
        .and_then(save_handler);  
        
    let delete = warp::path("delete");
    let delete_routes = delete
        .and(warp::post())
        .and(warp::body::json())        
        .and_then(delete_handler); 

    let list = warp::path("diagrams");     
        
    let list_routes = list
        .and(warp::get())                
        .and_then(list_handler);

    let design = warp::path("diagram");
    let design_routes = design        
        .and(warp::get()) 
        .and(warp::query())      
        .and_then(diagram_handler);

    let update = warp::path!("update-data")
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and(with_processor(processor.clone()))        
        .and_then(data_handler);
    
    let execute = warp::path!("execute-command")
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(execute_command);

    let hmi_route = warp::path("hmi")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and_then(hmi_handler);

    let equipment_list = warp::path("equipment-list");     
        
    let equipment_routes = equipment_list
        .and(warp::get())                
        .and_then(equipment_handler);

    let command_list = warp::path("command-list");     
        
    let command_routes = command_list
        .and(warp::get())                
        .and_then(command_handler);

    let cors = warp::cors()        
        .allow_methods(vec!["POST", "GET"])        
        .allow_any_origin()
        .allow_headers(vec![
            "User-Agent", 
            "Sec-Fetch-Mode", 
            "Referer", 
            "Origin", 
            "Access-Control-Request-Method", 
            "Access-Control-Request-Headers", 
            "content-type",
            "upgrade", 
            "authorization"]);   

    let routes = login_routes
        //.or(static_route)
        .or(user_profile)
        .or(save_routes)
        .or(delete_routes)
        .or(list_routes)
        .or(equipment_routes)
        .or(command_routes)
        .or(design_routes)
        .or(hmi_route)
        .or(update)
        .or(execute)        
        .with(cors)
        .with(warp::log("openfmb"));  
     
    let mut server_uri = "0.0.0.0:32771".to_string();
    
    if let Ok(ip) = config.get_str("hmi.server_uri") {
        server_uri = ip.clone();
    }    

    warp::serve(routes).run(server_uri.to_socket_addrs().unwrap().next().unwrap()).await;

}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

fn with_users(users: Users) -> impl Filter<Extract = (Users,), Error = Infallible> + Clone {
    warp::any().map(move || users.clone())
}

fn with_processor(process: ActorRef<ProcessorMsg>) -> impl Filter<Extract = (ActorRef<ProcessorMsg>,), Error = Infallible> + Clone {
    warp::any().map(move || process.clone())
}

fn init_users() -> HashMap<String, User> {
    let mut map = HashMap::new();
    map.insert(
        String::from("1"),
        User {
            id: String::from("1"),
            username: String::from("admin"),
            email: String::from(""),
            pwd: String::from("1234"),
            name: String::from("Cory Nguyen"),
            role: String::from("Admin"),
        },
    );
    map.insert(
        String::from("2"),
        User {
            id: String::from("2"),
            username: String::from("hmi"),
            email: String::from(""),
            pwd: String::from("4321"),
            name: String::from("HMI User"),
            role: String::from("User"),
        },
    );
    map
}
