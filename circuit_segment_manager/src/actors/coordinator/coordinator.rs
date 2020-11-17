pub use super::subscriber::*;
use crate::{
    actors::{
        coordinator::subscriber::subscriber::SubscriberMsg, Device, DeviceMsg, Persistor,
        PersistorMsg, Publisher, PublisherMsg,
    },
    messages::{ActorRefWrap, ActorStats, PrintTree, RequestActorStats, StartProcessing},
    traits::CircuitSegmentActor,
};
use log::info;

use riker::actors::*;

///The Coordinator is the root actor in any process. It receives all messages, collects basic statistics,
/// and routes them to more specific actors, which in turn might delegate to even more specific ones
#[actor(StartProcessing, RequestActorStats, ActorStats)]
#[derive(Debug, Clone)]
pub struct Coordinator {
    pub device_count: u32,
    pub message_count: u32,
    pub publisher: ActorRef<PublisherMsg>,
    pub subscriber: ActorRef<SubscriberMsg>,
    pub persistor: ActorRef<PersistorMsg>,
    pub device: ActorRef<DeviceMsg>,
}

impl
    ActorFactoryArgs<(
        ActorRef<PublisherMsg>,
        ActorRef<SubscriberMsg>,
        ActorRef<PersistorMsg>,
        ActorRef<DeviceMsg>,
    )> for Coordinator
{
    fn create_args(
        args: (
            ActorRef<PublisherMsg>,
            ActorRef<SubscriberMsg>,
            ActorRef<PersistorMsg>,
            ActorRef<DeviceMsg>,
        ),
    ) -> Self {
        Coordinator {
            device_count: 0,
            message_count: 0,
            publisher: args.0,
            subscriber: args.1,
            persistor: args.2,
            device: args.3,
        }
    }
}

impl CircuitSegmentActor for Coordinator {
    fn get_message_count(&self) -> u32 {
        self.message_count
    }
}

impl Coordinator {
    pub fn get_child_actors(
        &mut self,
    ) -> (
        ActorRef<PublisherMsg>,
        ActorRef<SubscriberMsg>,
        ActorRef<PersistorMsg>,
        ActorRef<DeviceMsg>,
    ) {
        (
            self.publisher.clone(),
            self.subscriber.clone(),
            self.persistor.clone(),
            self.device.clone(),
        )
    }
}

impl Actor for Coordinator {
    type Msg = CoordinatorMsg;

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Sender) {
        //        info!("coordinator received msg {:?}", msg.clone());
        self.message_count += 1;
        self.receive(ctx, msg, sender);
    }

    fn supervisor_strategy(&self) -> Strategy {
        Strategy::Restart
    }

    ///This  sets up the child actors as each of them can, in turn, recursively
    fn pre_start(&mut self, ctx: &Context<Self::Msg>) {
        //        self.publisher = Some(ctx.actor_of(Props::new(Publisher::actor), "Publisher").unwrap());
        //        self.persistor = Some(ctx.actor_of(Props::new(Persistor::actor), "Persistor").unwrap());
        //        self.device = Some(
        //            ctx.actor_of(
        //                Props::new_args(Processor::actor, self.publisher.clone().unwrap()),
        //                "Processor",
        //            )
        //            .unwrap(),
        //        );
        //
        //        self.subscriber = Some(
        //            ctx.actor_of(
        //                Props::new_args(
        //                    Subscriber::actor,
        //                    (
        //                        self.device.clone().unwrap(),
        //                        self.publisher.clone().unwrap(),
        //                        self.persistor.clone().unwrap(),
        //                    ),
        //                ),
        //                "Subscriber",
        //            )
        //            .unwrap(),
        //        );
        self.device.tell(
            ActorRefWrap(self.publisher.clone()),
            Some(ctx.myself.clone().into()),
        );

        //        self.device
        //            .tell(ActorRefWrap(self.persistor.clone()), Some(ctx.myself.clone().into()));

        //        self.subscriber.unwrap()
    }

    fn post_start(&mut self, _ctx: &Context<Self::Msg>) {}
}

impl Receive<PrintTree> for Coordinator {
    type Msg = CoordinatorMsg;

    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: PrintTree, _sender: Sender) {}
}

impl Receive<ActorStats> for Coordinator {
    type Msg = CoordinatorMsg;

    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: ActorStats, sender: Sender) {
        //print!("message count: {:8}", msg.message_count);
        match msg.persisted_message_count {
            Some(count) => print!(", persisted: {:8}: ", count),
            None => {}
        }
        info!(": {}", sender.unwrap().path())
    }
}

impl Receive<RequestActorStats> for Coordinator {
    type Msg = CoordinatorMsg;

    fn receive(&mut self, ctx: &Context<Self::Msg>, _msg: RequestActorStats, _sender: Sender) {
        ctx.myself.tell(
            ActorStats {
                message_count: self.message_count,
                persisted_message_count: None,
            },
            Some(ctx.myself.clone().into()),
        );
        self.subscriber
            .tell(RequestActorStats, Some(ctx.myself.clone().into()));
        self.publisher
            .tell(RequestActorStats, Some(ctx.myself.clone().into()));
        self.device
            .tell(RequestActorStats, Some(ctx.myself.clone().into()));
        self.persistor
            .tell(RequestActorStats, Some(ctx.myself.clone().into()));
    }
}

impl Receive<StartProcessing> for Coordinator {
    type Msg = CoordinatorMsg;

    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: StartProcessing, _sender: Sender) {
        self.subscriber.tell(StartProcessing, None);
    }
}

use microgrid_protobuf::MicrogridControl;
impl Receive<MicrogridControl> for Coordinator {
    type Msg = CoordinatorMsg;

    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: MicrogridControl, _sender: Sender) {
        panic!();
        //        self.subscriber.tell(MicrogridControl, None);
    }
}
