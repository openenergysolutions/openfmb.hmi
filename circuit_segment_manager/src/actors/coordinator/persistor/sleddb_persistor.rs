use crate::actors::coordinator::persistor::generic_openfmb_device_persistor::GenericOpenFMBDevicePersistorMsg;

use crate::{
    actors::{
        generic_openfmb_device_persistor::GenericOpenFMBDevicePersistor, Coordinator,
        CoordinatorMsg, Device, DeviceMsg, GenericOpenFMBDevice, GenericOpenFMBProfileMsg,
        PersistorMsg, PublisherMsg,
    },
    messages::*,
};
use log::warn;

// use prost::Message;
use riker::actors::*;
// use rocksdb::{IteratorMode, Options, ReadOptions, DB as RocksDB};
use std::{collections::HashMap, path::Path};

use uuid::Uuid;

#[actor(OpenFMBMessage, RequestActorStats)]
#[derive(Debug)]
pub struct SledDBOpenFMBPersistor {
    pub message_count: u32,
    pub openfmb_device_actors: HashMap<Uuid, ActorRef<GenericOpenFMBDevicePersistorMsg>>,
    pub db: sled::Db,
}

impl SledDBOpenFMBPersistor {
    fn get_persisted_message_count(&self) -> u32 {
        let mut message_count = 0;
        let rocksdb_iter = self.db.iter();
        for _it in rocksdb_iter {
            message_count += 1;
            //            cout << it->key().ToString() << ": " << it->value().ToString() << endl;
        }
        message_count
    }

    fn ensure_actor(
        &mut self,
        ctx: &Context<SledDBOpenFMBPersistorMsg>,
        msg: &OpenFMBMessage,
    ) -> ActorRef<GenericOpenFMBDevicePersistorMsg> {
        //dbg!(msg);
        let device_mrid = msg.device_mrid().unwrap();
        let actor = self.openfmb_device_actors.get(&device_mrid);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                //warn!("ensuring device subscriber: {:?}", device_mrid);
                let generation_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBDevicePersistor, Uuid>(
                        &device_mrid.to_string(),
                        device_mrid,
                    );
                let generation_actor_ref = match generation_actor_ref {
                    Ok(actor) => actor,
                    Err(e) => panic!(e),
                };
                self.openfmb_device_actors
                    .insert(device_mrid, generation_actor_ref.clone());
                generation_actor_ref
            }
        }
    }
}

impl Actor for SledDBOpenFMBPersistor {
    type Msg = SledDBOpenFMBPersistorMsg;

    fn pre_start(&mut self, _ctx: &Context<Self::Msg>) {}

    fn post_start(&mut self, _ctx: &Context<Self::Msg>) {}

    fn post_stop(&mut self) {}

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) {
        self.message_count += 1;
        self.receive(ctx, msg, sender);
    }
}

impl SledDBOpenFMBPersistor {}

impl Receive<RequestActorStats> for SledDBOpenFMBPersistor {
    type Msg = SledDBOpenFMBPersistorMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: RequestActorStats, sender: Sender) {
        for (_mrid, device) in self.openfmb_device_actors.clone() {
            device.send_msg(msg.clone().into(), sender.clone());
        }
        let stats_msg: CoordinatorMsg = ActorStats {
            message_count: self.message_count,
            persisted_message_count: Some(self.get_persisted_message_count()),
        }
        .into();
        sender
            .unwrap()
            .try_tell(stats_msg, Some(ctx.myself.clone().into()))
            .unwrap();
    }
}

impl Receive<OpenFMBMessage> for SledDBOpenFMBPersistor {
    type Msg = SledDBOpenFMBPersistorMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: OpenFMBMessage, _sender: Sender) {
        let child_actor = self.ensure_actor(ctx, &msg);
        child_actor.send_msg(msg.into(), Some(ctx.myself.clone().into()))
    }
}

// impl Receive<OpenFMBMessage> for RocksDBOpenFMBPersistor {
//     type Msg = RocksDBOpenFMBPersistorMsg;
//     fn receive(&mut self, ctx: &Context<Self::Msg>, msg: OpenFMBMessage, sender: Sender) {
//         let child_actor = self.ensure_actor(ctx, &msg);
//         child_actor.send_msg(msg.into(), Some(ctx.myself.clone().into()))
//         //        self.openfmb_device.clone().unwrap().send_msg(msg.into(), ctx.myself.clone())
//     }
// }
