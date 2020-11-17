use crate::actors::microgrid::MicrogridMsg;

use super::super::{publisher::PublisherMsg, CoordinatorMsg};
use crate::messages::*;
use log::info;
use riker::actors::*;

#[actor(StartProcessing, RequestActorStats)]
#[derive(Clone, Debug)]
pub struct RawStdinSubscriber {
    message_count: u32,
    publisher: ActorRef<PublisherMsg>,
    microgrid: ActorRef<MicrogridMsg>, //openfmb_file_subscriber: Option<OpenFMBFileSubscriber>,
}

impl Actor for RawStdinSubscriber {
    type Msg = RawStdinSubscriberMsg;

    fn pre_start(&mut self, ctx: &Context<Self::Msg>) {
        self.receive(ctx, StartProcessing, None);
    }

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

impl Receive<StartProcessing> for RawStdinSubscriber {
    type Msg = RawStdinSubscriberMsg;

    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: StartProcessing, _sender: Sender) {
        // let stdin = io::stdin();
        // //        info!("raw publisher {:?}", self.raw_stdout_publisher);
        // for line in stdin.lock().lines() {
        //     match line.unwrap().as_ref() {
        //         "i" => {
        //             self.microgrid.send_msg(MicrogridControlMessage::InitiateIsland.into(), None);
        //             //info!("islanding microgrid")
        //         }
        //         "c" => {
        //             self.microgrid.send_msg(MicrogridControlMessage::InitiateReconnect.into(), None);
        //             //info!("connecting microgrid")
        //         }
        //         foo => info!("unsupported message from text {}", foo)
        //     }
        //     //            self.publisher.send_msg(TextLine { line: line.unwrap() }.into(), None);
        // }
    }
}

impl Receive<ActorRefWrap<PublisherMsg>> for RawStdinSubscriber {
    type Msg = RawStdinSubscriberMsg;

    fn receive(
        &mut self,
        _ctx: &Context<Self::Msg>,
        msg: ActorRefWrap<PublisherMsg>,
        _sender: Sender,
    ) {
        self.publisher = msg.0;
    }
}

impl Receive<RequestActorStats> for RawStdinSubscriber {
    type Msg = RawStdinSubscriberMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, _msg: RequestActorStats, sender: Sender) {
        info!("fuck");
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
