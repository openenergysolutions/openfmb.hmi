use crate::{
    actors::CoordinatorMsg,
    messages::{
        datetime_from_timestamp, ActorStats, DebugPersistence, OpenFMBCommon, OpenFMBMessage,
        OpenFMBTimestampWrapper, RequestActorStats,
    },
};

use log::error;

use riker::actors::*;
use sled;
// use rocksdb::{
//     IteratorMode, Options as RocksOptions, ReadOptions, DB as RocksDB,
// };
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::fmt::Debug;

use std::{collections::HashMap, path::Path};
use uuid::Uuid;

#[actor(OpenFMBMessage, RequestActorStats)]
#[derive(Debug)]
pub struct GenericOpenFMBDevicePersistor {
    pub message_count: u32,
    pub mrid: Uuid,
    //pub db: sled::Db,
}

impl ActorFactoryArgs<Uuid> for GenericOpenFMBDevicePersistor {
    fn create_args(args: Uuid) -> Self {
        GenericOpenFMBDevicePersistor {
            message_count: 0,
            mrid: args,
            //db: None
        }
    }
}

impl GenericOpenFMBDevicePersistor {}

impl Actor for GenericOpenFMBDevicePersistor {
    type Msg = GenericOpenFMBDevicePersistorMsg;

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) {
        self.message_count += 1;
        self.receive(ctx, msg, sender)
    }
}

impl Receive<RequestActorStats> for GenericOpenFMBDevicePersistor {
    type Msg = GenericOpenFMBDevicePersistorMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, _msg: RequestActorStats, sender: Sender) {
        let stats_msg: CoordinatorMsg = ActorStats {
            message_count: self.message_count,
            persisted_message_count: None,
            //Some(self.get_persisted_message_count()),
        }
        .into();
        sender
            .unwrap()
            .try_tell(stats_msg, Some(ctx.myself.clone().into()))
            .unwrap();
    }
}

#[derive(Serialize, Deserialize)]
struct DateMRIDWrapper {
    date: String,
    mrid: Uuid,
}

impl Receive<OpenFMBMessage> for GenericOpenFMBDevicePersistor {
    type Msg = GenericOpenFMBDevicePersistorMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: OpenFMBMessage, _sender: Sender) {
        //TODO that's a whole lot of lack of error handling
        let datetime = &datetime_from_timestamp(msg.message_timestamp().unwrap());
        let _dt_wrap = DateMRIDWrapper {
            date: datetime.to_rfc2822(),
            mrid: msg.message_mrid().unwrap(),
        };
        // self.db
        //     .put(format!("{:?}", datetime), bincode::serialize(&msg).unwrap())
        //     .unwrap()
    }
}

// impl Receive<DebugPersistence> for GenericOpenFMBDevicePersistor {
//     type Msg = GenericOpenFMBDevicePersistorMsg;
//     fn receive(&mut self, ctx: &Context<Self::Msg>, msg: DebugPersistence, sender: Sender) {
//         dbg!(msg);
//     }
// }
