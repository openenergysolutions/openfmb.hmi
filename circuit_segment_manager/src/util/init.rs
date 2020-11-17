use crate::{
    actors::{coordinator::subscriber::subscriber::Subscriber, *},
    messages::*,
};
use log::{debug, error, info, warn};
use riker::actors::*;
use snafu::{OptionExt, ResultExt, Snafu};

use crate::actors::coordinator::subscriber::subscriber::SubscriberMsg;
use std::{fmt::format, io, io::prelude::*, path::PathBuf, time::Duration};

pub fn microgrid_setup() -> Result<
    (
        ActorSystem,
        ActorRef<CoordinatorMsg>,
        ActorRef<MicrogridMsg>,
    ),
    Error,
> {
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
        .context(ActorConfigError)?;
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
    // let gui = match sys.config().get_bool("coordinator.cursive_ui_enabled") {
    //     Ok(true) => Some(
    //         sys.actor_of_args::<CursiveUI, (ActorRef<PublisherMsg>, ActorRef<MicrogridMsg>)>(
    //             "CursiveUI",
    //             (publisher.clone(), microgrid.clone()),
    //         )
    //         .unwrap(),
    //     ),
    //     Ok(false) => None,
    //     Err(err) => {
    //         warn!(
    //             "cursive_ui_enabled option not set, error returned: {:?}",
    //             err
    //         );
    //         None
    //     }
    // };
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

    Ok((sys, coordinator_actor, microgrid))
}

use config::Config;
use nats::Connection;
use std::{
    borrow::Borrow,
    io::{stdin, stdout, Write},
    thread::sleep,
};

pub fn setup_logger(config: &Config) -> Result<(), fern::InitError> {
    use fern::colors::{Color, ColoredLevelConfig};

    // configure colors for the whole line
    let colors_level = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        // we actually don't need to specify the color for debug and info, they are white by default
        .info(Color::Green)
        .debug(Color::White)
        // depending on the terminals color scheme, this is the same as the background color
        .trace(Color::Blue);

    let mut dispatch = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(fern::log_file("output.log")?)
        .level_for(
            "circuit_segment_coordinator::actors::coordinator::microgrid",
            log::LevelFilter::Warn,
        )
        .chain(fern::log_file("microgrid_warn.log")?)
        .level_for(
            "circuit_segment_coordinator::actors::coordinator::microgrid",
            log::LevelFilter::Debug,
        )
        .chain(fern::log_file("microgrid_debug.log")?);
    if !config
        .get_bool("coordinator.cursive_ui_enabled")
        .unwrap_or(false)
    {
        dispatch = dispatch
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "{date}][{target}][{level}] {message}\x1B[0m",
                    date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    target = record.target(),
                    level = colors_level.color(record.level()),
                    message = message,
                ));
            })
            .chain(std::io::stdout());
    }
    dispatch.apply()?;

    Ok(())
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Actor System Error"))]
    ActorSystemError { source: riker::system::SystemError },
    #[snafu(display("IO Error"))]
    IOError { source: std::io::Error },
    #[snafu(display("Actor Creation Error"))]
    ActorCreateError { source: riker::actor::CreateError },
    #[snafu(display("Actor Configuration Error"))]
    ActorConfigError { source: config::ConfigError },
}
