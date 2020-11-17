use crate::{
    actors::{CoordinatorMsg, DeviceMsg, PersistorMsg, Publisher, PublisherMsg},
    messages::{ActorStats, OpenFMBMessage, RequestActorStats},
};

use openfmb_ops_protobuf::openfmb::{
    commonmodule::{
        CheckConditions, ConductingEquipment, ControlDpc, ControlFscc, ControlMessageInfo,
        ControlScheduleFsch, ControlTimestamp, DynamicTestKind, EnergyConsumer,
        EngGridConnectModeKind, EngScheduleParameter, EnsDynamicTestKind, GridConnectModeKind,
        IdentifiedObject, MessageInfo, NamedObject, OptionalStateKind, ScheduleCsg,
        ScheduleParameterKind, SchedulePoint, StateKind, StatusMessageInfo,
        SwitchControlScheduleFsch, SwitchCsg, SwitchPoint, Timestamp,
    },
    essmodule::{
        EssControl, EssControlFscc, EssControlProfile, EssControlScheduleFsch, EssPoint, Esscsg,
    },
    loadmodule::{
        LoadControl, LoadControlFscc, LoadControlProfile, LoadReadingProfile, LoadStatusProfile,
    },
    switchmodule::{
        SwitchControl, SwitchControlFscc, SwitchControlProfile, SwitchStatus, SwitchStatusProfile,
        SwitchStatusXswi,
    },
};
use riker::actors::*;

use std::str::FromStr;

use std::time::SystemTime;

use config::Config;
use uuid::Uuid;

#[actor(
    OpenFMBMessage,
    RequestActorStats,
    LoadReadingProfile,
    LoadStatusProfile
)]
#[derive(Debug)]
pub struct GenericOpenFMBDevice {
    pub message_count: u32,
    pub mrid: Option<String>,
    pub publisher: ActorRef<PublisherMsg>,
    pub persistor: ActorRef<PersistorMsg>,
    pub last_state: Option<(Islanded, SystemTime)>,
    cfg: Config,
}

impl ActorFactoryArgs<(ActorRef<PublisherMsg>, ActorRef<PersistorMsg>)> for GenericOpenFMBDevice {
    fn create_args(args: (ActorRef<PublisherMsg>, ActorRef<PersistorMsg>)) -> Self {
        GenericOpenFMBDevice {
            message_count: 0,
            mrid: None,
            publisher: args.0,
            persistor: args.1,
            last_state: None,
            cfg: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Islanded {
    True,
    False,
    Islanding,
    Reconnecting,
}

pub trait OpenFMBDevice {}

impl Actor for GenericOpenFMBDevice {
    type Msg = GenericOpenFMBDeviceMsg;

    fn pre_start(&mut self, _ctx: &Context<Self::Msg>) {
        //        self.device.send_msg)
    }

    //    fn post_start(&mut self, ctx: &Context<Self::Msg>) { unimplemented!() }
    //
    //    fn post_stop(&mut self) { unimplemented!() }
    //
    //    fn supervisor_strategy(&self) -> Strategy { unimplemented!() }

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

impl Receive<RequestActorStats> for GenericOpenFMBDevice {
    type Msg = GenericOpenFMBDeviceMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, _msg: RequestActorStats, sender: Sender) {
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

impl Receive<OpenFMBMessage> for GenericOpenFMBDevice {
    type Msg = GenericOpenFMBDeviceMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: OpenFMBMessage, _sender: Sender) {
        self.persistor
            .send_msg(msg.clone().into(), Some(ctx.myself.clone().into()));
        use OpenFMBMessage::*;
        match msg {
            LoadReading(reading) => {
                ctx.myself.send_msg(reading.as_ref().clone().into(), None);
            }
            LoadStatus(status) => {
                ctx.myself.send_msg(status.as_ref().clone().into(), None);
            }
            SwitchControl(_status) => {
                //ctx.myself.send_msg(status.as_ref().clone().into(), None);
            }
            _ => {
                //    warn!("not handling msg {:?} from {:?}: ", msg.message_type(), msg.device_mrid())
            }
        }
    }
}

impl Receive<LoadReadingProfile> for GenericOpenFMBDevice {
    type Msg = GenericOpenFMBDeviceMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: LoadReadingProfile, _sender: Sender) {
        // info!("{:?}", msg.load_reading.unwrap());
    }
}

impl Receive<LoadStatusProfile> for GenericOpenFMBDevice {
    type Msg = GenericOpenFMBDeviceMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: LoadStatusProfile, _sender: Sender) {
        // if self.last_state == None {self.last_state = Some(
        //     (self.is_islanded(msg.clone()), SystemTime::now())
        // )}
        // &self.toggle_islanded_interval(ctx, msg, Duration::from_secs(15));
        //
    }
}

impl Receive<SwitchStatusProfile> for GenericOpenFMBDevice {
    type Msg = GenericOpenFMBDeviceMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: SwitchStatusProfile, _sender: Sender) {
        if self.last_state == None {
            self.last_state = Some((self.is_islanded(msg.clone()), SystemTime::now()))
        }
        //&self.toggle_islanded_interval(ctx, msg, Duration::from_secs(15));
    }
}

pub enum SwitchState {
    Open,
    Closed,
}

pub enum ESSState {
    On,
    Off,
}

impl GenericOpenFMBDevice {
    fn is_islanded(&self, _msg: SwitchStatusProfile) -> Islanded {
        //way 1 open and way3 closed == islanded
        // way 3 open and way 1 closed = connected
        //verify state of battery inverter

        // look at change voltage and frequency dv and df
        // if not in range

        todo!()
    }
}
