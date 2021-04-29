// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::messages::StartProcessing;

use super::hmi_subscriber::HmiSubscriberMsg;
use super::hmi_publisher::HmiPublisherMsg;

use riker::actors::*;
use std::fmt::Debug;

#[allow(unused_imports)]
use openfmb_messages::{
    breakermodule::{
        Breaker, BreakerDiscreteControl, BreakerDiscreteControlProfile, BreakerDiscreteControlXcbr,
        BreakerStatusProfile,
    },
    commonmodule::{
        CheckConditions, ConductingEquipment, ControlDpc, ControlFscc, ControlMessageInfo,
        ControlScheduleFsch, ControlTimestamp, DbPosKind, DynamicTestKind, EnergyConsumer,
        EngGridConnectModeKind, EngScheduleParameter, EnsDynamicTestKind, Ess, GridConnectModeKind,
        IdentifiedObject, MessageInfo, NamedObject, OptionalStateKind, ScheduleCsg,
        ScheduleParameterKind, SchedulePoint, StateKind, StatusMessageInfo,
        SwitchCsg, SwitchPoint, Timestamp,
    },
    essmodule::{
        EssControl, EssControlFscc, EssControlProfile, EssControlScheduleFsch, EssPoint,
        EssReading, EssReadingProfile, EssStatus, EssStatusProfile, Esscsg,
    },
    generationmodule::{
        GeneratingUnit, GenerationControl, GenerationControlFscc, GenerationControlProfile,
        GenerationControlScheduleFsch, GenerationCsg, GenerationEventAndStatusZgen,
        GenerationPoint, GenerationReadingProfile, GenerationStatusProfile,
    },
    loadmodule::{
        LoadControl, LoadControlFscc, LoadControlProfile, LoadControlScheduleFsch, LoadCsg,
        LoadPoint, LoadReadingProfile, LoadStatusProfile,
    },
    metermodule::MeterReadingProfile,
    solarmodule::{
        SolarControl, SolarControlFscc, SolarControlProfile, SolarControlScheduleFsch, SolarCsg,
        SolarInverter, SolarPoint, SolarReadingProfile, SolarStatusProfile,
    },
    switchmodule::{
        ProtectedSwitch, SwitchDiscreteControl, SwitchDiscreteControlProfile,
        SwitchReadingProfile, SwitchStatusProfile, SwitchStatusXswi,
    },
};

#[actor(StartProcessing)]
#[derive(Clone, Debug)]
pub struct Hmi {
    pub message_count: u32,            
    pub publisher: ActorRef<HmiPublisherMsg>,    
    pub subscriber: ActorRef<HmiSubscriberMsg>,    
}
impl
ActorFactoryArgs<(
    ActorRef<HmiPublisherMsg>,
    ActorRef<HmiSubscriberMsg>
)> for Hmi
{
    fn create_args(
        args: (
            ActorRef<HmiPublisherMsg>,
            ActorRef<HmiSubscriberMsg>                      
        ),
    ) -> Self {
        Hmi {
            message_count: 0,            
            publisher: args.0,
            subscriber: args.1                        
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

impl Receive<StartProcessing> for Hmi {
    type Msg = HmiMsg;

    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: StartProcessing, _sender: Sender) {
        self.subscriber.tell(StartProcessing, None);
    }
}
