// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use riker::actors::*;
use crate::messages::OpenFMBMessage;
use super::processor::ProcessorMsg;

#[actor(OpenFMBMessage)]
#[derive(Debug, Clone)]
pub struct ProfileSubscriber {
    pub message_count: u32,
    pub processor: ActorRef<ProcessorMsg>,
}

impl ActorFactoryArgs<ActorRef<ProcessorMsg>> for ProfileSubscriber {
    fn create_args(args: ActorRef<ProcessorMsg>) -> Self {
        ProfileSubscriber {
            message_count: 0,            
            processor: args,
        }
    }
}

impl ProfileSubscriber {
}

impl Actor for ProfileSubscriber {
    type Msg = ProfileSubscriberMsg;

    fn pre_start(&mut self, _ctx: &Context<Self::Msg>) {}

    fn post_start(&mut self, _ctx: &Context<Self::Msg>) {}

    fn post_stop(&mut self) {}

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) {
        self.message_count += 1;
        self.receive(ctx, msg, sender);
    }
}

impl ProfileSubscriber {}

impl Receive<OpenFMBMessage> for ProfileSubscriber {
    type Msg = ProfileSubscriberMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: OpenFMBMessage, _sender: Sender) {        
        self.processor.send_msg(msg.clone().into(), ctx.myself.clone());
    }
}
