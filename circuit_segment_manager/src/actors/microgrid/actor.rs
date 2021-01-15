use crate::actors::microgrid::{state::SwitchState, MicrogridMsg};
use riker::{
    actor::{Actor, BasicActorRef, Context, Receive, Sender},
    actors::SystemMsg,
};

use microgrid_protobuf::{device_control::DeviceControlMessage, MicrogridControl};

use crate::{
    actors::{
        coordinator::openfmb::openfmb::OpenFMBDevice,
        CoordinatorMsg, DeviceMsg, Microgrid, MicrogridState,
        NetZeroState::{ESSCharge, ESSDischarge},
        PersistorMsg, Publisher, PublisherMsg, ShopLoadReading,
    },
    messages::{
        ActorStats, BreakerStatusExt, OpenFMBCommon, OpenFMBMessage, RequestActorStats,
        SwitchStatusExt,
    },
    traits::{CircuitSegment, CircuitSegmentConnectionState, ConnectableCircuitSegment},
};
use log::{debug, info, warn};
use openfmb_messages::commonmodule::{
    DynamicTestKind, EnsDynamicTestKind, OptionalStateKind,
};
use openfmb_messages::switchmodule::SwitchStatus;
use openfmb_messages::{
    breakermodule::{BreakerDiscreteControlProfile, BreakerStatusProfile, BreakerReadingProfile},
    commonmodule::{DbPosKind, StateKind},
    essmodule::{EssControlProfile, EssReadingProfile, EssStatusProfile},
    generationmodule::{
        GenerationControlProfile, GenerationReadingProfile, GenerationStatusProfile,
    },
    loadmodule::{LoadControlProfile, LoadReadingProfile, LoadStatusProfile},
    metermodule::MeterReadingProfile,
    solarmodule::{SolarControlProfile, SolarReadingProfile, SolarStatusProfile},
    switchmodule::{SwitchDiscreteControlProfile, SwitchReadingProfile, SwitchStatusProfile},
};
use openfmb_messages_ext::{
    ess::{ESSStatusExt, EssStatusExt},
    generation::GenerationStatusExt,
    load::LoadReadingExt,
    meter::reading::MeterReadingExt,
    solar::SolarStatusExt,
    breaker::BreakerReadingExt,
    EssReadingExt, OpenFMBExt, SwitchReadingExt, *,
};
use std::time::SystemTime;

impl Actor for Microgrid {
    type Msg = MicrogridMsg;

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

impl Receive<RequestActorStats> for Microgrid {
    type Msg = MicrogridMsg;
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

impl Receive<OpenFMBMessage> for Microgrid {
    type Msg = MicrogridMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: OpenFMBMessage, _sender: Sender) {
        self.persistor
            .send_msg(msg.clone().into(), Some(ctx.myself.clone().into()));
        use super::OpenFMBMessage::*;
        match msg {
            LoadReading(reading) => {
                ctx.myself.send_msg(reading.as_ref().clone().into(), None);
            }
            LoadStatus(status) => {
                ctx.myself.send_msg(status.as_ref().clone().into(), None);
            }
            MeterReading(reading) => {
                ctx.myself.send_msg(reading.as_ref().clone().into(), None);
            }

            SwitchReading(reading) => {
                ctx.myself.send_msg(reading.as_ref().clone().into(), None);
            }

            SwitchStatus(status) => {
                ctx.myself.send_msg(status.as_ref().clone().into(), None);
            }
            SolarReading(reading) => {
                ctx.myself.send_msg(reading.as_ref().clone().into(), None);
            }

            SolarStatus(status) => {
                ctx.myself.send_msg(status.as_ref().clone().into(), None);
            }
            ESSStatus(status) => {
                ctx.myself.send_msg(status.as_ref().clone().into(), None);
            }
            ESSReading(reading) => {
                ctx.myself.send_msg(reading.as_ref().clone().into(), None);
            }
            GenerationReading(reading) => {
                ctx.myself.send_msg(reading.as_ref().clone().into(), None);
            }
            GenerationStatus(status) => {
                ctx.myself.send_msg(status.as_ref().clone().into(), None);
            }
            BreakerStatus(status) => {
                ctx.myself.send_msg(status.as_ref().clone().into(), None);
            }
            BreakerReading(status) => {
                ctx.myself.send_msg(status.as_ref().clone().into(), None);
            }
            SwitchControl(_status) => {}
            GeneratorControl(_status) => {}
            SolarControl(_status) => {}
            EssControl(_status) => {}
            LoadControl(_status) => {}
            BreakerControl(_status) => {}
            ResourceStatus(_status) => {}
            ResourceControl(_status) => {}
            _ => {
                panic!(
                    "not handling msg {:?} from {:?}: ",
                    msg.message_type(),
                    msg.device_mrid()
                );
            }
        }
    }
}

impl Receive<LoadReadingProfile> for Microgrid {
    type Msg = MicrogridMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: LoadReadingProfile, _sender: Sender) {
        //FIXME why don't we get any of these messages?
        self.microgrid_status.loadbank.load_reading = msg.load_reading();
        // info!(
        //     "load_reading from {:?} {:?}",
        //     openfmb_protobuf_ext::OpenFMBExt::device_name(&msg).unwrap(),
        //     openfmb_protobuf_ext::OpenFMBExt::device_mrid(&msg).unwrap()
        // )
    }
}

impl Receive<DeviceControlMessage> for Microgrid {
    type Msg = MicrogridMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: DeviceControlMessage, _sender: Sender) {
        match msg {
            DeviceControlMessage::EnableSolarInverter => {
                let msg: SolarControlProfile = SolarControlProfile::solar_on_msg(
                    &self
                        .cfg
                        .get_str("circuit_segment_devices.parker-invert.mrid")
                        .unwrap(),
                    80000.0,
                );
                self.publisher.send_msg(msg.into(), None);
            }
            DeviceControlMessage::DisableSolarInverter => {
                let msg: SolarControlProfile = SolarControlProfile::solar_off_msg(
                    &self
                        .cfg
                        .get_str("circuit_segment_devices.parker-invert.mrid")
                        .unwrap(),
                );
                self.publisher.send_msg(msg.into(), None);
            }
            DeviceControlMessage::EnableLoadbank => {
                self.publisher.send_msg(
                    LoadControlProfile::loadbank_on_msg(
                        &self
                            .cfg
                            .get_str("circuit_segment_devices.load-bank.mrid")
                            .unwrap(),
                        150000.0,
                    )
                    .into(),
                    None,
                );
            }
            DeviceControlMessage::DisableLoadbank => {
                let msg: LoadControlProfile = LoadControlProfile::loadbank_off_msg(
                    &self
                        .cfg
                        .get_str("circuit_segment_devices.load-bank.mrid")
                        .unwrap(),
                );
                self.publisher.send_msg(msg.into(), None);
            }
            DeviceControlMessage::EssStart => {
                let msg: EssControlProfile = EssControlProfile::start_now_gridconnected_msg(
                    &self
                        .cfg
                        .get_str("circuit_segment_devices.abb_pcs100.mrid")
                        .unwrap(),
                    self.cfg.get_float("tests.ess_start_charge_rate").unwrap(),
                );
                self.publisher.send_msg(msg.into(), None);
            }
            DeviceControlMessage::EssDischarge => {
                let msg: EssControlProfile = EssControlProfile::discharge_now_msg(
                    &self
                        .cfg
                        .get_str("circuit_segment_devices.abb_pcs100.mrid")
                        .unwrap(),
                    19000.0,
                );
                self.publisher.send_msg(msg.into(), None);
            }
            DeviceControlMessage::EssSocManage => {
                let msg: EssControlProfile = EssControlProfile::soc_manage_msg(
                    &self
                        .cfg
                        .get_str("circuit_segment_devices.abb_pcs100.mrid")
                        .unwrap(),
                );
                self.publisher.send_msg(msg.into(), None);
            }
            DeviceControlMessage::EssSocLimits => {
                let msg: EssControlProfile = EssControlProfile::soc_limits_msg(
                    &self
                        .cfg
                        .get_str("circuit_segment_devices.abb_pcs100.mrid")
                        .unwrap(),
                );
                self.publisher.send_msg(msg.into(), None);
            }
            DeviceControlMessage::EssStop => {
                let msg: EssControlProfile = EssControlProfile::stop_now_msg(
                    &self
                        .cfg
                        .get_str("circuit_segment_devices.abb_pcs100.mrid")
                        .unwrap(),
                );
                self.publisher.send_msg(msg.into(), None);
            }
            DeviceControlMessage::GeneratorOn => match self.microgrid_status.generator.disabled {
                true => {}
                false => {
                    let msg: GenerationControlProfile = GenerationControlProfile::generator_on_msg(
                        &self
                            .cfg
                            .get_str("circuit_segment_devices.turbine-array.mrid")
                            .unwrap(),
                        130000.0,
                    );
                    self.publisher.send_msg(msg.into(), None);
                }
            },
            DeviceControlMessage::GeneratorDisabled => {
                let msg: GenerationControlProfile = GenerationControlProfile::generator_off_msg(
                    &self
                        .cfg
                        .get_str("circuit_segment_devices.turbine-array.mrid")
                        .unwrap(),
                );
                self.publisher.send_msg(msg.into(), None);
                self.microgrid_status.generator.disabled = true
            }
            DeviceControlMessage::GeneratorEnabled => {
                self.microgrid_status.generator.disabled = false
            }
            DeviceControlMessage::GeneratorOff => {
                let msg: GenerationControlProfile = GenerationControlProfile::generator_off_msg(
                    &self
                        .cfg
                        .get_str("circuit_segment_devices.turbine-array.mrid")
                        .unwrap(),
                );
                self.publisher.send_msg(msg.into(), None);
            }
            DeviceControlMessage::SwitchOneOpen => {
                let msg: BreakerDiscreteControlProfile = BreakerDiscreteControlProfile::breaker_open_msg(
                    &self
                        .cfg
                        .get_str("circuit_segment_devices.way1.mrid")
                        .unwrap(),
                );
                self.publisher.send_msg(msg.into(), None);
            }
            DeviceControlMessage::SwitchOneClosed => {
                let msg: BreakerDiscreteControlProfile = BreakerDiscreteControlProfile::breaker_close_msg(
                    &self
                        .cfg
                        .get_str("circuit_segment_devices.way1.mrid")
                        .unwrap(),
                );
                self.publisher.send_msg(msg.into(), None);
            }
            DeviceControlMessage::SwitchTwoOpen => {
                let msg: BreakerDiscreteControlProfile = BreakerDiscreteControlProfile::breaker_open_msg(
                    &self
                        .cfg
                        .get_str("circuit_segment_devices.way2.mrid")
                        .unwrap(),
                );
                self.publisher.send_msg(msg.into(), None);
            }
            DeviceControlMessage::SwitchTwoClosed => {
                let msg: BreakerDiscreteControlProfile = BreakerDiscreteControlProfile::breaker_close_msg(
                    &self
                        .cfg
                        .get_str("circuit_segment_devices.way2.mrid")
                        .unwrap(),
                );
                self.publisher.send_msg(msg.into(), None);
            }

            DeviceControlMessage::BreakerThreeOpen => {
                let msg: BreakerDiscreteControlProfile =
                    BreakerDiscreteControlProfile::breaker_open_msg(
                        &self
                            .cfg
                            .get_str("circuit_segment_devices.breaker3.mrid")
                            .unwrap(),
                    );
                self.publisher.send_msg(msg.into(), None);
            }
            DeviceControlMessage::BreakerThreeClosed => {
                let msg: BreakerDiscreteControlProfile =
                    BreakerDiscreteControlProfile::breaker_close_msg(
                        &self
                            .cfg
                            .get_str("circuit_segment_devices.breaker3.mrid")
                            .unwrap(),
                    );
                self.publisher.send_msg(msg.into(), None);
            }
            DeviceControlMessage::SwitchFourOpen => {
                let msg: BreakerDiscreteControlProfile = BreakerDiscreteControlProfile::breaker_open_msg(
                    &self
                        .cfg
                        .get_str("circuit_segment_devices.way4.mrid")
                        .unwrap(),
                );
                self.publisher.send_msg(msg.into(), None);
            }
            DeviceControlMessage::SwitchFourClosed => {
                let msg: BreakerDiscreteControlProfile = BreakerDiscreteControlProfile::breaker_close_msg(
                    &self
                        .cfg
                        .get_str("circuit_segment_devices.way4.mrid")
                        .unwrap(),
                );
                self.publisher.send_msg(msg.into(), None);
            }
        }
    }
}

impl Receive<MicrogridControl> for Microgrid {
    type Msg = MicrogridMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: MicrogridControl, _sender: Sender) {
        //dbg!(&msg);
        use microgrid_protobuf::microgrid_control::ControlMessage::*;
        match msg.control_message.unwrap() {
            InitiateIsland(_msg) => self.island(ctx),
            ReconnectPretestOne(_msg) => self.reconnect_pretest_one(ctx),
            ReconnectPretestTwo(_msg) => self.reconnect_pretest_two(ctx),
            ReconnectTest(_msg) => self.reconnect(ctx),
            ResetDevices(_msg) => self.reset_microgrid_devices(ctx),
            InitiateGridConnect(_msg) => self.reconnect(ctx),
            EnableNetZero(_msg) => self.enable_netzero(ctx),
            DisableNetZero(_msg) => self.disable_netzero(ctx),
            Shutdown(_msg) => panic!("not supported"),
        }
    }
}

impl Receive<SolarReadingProfile> for Microgrid {
    type Msg = MicrogridMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: SolarReadingProfile, _sender: Sender) {
        self.microgrid_status.solar.last_net = msg.solar_reading() as f32;
    }
}

impl Receive<GenerationReadingProfile> for Microgrid {
    type Msg = MicrogridMsg;
    fn receive(
        &mut self,
        _ctx: &Context<Self::Msg>,
        msg: GenerationReadingProfile,
        _sender: Sender,
    ) {
        if let Ok(device_mrid) = openfmb_messages_ext::OpenFMBExt::device_mrid(&msg) {
            if device_mrid == self.devices.turbine_array {
                self.microgrid_status.generator.last_net = msg.generation_reading() as f32;
            }
        }
    }
}

impl Receive<GenerationStatusProfile> for Microgrid {
    type Msg = MicrogridMsg;
    fn receive(
        &mut self,
        _ctx: &Context<Self::Msg>,
        msg: GenerationStatusProfile,
        _sender: Sender,
    ) {
        if let (Ok(device_mrid), Ok(status)) = (
            openfmb_messages_ext::OpenFMBExt::device_mrid(&msg),
            msg.generation_status(),
        ) {
            if device_mrid == self.devices.turbine_array {
                self.microgrid_status.generator.state = Some(status);
            }
        }
    }
}

impl Receive<EssReadingProfile> for Microgrid {
    type Msg = MicrogridMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: EssReadingProfile, _sender: Sender) {
        //dbg!(&msg);
        self.microgrid_status.battery.last_net = msg.ess_reading().unwrap() as f32;
    }
}

impl Receive<SwitchReadingProfile> for Microgrid {
    type Msg = MicrogridMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: SwitchReadingProfile, _sender: Sender) {
        match openfmb_messages_ext::OpenFMBExt::device_name(&msg.clone())
            .unwrap()
            .as_ref()
        {
            "way1" => {
                self.microgrid_status.way1.last_net = msg.switch_reading() as f32;
                self.net_zero(ctx);
                info!("{}", self.microgrid_status);
            }
            "way2" => {
                self.microgrid_status.way2.last_net = msg.switch_reading() as f32;
            }
            "way3" => {
                self.microgrid_status.way3.last_net = msg.switch_reading() as f32;
            }
            "way4" => {
                self.microgrid_status.way4.last_net = msg.switch_reading() as f32;
            }

            _ => {}
        }
    }
}

impl Receive<LoadStatusProfile> for Microgrid {
    type Msg = MicrogridMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: LoadStatusProfile, _sender: Sender) {
        match msg.device_state().unwrap().as_str() {
            "On" => self.microgrid_status.loadbank.state = Some(StateKind::On),
            "Off" => self.microgrid_status.loadbank.state = Some(StateKind::Off),
            _ => panic!("impossible value"),
        }
    }
}

impl Receive<SolarStatusProfile> for Microgrid {
    type Msg = MicrogridMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: SolarStatusProfile, _sender: Sender) {
        match &msg.solar_status {
            None => {}
            Some(_solar_status) => {
                self.microgrid_status.solar.state = Some(msg.solar_status().unwrap())
            }
        }
    }
}

impl Receive<EssStatusProfile> for Microgrid {
    type Msg = MicrogridMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: EssStatusProfile, _sender: Sender) {
        self.microgrid_status.battery.soc = msg.ess_soc().unwrap() as f32;
        self.microgrid_status.battery.mode = match msg.ess_mode() {
            Ok(msg) => Some(msg),
            Err(_) => None,
        };
        let battery_state = msg
            .ess_status
            .clone()
            .and_then(|ess_status| ess_status.ess_status_zgen)
            .and_then(|ess_status_zgen| ess_status_zgen.e_ss_event_and_status_zgen)
            .and_then(|e_ss_event_and_status_zgen| e_ss_event_and_status_zgen.point_status)
            .and_then(|point_status| point_status.state)
            .and_then(|state| Some(state.value));
        self.microgrid_status.battery.state = match battery_state {
            Some(0) => Some(StateKind::Undefined),
            Some(1) => Some(StateKind::Off),
            Some(2) => Some(StateKind::On),
            Some(3) => Some(StateKind::Standby),
            _ => None,
        };

        self.microgrid_status.battery.status = msg.ess_status;
    }
}

impl Receive<MeterReadingProfile> for Microgrid {
    type Msg = MicrogridMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: MeterReadingProfile, _sender: Sender) {
        match openfmb_messages_ext::OpenFMBExt::device_name(&msg)
            .unwrap()
            .as_str()
        {
            "ion-shop" | "ion_shop" => {
                self.microgrid_status.shop.ts = SystemTime::now();
                self.microgrid_status.shop.reading = ShopLoadReading::Load(msg.meter_reading() as f32);
            }
            "way1" => {
                todo!();
            }
            "way2" => todo!(),
            "way3" => todo!(),
            "way4" => todo!(),
            "load-bank" => self.microgrid_status.loadbank.load_reading = msg.meter_reading(),
            "load-bank-meter" => self.microgrid_status.loadbank.load_reading = msg.meter_reading(),
            "satec-aux-saft" => {}      //
            "satec-aux-abb" => {}       //
            "satec_aux_saft_dnp3" => {} //
            "ion_lb" | "ion-lb" => {}   //meter for aux power
            "sat_gypsum" | "sat-gypsum" => {}
            "eaton_m410" | "eaton-m410" => {} //meter
            "satec_aux_abb_dnp3" => {}        //meter for aux power

            _ => todo!(
                "{}",
                openfmb_messages_ext::OpenFMBExt::device_name(&msg).unwrap()
            ),
        }
    }
}

impl Receive<BreakerStatusProfile> for Microgrid {
    type Msg = MicrogridMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: BreakerStatusProfile, _sender: Sender) {
        match openfmb_messages_ext::OpenFMBExt::device_name(&msg)
            .unwrap()
            .as_ref()
        {
            "breaker-3" | "breaker3" => {
                self.microgrid_status.brk3.status = Some(msg.breaker_status());
            }
            "way1" => {
                //dbg!(&msg.switch_status);
                self.microgrid_status.way1.status = Some(msg.breaker_status());
                self.microgrid_status.way1.state = msg.breaker_status.clone();
            }
            "way2" => {
                self.microgrid_status.way2.status = Some(msg.breaker_status());
                self.microgrid_status.way2.state = msg.breaker_status.clone();
            }
            "way3" => {
                self.microgrid_status.way3.status = Some(msg.breaker_status());
                self.microgrid_status.way3.state = msg.breaker_status.clone();
            }
            "way4" => {
                self.microgrid_status.way4.status = Some(msg.breaker_status());
                self.microgrid_status.way4.state = msg.breaker_status.clone();
            }
            other => debug!("Ignoring status update for breaker named {}", other), // Ignore other breakers
        }

        //TODO review with tupshin, is this needed here? We have SwitchStatusProfile updates below
        // self.microgrid_status.way1 = SwitchState {
        //     last_net: self.microgrid_status.way1.last_net,
        //     status: Some(DbPosKind::Transient),
        //     state: self.microgrid_status.way1.state.clone(),
        // };
    }
}

impl Receive<BreakerReadingProfile> for Microgrid {
    type Msg = MicrogridMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: BreakerReadingProfile, _sender: Sender) {
        match openfmb_messages_ext::OpenFMBExt::device_name(&msg.clone())
            .unwrap()
            .as_ref()
        {
            "way1" => {
                self.microgrid_status.way1.last_net = msg.breaker_reading() as f32;
                self.net_zero(ctx);
                info!("{}", self.microgrid_status);
            }
            "way2" => {
                self.microgrid_status.way2.last_net = msg.breaker_reading() as f32;
            }
            "way3" => {
                self.microgrid_status.way3.last_net = msg.breaker_reading() as f32;
            }
            "way4" => {
                self.microgrid_status.way4.last_net = msg.breaker_reading() as f32;
            }

            _ => {}
        }
    }
}

impl Receive<SwitchStatusProfile> for Microgrid {
    type Msg = MicrogridMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: SwitchStatusProfile, _sender: Sender) {
        match openfmb_messages_ext::OpenFMBExt::device_name(&msg)
            .unwrap()
            .as_ref()
        {
            "way1" => {
                //dbg!(&msg.switch_status);
                //self.microgrid_status.way1.status = Some(msg.switch_status());
                //self.microgrid_status.way1.state = msg.switch_status.clone();
            }
            "way2" => {
                //self.microgrid_status.way2.status = Some(msg.switch_status());
                //self.microgrid_status.way2.state = msg.switch_status.clone();
            }
            "way3" => {
                //self.microgrid_status.way3.status = Some(msg.switch_status());
                //self.microgrid_status.way3.state = msg.switch_status.clone();
            }
            "way4" => {
                //self.microgrid_status.way4.status = Some(msg.switch_status());
                //self.microgrid_status.way4.state = msg.switch_status.clone();
            }
            other => debug!("Ignoring status update for switch named {}", other),
        }
    }
}
