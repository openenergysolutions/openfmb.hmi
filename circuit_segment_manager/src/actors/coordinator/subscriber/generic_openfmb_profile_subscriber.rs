use crate::actors::coordinator::subscriber::generic_openfmb_device_subscriber::{
    GenericOpenFMBDevicePersistorSubscriber, GenericOpenFMBDevicePersistorSubscriberMsg,
};

use crate::{
    actors::{Coordinator, CoordinatorMsg, Device, DeviceMsg, GenericOpenFMBDevice},
    messages::{ActorStats, OpenFMBMessage, RequestActorStats},
};

// use prost::Message;
use crate::actors::coordinator::persistor::generic_openfmb_device_persistor::GenericOpenFMBDevicePersistor;
use riker::actors::*;
use std::collections::HashMap;
use uuid::Uuid;

#[actor(RequestActorStats, OpenFMBMessage)]
#[derive(Debug, Clone)]
pub struct GenericOpenFMBProfileSubscriber {
    pub message_count: u32,
    pub openfmb_devices: HashMap<Uuid, ActorRef<GenericOpenFMBDevicePersistorSubscriberMsg>>,
    pub device: ActorRef<DeviceMsg>,
}

impl ActorFactoryArgs<ActorRef<DeviceMsg>> for GenericOpenFMBProfileSubscriber {
    fn create_args(args: ActorRef<DeviceMsg>) -> Self {
        GenericOpenFMBProfileSubscriber {
            message_count: 0,
            openfmb_devices: Default::default(),
            device: args,
        }
    }
}

impl GenericOpenFMBProfileSubscriber {
    fn ensure_actor(
        &mut self,
        ctx: &Context<GenericOpenFMBProfileSubscriberMsg>,
        msg: &OpenFMBMessage,
    ) -> ActorRef<GenericOpenFMBDevicePersistorSubscriberMsg> {
        //        dbg!(msg);
        let mrid = msg.device_mrid().unwrap();
        let actor = self.openfmb_devices.get(&mrid);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let generation_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBDevicePersistorSubscriber, ActorRef<DeviceMsg>>(
                        &mrid.to_string(),
                        self.device.clone(),
                    )
                    .unwrap();
                self.openfmb_devices
                    .insert(mrid, generation_actor_ref.clone());
                generation_actor_ref
            }
        }
    }
}

impl Actor for GenericOpenFMBProfileSubscriber {
    type Msg = GenericOpenFMBProfileSubscriberMsg;

    fn pre_start(&mut self, _ctx: &Context<Self::Msg>) {}

    fn post_start(&mut self, _ctx: &Context<Self::Msg>) {}

    fn post_stop(&mut self) {}

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) {
        self.message_count += 1;
        self.receive(ctx, msg, sender);
    }
}

impl GenericOpenFMBProfileSubscriber {}

impl Receive<RequestActorStats> for GenericOpenFMBProfileSubscriber {
    type Msg = GenericOpenFMBProfileSubscriberMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: RequestActorStats, sender: Sender) {
        for (_mrid, device) in self.openfmb_devices.clone() {
            device.send_msg(msg.clone().into(), sender.clone());
        }
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

impl Receive<OpenFMBMessage> for GenericOpenFMBProfileSubscriber {
    type Msg = GenericOpenFMBProfileSubscriberMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: OpenFMBMessage, _sender: Sender) {
        let actor = self.ensure_actor(ctx, &msg);
        actor.send_msg(msg.into(), ctx.myself.clone());
    }
}
