// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use log::{error, info, warn};
use riker::actors::*;
use std::str::FromStr;

use config::Config;

/// An actor that is meant to log all of the actor system events
/// to a text log
#[derive(Default)]
pub struct SystemEventLog;

impl Actor for SystemEventLog {
    type Msg = SystemEvent;

    fn pre_start(&mut self, ctx: &Context<Self::Msg>) {
        let sub = Box::new(ctx.myself());
        ctx.system.sys_events().tell(
            Subscribe {
                actor: sub,
                topic: "*".into(),
            },
            None,
        );
    }

    fn recv(&mut self, _ctx: &Context<Self::Msg>, _msg: Self::Msg, _sender: Sender) {
        unreachable!();
    }

    fn sys_recv(&mut self, _ctx: &Context<Self::Msg>, msg: SystemMsg, _sender: Sender) {
        match msg {
            SystemMsg::Event(SystemEvent::ActorCreated(msg)) => {
                info!("ACTOR CREATED {}", msg.actor.uri());
            }
            SystemMsg::Event(SystemEvent::ActorRestarted(msg)) => {
                error!("ACTOR RESTARTED {}", msg.actor.uri());
            }
            SystemMsg::Event(SystemEvent::ActorTerminated(msg)) => {
                error!("ACTOR TERMINATED {}", msg.actor.uri());
            }
            other => {
                warn!("unhandled sys msg {:?}", other);
            }
        }
    }
}

pub fn setup_logger(config: &Config) -> Result<(), fern::InitError> {
    use fern::colors::{Color, ColoredLevelConfig};

    let s = config
        .get_str("coordinator.log-level")
        .unwrap_or("INFO".into());
    let level = log::LevelFilter::from_str(&s).unwrap_or(log::LevelFilter::Info);

    // configure colors for the whole line
    let colors_level = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::White)
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
        .level(level)
        .chain(fern::log_file("output.log")?);

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

    dispatch.apply()?;

    Ok(())
}
