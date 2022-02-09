// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::coordinator::StartProcessingMessages;

use super::coordinator::*;
use super::processor::ProcessorMsg;

use riker::actors::*;
use timer::Timer;

#[actor(StartProcessingMessages)]
pub struct Monitor {
    processor: ActorRef<ProcessorMsg>,
    timer: Timer,
}

impl ActorFactoryArgs<ActorRef<ProcessorMsg>> for Monitor {
    fn create_args(args: ActorRef<ProcessorMsg>) -> Self {
        Monitor {
            processor: args,
            timer: Timer::new(),
        }
    }
}

impl Actor for Monitor {
    type Msg = MonitorMsg;

    fn post_start(&mut self, _ctx: &Context<Self::Msg>) {
        let processor = self.processor.clone();
        let guard = {
            self.timer
                .schedule_repeating(chrono::Duration::milliseconds(1000), move || {
                    let status = CoordinatorOptions::current_status();
                    processor.tell(status, None);
                })
        };
        guard.ignore();
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
