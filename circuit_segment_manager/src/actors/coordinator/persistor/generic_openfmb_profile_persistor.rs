use crate::actors::{
    coordinator::persistor::generic_openfmb_device_persistor::{
        GenericOpenFMBDevicePersistor, GenericOpenFMBDevicePersistorMsg,
    },
    CoordinatorMsg,
};

use crate::messages::{ActorStats, OpenFMBMessage, RequestActorStats};

use riker::actors::*;
// use rocksdb::{IteratorMode, Options as RocksOptions, ReadOptions, DB as RocksDB};
use std::collections::HashMap;

use std::path::Path;
use uuid::Uuid;

#[actor(OpenFMBMessage, RequestActorStats)]
#[derive(Debug)]
pub struct GenericOpenFMBProfilePersistor {
    pub message_count: u32,
    pub openfmb_device_subscribers: HashMap<Uuid, ActorRef<GenericOpenFMBDevicePersistorMsg>>,
    pub db: sled::Db,
    pub actor_name: String,
}

impl GenericOpenFMBProfilePersistor {
    fn ensure_actor(
        &mut self,
        ctx: &Context<GenericOpenFMBProfilePersistorMsg>,
        msg: &OpenFMBMessage,
    ) -> ActorRef<GenericOpenFMBDevicePersistorMsg> {
        //dbg!(msg);
        let device_mrid = msg.device_mrid().unwrap();
        let actor = self.openfmb_device_subscribers.get(&device_mrid);
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
                self.openfmb_device_subscribers
                    .insert(device_mrid, generation_actor_ref.clone());
                generation_actor_ref
            }
        }
    }

    fn get_persisted_message_count(&self) -> u32 {
        let sled_iter = self.db.iter();
        let mut message_count = 0;
        for _it in sled_iter {
            message_count += 1;
            //            cout << it->key().ToString() << ": " << it->value().ToString() << endl;
        }
        message_count
    }
}

//    pub(crate) fn get_profile_type(profile: String) -> OpenFMBProfileType {
//        match profile.as_str() {
//            "GenerationStatusProfile" | "GenerationReadingProfile" => OpenFMBProfileType::Generation,
//            "SwitchReadingProfile" | "SwitchStatusProfile" => OpenFMBProfileType::Switch,
//            "MeterReadingProfile" => OpenFMBProfileType::Meter,
//            "SolarReadingProfile" | "SolarStatusProfile" => OpenFMBProfileType::Solar,
//            "ESSReadingProfile" | "ESSStatusProfile" => OpenFMBProfileType::ESS,
//            "LoadReadingProfile" | "LoadStatusProfile" => OpenFMBProfileType::Load,
//            "ShuntReadingProfile" | "ShuntStatusProfile" => OpenFMBProfileType::Shunt,
//            "RecloserReadingProfile" | "RecloserStatusProfile" => OpenFMBProfileType::Recloser,
//            _ => panic!("unsupported message profile: {}", profile),
//        }
//    }

impl Actor for GenericOpenFMBProfilePersistor {
    type Msg = GenericOpenFMBProfilePersistorMsg;

    fn pre_start(&mut self, _ctx: &Context<Self::Msg>) {}

    fn sys_recv(
        &mut self,
        _ctx: &Context<Self::Msg>,
        _msg: SystemMsg,
        _sender: Option<BasicActorRef>,
    ) {
        unimplemented!()
    }

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) {
        self.message_count += 1;
        self.receive(ctx, msg, sender)
    }
}

impl Receive<RequestActorStats> for GenericOpenFMBProfilePersistor {
    type Msg = GenericOpenFMBProfilePersistorMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: RequestActorStats, sender: Sender) {
        for (_mrid, device) in self.openfmb_device_subscribers.clone() {
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

impl Receive<OpenFMBMessage> for GenericOpenFMBProfilePersistor {
    type Msg = GenericOpenFMBProfilePersistorMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: OpenFMBMessage, _sender: Sender) {
        let child_actor = self.ensure_actor(ctx, &msg);
        child_actor.send_msg(msg.into(), Some(ctx.myself.clone().into()))
    }
}
