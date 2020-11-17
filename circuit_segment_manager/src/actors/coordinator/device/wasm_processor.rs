use crate::actors::Publisher;
use crate::{
    actors::{CoordinatorMsg, GenericOpenFMBProfileMsg, PublisherMsg},
    messages::*,
};
use riker::actors::*;
use std::collections::HashMap;

#[actor(RequestActorStats)]
#[derive(Clone, Debug)]
pub struct WASMProcessor {
    message_count: u32,
    openfmb_profile_actors: HashMap<OpenFMBProfileType, ActorRef<GenericOpenFMBProfileMsg>>,
    publisher: ActorRef<PublisherMsg>,
}

impl ActorFactoryArgs<ActorRef<PublisherMsg>> for WASMProcessor {
    fn create_args(args: ActorRef<PublisherMsg>) -> Self {
        WASMProcessor {
            message_count: 0,
            openfmb_profile_actors: Default::default(),
            publisher: args,
        }
    }
}

impl Actor for WASMProcessor {
    type Msg = WASMProcessorMsg;

    fn pre_start(&mut self, _ctx: &Context<Self::Msg>) {
        //        let subscribers = vec![ctx
        //            .actor_of(Props::new(OpenFMBFileSubscriber::actor), "OpenFMBFileSubscriber")
        //            .unwrap()];
        //        let publisher = vec![ctx
        //            .actor_of(Props::new(WASMProcessor::actor), "WASMProcessor")
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

impl Receive<RequestActorStats> for WASMProcessor {
    type Msg = WASMProcessorMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: RequestActorStats, sender: Sender) {
        let stats_msg: CoordinatorMsg = ActorStats {
            message_count: self.message_count,
            persisted_message_count: None,
        }
        .into();
        sender
            .clone()
            .unwrap()
            .try_tell(stats_msg, Some(ctx.myself.clone().into()))
            .unwrap();
        for child in self.openfmb_profile_actors.clone() {
            child.1.send_msg(msg.clone().into(), sender.clone());
        }
        //        match &self.openfmb_file_subscriber {
        //            Some(child) => child.tell(RequestActorStats, sender.clone()),
        //            None => {}
        //        }
        let _stats_msg: CoordinatorMsg = ActorStats {
            message_count: self.message_count,
            persisted_message_count: None,
        }
        .into();
        //sender.clone().unwrap().try_tell(stats_msg, sender.clone());
        //        self.publisher.clone().send_msg(msg.into(), ctx.myself.clone())
    }
}

//impl Receive<Add> for WASMProcessor {
//    type Msg = WASMProcessorMsg;
//
//    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: Add, _sender: Sender) {
//        self.message_count += 1;
//    }
//}
//
//impl Receive<Sub> for WASMProcessor {
//    type Msg = WASMProcessorMsg;
//
//    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: Sub, _sender: Sender) {
//        self.message_count -= 1;
//    }
//}
//
//impl Receive<Print> for WASMProcessor {
//    type Msg = WASMProcessorMsg;
//
//    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: Print, _sender: Sender) {
//        info!("Total counter value: {}", self.message_count);
//    }
//}
