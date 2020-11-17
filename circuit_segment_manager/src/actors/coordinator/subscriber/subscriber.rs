use crate::actors::microgrid::MicrogridMsg;

use crate::actors::coordinator::persistor::PersistorMsg;

use super::super::{Device, DeviceMsg, Publisher, PublisherMsg};

use super::{
    openfmb_nats_subscriber::{OpenFMBNATSSubscriber, OpenFMBNATSSubscriberMsg},
    raw_stdin_subscriber::{RawStdinSubscriber, RawStdinSubscriberMsg},
};
use crate::{actors::CoordinatorMsg, messages::*};
use log::{info, warn};

use nats::Connection;
use riker::actors::*;
use std::fmt::Debug;

#[actor(PrintTree, StartProcessing, RequestActorStats)]
#[derive(Clone, Debug)]
pub struct Subscriber {
    pub message_count: u32,
    pub nats_client: nats::Connection,
    //pub openfmb_file_subscriber: Option<ActorRef<OpenFMBFileSubscriberMsg>>,
    pub openfmb_nats_subscriber: Option<ActorRef<OpenFMBNATSSubscriberMsg>>,
    pub raw_stdin_subscriber: Option<ActorRef<RawStdinSubscriberMsg>>,
    pub publisher: ActorRef<PublisherMsg>,
    pub processor: ActorRef<DeviceMsg>,
    pub microgrid: ActorRef<MicrogridMsg>,
    pub persistor: ActorRef<PersistorMsg>, //    pub publisher: Option<ActorRef<PublisherMsg>>,
}
impl
    ActorFactoryArgs<(
        ActorRef<PublisherMsg>,
        ActorRef<DeviceMsg>,
        ActorRef<PersistorMsg>,
        ActorRef<MicrogridMsg>,
        Connection,
    )> for Subscriber
{
    fn create_args(
        args: (
            ActorRef<PublisherMsg>,
            ActorRef<DeviceMsg>,
            ActorRef<PersistorMsg>,
            ActorRef<MicrogridMsg>,
            Connection,
        ),
    ) -> Self {
        Subscriber {
            message_count: 0,
            nats_client: args.4,
            //openfmb_file_subscriber: None,
            openfmb_nats_subscriber: None,
            raw_stdin_subscriber: None,
            processor: args.1,
            publisher: args.0,
            persistor: args.2,
            microgrid: args.3,
        }
    }
}

impl Subscriber {
    pub fn get_child_actors(
        &mut self,
    ) -> (
        //Option<ActorRef<OpenFMBFileSubscriberMsg>>,
        Option<ActorRef<OpenFMBNATSSubscriberMsg>>,
        Option<ActorRef<RawStdinSubscriberMsg>>,
    ) {
        (
            //    self.openfmb_file_subscriber.clone(),
            self.openfmb_nats_subscriber.clone(),
            self.raw_stdin_subscriber.clone(),
        )
    }
}

impl Actor for Subscriber {
    type Msg = SubscriberMsg;

    fn pre_start(&mut self, ctx: &Context<Self::Msg>) {
        // self.openfmb_file_subscriber = Some(
        //     ctx.actor_of(
        //         Props::new_args(
        //             OpenFMBFileSubscriber::actor,
        //             (self.processor.clone(), self.publisher.clone()),
        //         ),
        //         "OpenFMBFileSubscriber",
        //     )
        //     .unwrap(),
        // );
        self.openfmb_nats_subscriber = Some(
            ctx.actor_of_args::<OpenFMBNATSSubscriber, (
                ActorRef<DeviceMsg>,
                ActorRef<PublisherMsg>,
                ActorRef<MicrogridMsg>,
                Connection,
            )>(
                "OpenFMBNATSSubscriber",
                (
                    self.processor.clone(),
                    self.publisher.clone(),
                    self.microgrid.clone(),
                    self.nats_client.clone(),
                ),
            )
            .unwrap(),
        );
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
        self.receive(ctx, msg.clone(), sender);
    }
}

impl Receive<PrintTree> for Subscriber {
    type Msg = SubscriberMsg;

    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: PrintTree, _sender: Sender) {}
}

impl Receive<StartProcessing> for Subscriber {
    type Msg = SubscriberMsg;

    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: StartProcessing, sender: Sender) {
        info!(
            "subscriber got a start_processing message from {:?}",
            sender
        );
        match sender {
            Some(_sender) => {
                //                info!("sending to sender");
                //                sender.try_tell("bumb", Some(ctx.myself.clone().into())).unwrap()
            }
            None => warn!("no sender"),
        };
        // self.openfmb_file_subscriber
        //     .as_ref()
        //     .unwrap()
        //     .tell(StartProcessing, None);

        self.openfmb_nats_subscriber
            .as_ref()
            .unwrap()
            .tell(StartProcessing, None);
    }
}

impl Receive<RequestActorStats> for Subscriber {
    type Msg = SubscriberMsg;

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

        // match &self.raw_stdin_subscriber {
        //     Some(child) => {
        //         child.tell(msg.clone(), sender.clone());
        //     }
        //     None => todo!(),
        // }
        // match &self.openfmb_file_subscriber {
        //     Some(child) => {
        //         child.tell(msg.clone(), sender.clone());
        //     }
        //     None => {}
        // }
        match &self.openfmb_nats_subscriber {
            Some(child) => {
                child.tell(msg, sender);
            }
            None => {}
        }
        //info!("Publisher has seen {} messages", self.message_count);
    }
}

// impl Receive<LoadControlProfile> for Subscriber {
//     type Msg = SubscriberMsg;
//     fn receive(&mut self, ctx: &Context<Self::Msg>, msg: LoadControlProfile, sender: Sender) {
//         dbg!(msg.clone());
//         //self.openfmb_nats_subscriber.unwrap().send_msg(msg.into(), None);
// //        self.nats_broker.as_ref().unwrap().publish("?", &mut buffer,None).unwrap();
//     }
// }

//impl Receive<ActorRefWrap<ActorRef<Coordinator>>> for Subscriber {
//    type Msg = SubscriberMsg;
//
//    fn receive(&mut self, ctx: &Context<Self::Msg>, _msg: ActorRefWrap<ActorRef<Coordinator>>, sender: Sender) {
//        warn!("receive of CollectActorStats unimplemented");
//        info!("Subscriber has seen {} messages", self.message_count);
//    }
//}

//impl Stats<SubscriberMsg> for Subscriber {
//    fn message_count(&self) -> u32 {
//        info!("new method got called");
//        self.message_count
//    }
//
//    fn receive(&mut self, ctx: &Context<SubscriberMsg>, _msg: RequestActorStats, _sender: Sender) {
//        info!("Publisher has seen {} messages", self.message_count);
//    }
//}
