use crate::{actors::coordinator::CoordinatorMsg, messages::*};
use log::info;
use riker::actors::*;

#[actor(TextLine, RequestActorStats)]
#[derive(Clone, Debug)]
pub struct DebugStdoutPublisher {
    message_count: u32,
    //openfmb_file_subscriber: Option<OpenFMBFileSubscriber>,
}

impl ActorFactory for DebugStdoutPublisher {
    fn create() -> Self {
        DebugStdoutPublisher { message_count: 0 }
    }
}

impl Actor for DebugStdoutPublisher {
    type Msg = DebugStdoutPublisherMsg;

    fn pre_start(&mut self, _ctx: &Context<Self::Msg>) {
        //        let subscribers = vec![ctx
        //            .actor_of(Props::new(OpenFMBFileSubscriber::actor), "OpenFMBFileSubscriber")
        //            .unwrap()];
        //        let publisher = vec![ctx
        //            .actor_of(Props::new(OpenFMBFilePublisher::actor), "OpenFMBFilePublisher")
        //            .unwrap()];
    }

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

impl Receive<TextLine> for DebugStdoutPublisher {
    type Msg = DebugStdoutPublisherMsg;

    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: TextLine, _sender: Sender) {
        info!("{}", msg.line);
    }
}

impl Receive<RequestActorStats> for DebugStdoutPublisher {
    type Msg = DebugStdoutPublisherMsg;

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
