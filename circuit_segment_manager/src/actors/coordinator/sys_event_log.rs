use log::{error, info, warn};
use riker::actors::*;

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
