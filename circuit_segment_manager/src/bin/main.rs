use riker::actor::Tell;
use log::{debug, error, info, warn};
use circuit_segment_manager::util::init::{microgrid_setup, Error, setup_logger};
use std::thread::sleep;
use riker::system::ActorSystem;
use circuit_segment_manager::actors::{SystemEventLog, Publisher, Persistor, Microgrid, Device, Coordinator};
use circuit_segment_manager::actors::{PublisherMsg,PersistorMsg, MicrogridMsg, DeviceMsg, CoordinatorMsg};
use circuit_segment_manager::actors::coordinator::subscriber::subscriber::SubscriberMsg;
use circuit_segment_manager::util::init::Error::ActorConfigError;
use std::time::Duration;
use snafu::ResultExt;
use riker::actors::ActorRefFactory;
use nats::Connection;
use riker::actor::{ActorRef, ActorSelectionFactory};
use circuit_segment_manager::actors::subscriber::subscriber::Subscriber;
use circuit_segment_manager::messages::{StartProcessing, RequestActorStats};
use config::Config;

// extern crate termion;

fn main() -> Result<(), Error> {
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

//    Ok((sys, coordinator_actor, microgrid));

    // let device = sys
    //     .select("/user/Subscriber/OpenFMBFileSubscriber/RegulatorHandler/28c0c3be-8f2e-4bea-bd21-2a9b62f7fbd4")
    //     .unwrap();
    //

    // sleep(std::time::Duration::from_secs(100000));
    //
     Ok(())
}
