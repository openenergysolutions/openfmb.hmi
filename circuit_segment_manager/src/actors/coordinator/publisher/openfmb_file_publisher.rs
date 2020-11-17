use crate::{actors::coordinator::CoordinatorMsg, messages::*};

use riker::actors::*;

#[actor(RequestActorStats)]
#[derive(Clone, Default, Debug)]
pub struct OpenFMBFilePublisher {
    message_count: u32,
    //openfmb_file_subscriber: Option<OpenFMBFileSubscriber>,
}

impl Actor for OpenFMBFilePublisher {
    type Msg = OpenFMBFilePublisherMsg;

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

impl Receive<RequestActorStats> for OpenFMBFilePublisher {
    type Msg = OpenFMBFilePublisherMsg;
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
