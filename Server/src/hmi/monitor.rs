// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::messages::StartProcessingMessages;

use super::processor::ProcessorMsg;
use super::pubsub::*;

use riker::actors::*;
use std::{thread, time};

#[actor(StartProcessingMessages)]
#[derive(Clone, Debug)]
pub struct Monitor {
    processor: ActorRef<ProcessorMsg>,
}

impl Monitor {
    fn start(&self) {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let processor = self.processor.clone();
        let _background = rt.spawn(async move {
            let interval = time::Duration::from_millis(1000);
            loop {
                thread::sleep(interval);
                let status = PubSubOptions::current_status();
                processor.tell(status, None);
            }
        });
    }
}

impl ActorFactoryArgs<ActorRef<ProcessorMsg>> for Monitor {
    fn create_args(args: ActorRef<ProcessorMsg>) -> Self {
        Monitor { processor: args }
    }
}

impl Actor for Monitor {
    type Msg = MonitorMsg;

    fn pre_start(&mut self, _ctx: &Context<Self::Msg>) {
        self.start();
    }

    fn recv(&mut self, _ctx: &Context<Self::Msg>, _msg: Self::Msg, _sender: Option<BasicActorRef>) {
    }
}

impl Receive<StartProcessingMessages> for Monitor {
    type Msg = MonitorMsg;

    fn receive(
        &mut self,
        _ctx: &Context<Self::Msg>,
        _msg: StartProcessingMessages,
        _sender: Sender,
    ) {
        // do nothing
    }
}
