// use super::rocksdb_persistor::{RocksDBOpenFMBPersistor, RocksDBOpenFMBPersistorMsg};

use crate::{actors::CoordinatorMsg, messages::*};

use riker::actors::*;

#[actor(RequestActorStats, OpenFMBMessage)]
#[derive(Clone, Debug, Default)]
pub struct Persistor {
    message_count: u32,
    openfmb_profile: Option<()>, //Option<ActorRef<RocksDBOpenFMBPersistorMsg>>,
}

impl Actor for Persistor {
    type Msg = PersistorMsg;

    fn pre_start(&mut self, _ctx: &Context<Self::Msg>) {
        // self.openfmb_profile = Some(
        //     ctx.actor_of(Props::new(RocksDBOpenFMBPersistor::actor), "RocksDBPersistor")
        //         .unwrap(),
        // );
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

impl Receive<RequestActorStats> for Persistor {
    type Msg = PersistorMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, _msg: RequestActorStats, sender: Sender) {
        // self.openfmb_profile
        //     .as_ref()
        //     .unwrap()
        //     .send_msg(msg.clone().into(), sender.clone());
        //        self.rust_processor
        //            .clone()
        //            .unwrap()
        //            .send_msg(msg.clone().into(), sender.clone());
        //        self.wasm_processor
        //            .clone()
        //            .unwrap()
        //            .send_msg(msg.clone().into(), sender.clone());
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

impl Receive<OpenFMBMessage> for Persistor {
    type Msg = PersistorMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: OpenFMBMessage, _sender: Sender) {
        //dbg!(msg);

        // self.openfmb_profile
        //     .as_ref()
        //     .unwrap()
        //     .send_msg(msg.into(), ctx.myself.clone())
    }
}
