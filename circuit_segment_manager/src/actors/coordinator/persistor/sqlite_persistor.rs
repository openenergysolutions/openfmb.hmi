use riker::actors::*;
//use super::{Persistor, Publisher};

#[actor()]
#[derive(Clone, Default, Debug)]
pub struct SqlitePersistor {
    message_count: u32,
    //openfmb_file_subscriber: Option<OpenFMBFileSubscriber>,
}

impl Actor for SqlitePersistor {
    type Msg = SqlitePersistorMsg;

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

//impl Receive<Add> for SqlitePersistor {
//    type Msg = SqlitePersistorMsg;
//
//    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: Add, _sender: Sender) {
//        self.message_count += 1;
//    }
//}
//
//impl Receive<Sub> for SqlitePersistor {
//    type Msg = SqlitePersistorMsg;
//
//    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: Sub, _sender: Sender) {
//        self.message_count -= 1;
//    }
//}
//
//impl Receive<Print> for SqlitePersistor {
//    type Msg = SqlitePersistorMsg;
//
//    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: Print, _sender: Sender) {
//        info!("Total counter value: {}", self.message_count);
//    }
//}
