use crate::{actors::CoordinatorMsg, messages::*};

use riker::actors::*;

#[actor(RequestActorStats, OpenFMBMessage)]
#[derive(Clone, Debug, Default)]
pub struct Algorithm {
    message_count: u32,
}

impl Actor for Algorithm {
    type Msg = AlgorithmMsg;

    fn pre_start(&mut self, _ctx: &Context<Self::Msg>) {}

    fn post_start(&mut self, _ctx: &Context<Self::Msg>) {}

    fn post_stop(&mut self) {}

    fn supervisor_strategy(&self) -> Strategy {
        Strategy::Restart
    }

    fn sys_recv(
        &mut self,
        _ctx: &Context<Self::Msg>,
        _msg: SystemMsg,
        _sender: Option<BasicActorRef>,
    ) {
    }

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) {
        self.message_count += 1;
        self.receive(ctx, msg, sender);
    }
}

impl Receive<RequestActorStats> for Algorithm {
    type Msg = AlgorithmMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, _msg: RequestActorStats, sender: Sender) {
        let stats_msg: CoordinatorMsg = ActorStats {
            message_count: self.message_count,
            persisted_message_count: None,
        }
        .into();
        sender
            .unwrap()
            .try_tell(stats_msg, Some(ctx.myself.clone().into()))
            .unwrap();
    }
}

impl Receive<OpenFMBMessage> for Algorithm {
    type Msg = AlgorithmMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: OpenFMBMessage, _sender: Sender) {}
}
