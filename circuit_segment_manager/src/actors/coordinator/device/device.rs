use super::{
    openfmb::openfmb::{OpenFMBDevice, OpenFMBDeviceMsg},
    wasm_processor::{WASMProcessor, WASMProcessorMsg},
};

use crate::actors::microgrid::MicrogridMsg;

use crate::{
    actors::{coordinator::CoordinatorMsg, Persistor, PersistorMsg, PublisherMsg},
    messages::*,
};
use chrono::NaiveDateTime;

use crate::actors::Publisher;
use openfmb_messages::commonmodule::Timestamp;
use riker::actors::*;

#[actor(RequestActorStats, OpenFMBMessage, PublisherRefWrap)]
#[derive(Clone, Debug)]
pub struct Device {
    message_count: u32,
    openfmb_device: Option<ActorRef<OpenFMBDeviceMsg>>,
    wasm_processor: Option<ActorRef<WASMProcessorMsg>>,
    circuit_segment_coordinator: ActorRef<MicrogridMsg>,
    publisher: ActorRef<PublisherMsg>,
    persistor: ActorRef<PersistorMsg>,
}

impl
    ActorFactoryArgs<(
        ActorRef<PublisherMsg>,
        ActorRef<PersistorMsg>,
        ActorRef<MicrogridMsg>,
    )> for Device
{
    fn create_args(
        args: (
            ActorRef<PublisherMsg>,
            ActorRef<PersistorMsg>,
            ActorRef<MicrogridMsg>,
        ),
    ) -> Self {
        Device {
            message_count: 0,
            openfmb_device: None,
            wasm_processor: None,
            circuit_segment_coordinator: args.2,
            publisher: args.0,
            persistor: args.1,
        }
    }
}

impl Actor for Device {
    type Msg = DeviceMsg;

    fn pre_start(&mut self, ctx: &Context<Self::Msg>) {
        self.openfmb_device = Some(
            ctx.actor_of_args::<OpenFMBDevice, (ActorRef<PublisherMsg>, ActorRef<PersistorMsg>)>(
                "OpenFMBDevice",
                (self.publisher.clone(), self.persistor.clone()),
            )
            .unwrap(),
        );
        self.wasm_processor = Some(
            ctx.actor_of_args::<WASMProcessor, ActorRef<PublisherMsg>>(
                "WASMProcessor",
                self.publisher.clone(),
            )
            .unwrap(),
        );
    }

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) {
        self.message_count += 1;
        self.receive(ctx, msg, sender);
    }
}

pub fn openfmb_ts_to_timestamp(ts: &Timestamp) -> NaiveDateTime {
    match NaiveDateTime::from_timestamp_opt(ts.seconds as i64, ts.nanoseconds) {
        Some(date) => date,
        None => NaiveDateTime::from_timestamp(ts.seconds as i64, 0),
    }
}

impl Receive<RequestActorStats> for Device {
    type Msg = DeviceMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: RequestActorStats, sender: Sender) {
        self.openfmb_device
            .clone()
            .unwrap()
            .send_msg(msg.clone().into(), sender.clone());
        self.wasm_processor
            .clone()
            .unwrap()
            .send_msg(msg.clone().into(), sender.clone());
        self.circuit_segment_coordinator
            .clone()
            .send_msg(msg.clone().into(), sender.clone());
        let _stats_msg: CoordinatorMsg = ActorStats {
            message_count: self.message_count,
            persisted_message_count: None,
        }
        .into();
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

//impl<T: Clone + Send + std::fmt::Debug> Receive<ActorRefWrap<T>> for Processor {
//    type Msg = DeviceMsg;
//    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: ActorRefWrap<T>, sender: Sender) {
//        self.rust_processor
//            .clone()
//            .unwrap()
//            .send_msg(msg.into(), ctx.myself.clone())
//    }
//}

//impl Receive<ActorRefWrap<PersistorMsg>> for Processor {
//    type Msg = DeviceMsg;
//    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: ActorRefWrap<PersistorMsg>, sender: Sender) {
//        self.persistor = msg.0;
//        //        self.rust_processor
//        //            .clone()
//        //            .unwrap()
//        //            .send_msg(msg.into(), ctx.myself.clone())
//    }
//}

pub type PublisherRefWrap = ActorRefWrap<PublisherMsg>;

impl Receive<ActorRefWrap<PublisherMsg>> for Device {
    type Msg = DeviceMsg;
    fn receive(
        &mut self,
        _ctx: &Context<Self::Msg>,
        msg: ActorRefWrap<PublisherMsg>,
        _sender: Sender,
    ) {
        self.publisher = msg.clone().0;
        //        self.rust_processor
        //            .clone()
        //            .unwrap()
        //            .send_msg(msg.into(), ctx.myself.clone())
        //dbg!(msg);
    }
}

impl Receive<OpenFMBMessage> for Device {
    type Msg = DeviceMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: OpenFMBMessage, _sender: Sender) {
        self.openfmb_device
            .as_ref()
            .unwrap()
            .send_msg(msg.clone().into(), ctx.myself.clone());

        self.circuit_segment_coordinator
            .send_msg(msg.into(), ctx.myself.clone())
    }
}
