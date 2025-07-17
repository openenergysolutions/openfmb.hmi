// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use super::hmi_publisher::HmiPublisherMsg;
use super::hmi_subscriber::HmiSubscriberMsg;

use riker::actors::*;
use std::fmt::Debug;

pub mod coordinator;
pub mod export;
pub mod hmi_publisher;
pub mod hmi_subscriber;
pub mod monitor;
pub mod processor;
pub mod profile_subscriber;
pub mod utils;

pub use coordinator::*;
pub use export::*;
pub use utils::*;

#[actor(StartProcessingMessages)]
#[derive(Clone, Debug)]
pub struct Hmi {
    pub message_count: u32,
    pub publisher: ActorRef<HmiPublisherMsg>,
    pub subscriber: ActorRef<HmiSubscriberMsg>,
}
impl ActorFactoryArgs<(ActorRef<HmiPublisherMsg>, ActorRef<HmiSubscriberMsg>)> for Hmi {
    fn create_args(args: (ActorRef<HmiPublisherMsg>, ActorRef<HmiSubscriberMsg>)) -> Self {
        Hmi {
            message_count: 0,
            publisher: args.0,
            subscriber: args.1,
        }
    }
}

impl Actor for Hmi {
    type Msg = HmiMsg;

    fn pre_start(&mut self, _ctx: &Context<Self::Msg>) {}

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

impl Receive<StartProcessingMessages> for Hmi {
    type Msg = HmiMsg;

    fn receive(
        &mut self,
        _ctx: &Context<Self::Msg>,
        msg: StartProcessingMessages,
        _sender: Sender,
    ) {
        log::debug!("Received start processing message: {:?}", msg);
        self.subscriber.tell(msg.clone(), None);
        self.publisher.tell(msg.clone(), None);
    }
}
