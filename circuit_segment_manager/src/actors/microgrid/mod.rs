extern crate dimensioned as dim;
use config::Config;
use core::fmt::Display;
use std::{
    collections::HashMap,
    fmt::Debug,
    thread::sleep,
    time::{Duration, SystemTime},
};
use uuid::Uuid;

use log::{debug, info, warn};
use microgrid_protobuf::{CoordinationStatus, DeviceStatus};

mod actor;
mod state;
pub use actor::*;
pub use state::*;

/// Local macro to help debug the state machine by dumping what messages are being published from where
macro_rules! publish {
    ( $publisher:expr, $msg:expr, $from:expr ) => {{
        info!("microgrid controller publishing {:?}", $msg);
        $publisher.send_msg($msg.into(), $from);
    }};
}

use openfmb_ops_protobuf::openfmb::{
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
        SwitchControlScheduleFsch, SwitchCsg, SwitchPoint, Timestamp,
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
        ProtectedSwitch, SwitchControl, SwitchControlFscc, SwitchControlProfile,
        SwitchReadingProfile, SwitchStatusProfile, SwitchStatusXswi,
    },
};
use openfmb_protobuf_ext::{
    BreakerControlExt, EssControlExt, GenerationControlExt, GenerationReadingExt, LoadControlExt,
    OpenFMBExt, SolarControlExt, SolarReadingExt, SwitchControlExt, SwitchReadingExt,
};

use riker::actors::*;

use crate::{
    actors::{
        coordinator::openfmb::openfmb::OpenFMBDevice,
        CoordinatorMsg, DeviceMsg,
        NetZeroState::{ESSCharge, ESSDischarge},
        PersistorMsg, Publisher, PublisherMsg,
    },
    messages::{
        ActorStats, BreakerStatusExt, OpenFMBCommon, OpenFMBMessage, RequestActorStats,
        SwitchStatusExt,
    },
    traits::{CircuitSegment, CircuitSegmentConnectionState, ConnectableCircuitSegment},
};

use microgrid_protobuf::{device_control::DeviceControlMessage, MicrogridControl};

use openfmb_ops_protobuf::openfmb::{
    commonmodule::UnitMultiplierKind::Micro, switchmodule::SwitchStatus,
};
use std::fmt::Formatter;

pub enum CircuitSegmentError {}

#[derive(Debug)]
struct MicrogridDevices {
    turbine_array: Uuid,
}

#[actor(
    OpenFMBMessage,
    RequestActorStats,
    LoadReadingProfile,
    LoadStatusProfile,
    MeterReadingProfile,
    SwitchStatusProfile,
    SwitchReadingProfile,
    SolarReadingProfile,
    SolarStatusProfile,
    EssStatusProfile,
    MicrogridControl,
    DeviceControlMessage,
    EssReadingProfile,
    GenerationReadingProfile,
    GenerationStatusProfile,
    BreakerStatusProfile
)]
#[derive(Debug)]
pub struct Microgrid {
    pub message_count: u32,
    pub mrid: Option<String>,
    circuit_connections: CircuitConnections,
    pub publisher: ActorRef<PublisherMsg>,
    pub persistor: ActorRef<PersistorMsg>,
    pub cfg: Config,
    devices: MicrogridDevices,
    pub microgrid_status: CircuitSegmentStatus,
}

impl ActorFactoryArgs<(Config, ActorRef<PublisherMsg>, ActorRef<PersistorMsg>)> for Microgrid {
    fn create_args(
        (cfg, publisher, persistor): (Config, ActorRef<PublisherMsg>, ActorRef<PersistorMsg>),
    ) -> Self {
        let devices = MicrogridDevices {
            turbine_array: Uuid::parse_str(
                &cfg.get_str("circuit_segment_devices.turbine-array.mrid")
                    .unwrap(),
            )
            .unwrap(),
        };

        Microgrid {
            message_count: 0,
            mrid: None,
            circuit_connections: CircuitConnections {},
            publisher: publisher,
            persistor: persistor,
            cfg: cfg,
            devices: devices,
            microgrid_status: CircuitSegmentStatus {
                ctl_state: Default::default(),
                way1: Default::default(),
                way2: Default::default(),
                brk3: Default::default(),
                way3: Default::default(),
                way4: Default::default(),
                solar: Default::default(),
                generator: Default::default(),
                shop: Default::default(),
                battery: BatteryState {
                    status: None,
                    state: None,
                    mode: None,
                    soc: 0.0,
                    charge_rate: 0.0,
                    last_net: 0.0,
                    min_soc: 0.0,
                    max_soc: 0.0,
                },
                loadbank: Default::default(),
                algorithm: Default::default(),
            },
        }
    }
}

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub enum Islanded {
//     Islanded,
//     Connected,
//     Islanding,
//     Reconnecting,
// }

impl CircuitSegment for Microgrid {
    fn devices(&self) -> Vec<Box<OpenFMBDevice>> {
        todo!("pull this from config")
    }

    fn start_monitoring(&mut self) {
        unimplemented!()
    }

    fn stop_monitoring(&mut self) {
        unimplemented!()
    }

    fn start_coordinating(&mut self) {
        unimplemented!()
    }

    fn stop_coordinating(&mut self) {
        unimplemented!()
    }
}

//todo reimplement islanding and reconnecting in terms of this api
impl ConnectableCircuitSegment for Microgrid {
    fn connection_state(&self) -> CircuitSegmentConnectionState {
        unimplemented!()
    }

    fn update_connection_state(&mut self) -> CircuitSegmentConnectionState {
        unimplemented!()
    }

    fn update_disconnection_state(&mut self) -> CircuitSegmentConnectionState {
        unimplemented!()
    }

    fn update_connected_state(&mut self) -> CircuitSegmentConnectionState {
        unimplemented!()
    }

    fn connect(&mut self) -> Result<CircuitSegmentConnectionState, CircuitSegmentError> {
        unimplemented!()
    }

    fn disconnect(&mut self) -> Result<CircuitSegmentConnectionState, CircuitSegmentError> {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct CircuitConnections {}

impl Microgrid {
    pub fn comms_check(&mut self, ctx: &Context<MicrogridMsg>) {
        let timeout = Duration::from_secs(15);
        if SystemTime::now()
            .duration_since(self.microgrid_status.shop.ts)
            .unwrap()
            > timeout
        {
            self.net_zero_shutdown(ctx)
        }
    }

    // publish status for GUI
    fn send_status(&self) {
        let mut status_msg = CoordinationStatus::default();
        status_msg.segment_state = format!("{}", self.microgrid_status.state());
        status_msg.controller_state = format!("{}", self.microgrid_status.ctl_state);
        status_msg.algorithm_state = format!("{}", self.microgrid_status.algorithm);
        let device_states = vec![
            DeviceStatus {
                name: "way1".into(),
                state: format!("{}", self.microgrid_status.way1),
            },
            DeviceStatus {
                name: "way2".into(),
                state: format!("{}", self.microgrid_status.way2),
            },
            DeviceStatus {
                name: "way3".into(),
                state: format!("{}", self.microgrid_status.way3),
            },
            DeviceStatus {
                name: "brk3".into(),
                state: format!("{}", self.microgrid_status.brk3),
            },
            DeviceStatus {
                name: "way4".into(),
                state: format!("{}", self.microgrid_status.way4),
            },
            DeviceStatus {
                name: "solar".into(),
                state: format!("{}", self.microgrid_status.solar),
            },
            DeviceStatus {
                name: "generator".into(),
                state: format!("{}", self.microgrid_status.generator),
            },
            DeviceStatus {
                name: "battery".into(),
                state: format!("{}", self.microgrid_status.battery),
            },
            DeviceStatus {
                name: "loadbank".into(),
                state: format!("{}", self.microgrid_status.loadbank),
            },
        ];
        status_msg.device_states = device_states;
        let mut config_msg = microgrid_protobuf::Config::default();
        let mut coord_cfg_msg = microgrid_protobuf::CoordinatorConfig::default();
        coord_cfg_msg.environment = self
            .cfg
            .get_str("coordinator.environment")
            .unwrap_or("".into());
        let mut battery_cfg_msg = microgrid_protobuf::BatteryConfig::default();
        battery_cfg_msg.min_soc_soft =
            self.cfg.get_float("battery.min_soc_soft").unwrap_or(-1.0) as f32;
        battery_cfg_msg.min_soc_hard =
            self.cfg.get_float("battery.min_soc_hard").unwrap_or(-1.0) as f32;
        battery_cfg_msg.max_soc_hard =
            self.cfg.get_float("battery.max_soc_hard").unwrap_or(-1.0) as f32;
        battery_cfg_msg.max_soc_soft =
            self.cfg.get_float("battery.max_soc_soft").unwrap_or(-1.0) as f32;
        config_msg.coordinator = Some(coord_cfg_msg);
        config_msg.battery = Some(battery_cfg_msg);
        status_msg.config = Some(config_msg);
        publish!(self.publisher, status_msg, None);
    }

    pub fn net_zero_shutdown(&mut self, _ctx: &Context<MicrogridMsg>) {
        match self.microgrid_status.algorithm {
            NetZeroState::Disabled => return,
            _ => {
                publish!(
                    self.publisher,
                    GenerationControlProfile::generator_off_msg(
                        &self
                            .cfg
                            .get_str("circuit_segment_devices.turbine-array.mrid")
                            .unwrap(),
                    ),
                    None
                );
                publish!(
                    self.publisher,
                    SwitchControlProfile::switch_synchro_msg(
                        &self
                            .cfg
                            .get_str("circuit_segment_devices.way1.mrid")
                            .unwrap(),
                        true,
                    ),
                    None
                );
                // sleep(Duration::from_secs(10));
                self.microgrid_status.algorithm = NetZeroState::ShuttingDown
            }
        }
    }

    pub fn net_zero(&mut self, ctx: &Context<MicrogridMsg>) {
        use NetZeroState::*;
        let _delay = SystemTime::now()
            .duration_since(self.microgrid_status.shop.ts)
            .unwrap();
        self.send_status();
        match &self.microgrid_status.ctl_state {
            MicrogridControlState::Reconnecting(ReconnectingState::VerifyEssSynchro) => {
                let ess_state: bool = match &self.microgrid_status.battery.status.clone() {
                    Some(status) => {
                        status
                            .ess_status_zgen
                            .clone()
                            .unwrap()
                            .e_ss_event_and_status_zgen
                            .unwrap()
                            .gn_syn_st
                            .unwrap()
                            .st_val
                    }
                    None => {
                        warn!("no way1 status: {:?}", self.microgrid_status.way1);
                        false
                        // return
                    }
                };

                if ess_state {
                    self.microgrid_status
                        .update_ctl_state(MicrogridControlState::Reconnecting(
                            ReconnectingState::CleanupReconnect,
                        ));
                } else {
                    warn!("ess synchro check not yet on");
                }
            }
            MicrogridControlState::Reconnecting(ReconnectingState::VerifySwitchSynchro) => {
                warn!("verifying switch synchro");
                let switch_state = match &self.microgrid_status.way1.state.clone() {
                    Some(state) => {
                        state
                            .switch_status_xswi
                            .clone()
                            .unwrap()
                            .dynamic_test
                            .unwrap()
                            .st_val
                    }
                    None => {
                        warn!("no way1 state: {:?}", self.microgrid_status.way1);
                        return;
                    }
                };

                match switch_state {
                    1 => {
                        warn!("disabling ess synchro");
                        //switch synchron on, battery synchro off
                        publish!(
                            self.publisher,
                            EssControlProfile::build_synchro_profile(
                                &self
                                    .cfg
                                    .get_str("circuit_segment_devices.abb_pcs100.mrid")
                                    .unwrap(),
                                SystemTime::now(),
                                true,
                            ),
                            None
                        );
                        warn!("sending ess synchro ");
                        sleep(Duration::from_millis(50));
                        publish!(
                            self.publisher,
                            EssControlProfile::build_vsi_pq_control_profile(
                                &self
                                    .cfg
                                    .get_str("circuit_segment_devices.abb_pcs100.mrid")
                                    .unwrap(),
                                SystemTime::now(),
                            ),
                            None
                        );
                        self.microgrid_status.update_ctl_state(
                            MicrogridControlState::Reconnecting(
                                ReconnectingState::VerifyEssSynchro,
                            ),
                        );
                        warn!("switching to verify ess synchro mode ");
                    }
                    0 => {
                        //Switch not yet in synchro check state
                    }
                    _ => unreachable!(),
                }
            }
            MicrogridControlState::Reconnecting(ReconnectingState::VerifyReconnect) => {
                self.microgrid_status
                    .update_ctl_state(MicrogridControlState::Idle);
            }
            MicrogridControlState::Reconnecting(ReconnectingState::CleanupReconnect) => {
                match &self.microgrid_status.way1.status.unwrap() {
                    DbPosKind::Transient => warn!("way1 still in transient mode"),
                    DbPosKind::Invalid => warn!("way1 in invalid state"),
                    DbPosKind::Closed => {
                        publish!(
                            self.publisher,
                            EssControlProfile::build_synchro_profile(
                                &self
                                    .cfg
                                    .get_str("circuit_segment_devices.abb_pcs100.mrid")
                                    .unwrap(),
                                SystemTime::now(),
                                false,
                            ),
                            None
                        );
                        warn!("sending ess synchro OFF");
                        self.microgrid_status.update_ctl_state(
                            MicrogridControlState::Reconnecting(ReconnectingState::VerifyReconnect),
                        );
                    }
                    DbPosKind::Open => warn!("way1 still open"),
                };
                // let msg = SwitchControlProfile::switch_synchro_msg(
                //     &self.cfg.get_str("circuit_segment_devices.way1.mrid").unwrap(),
                //     false,
                // );
                // //dbg!(&msg);
                // publish!(self.publisher, msg, None);
                // sleep(Duration::from_secs(2));

                match self
                    .cfg
                    .get_str("coordinator.environment")
                    .unwrap()
                    .as_str()
                {
                    "dev" => {
                        // //way 1 closed
                        publish!(
                            self.publisher,
                            SwitchControlProfile::switch_close_msg(
                                &self
                                    .cfg
                                    .get_str("circuit_segment_devices.way1.mrid")
                                    .unwrap(),
                            ),
                            None
                        );
                        publish!(
                            self.publisher,
                            BreakerDiscreteControlProfile::breaker_open_msg(
                                &self
                                    .cfg
                                    .get_str("circuit_segment_devices.breaker3.mrid")
                                    .unwrap(),
                            ),
                            None
                        );
                    }
                    _ => {}
                }

                //                self.microgrid_status.state = MicroGridState::VerifyReconnect;

                // sleep(Duration::from_secs(1));
                //ess cleanup

                //breaker 3 open
            }
            MicrogridControlState::Islanding => {
                warn!("matching islanding state");
                match &self.microgrid_status.way1.status.unwrap() {
                    DbPosKind::Transient => {}
                    DbPosKind::Invalid => {}
                    DbPosKind::Closed => {}
                    DbPosKind::Open => {
                        //put battery into vsiiso mode
                        publish!(
                            self.publisher,
                            EssControlProfile::vsi_iso_msg(
                                &self
                                    .cfg
                                    .get_str("circuit_segment_devices.abb_pcs100.mrid")
                                    .unwrap(),
                            ),
                            None
                        );
                        self.microgrid_status
                            .update_ctl_state(MicrogridControlState::Idle);
                        // close breaker3 but only in dev
                        match self
                            .cfg
                            .get_str("coordinator.environment")
                            .unwrap()
                            .as_str()
                        {
                            "dev" => publish!(
                                self.publisher,
                                BreakerDiscreteControlProfile::breaker_close_msg(
                                    &self
                                        .cfg
                                        .get_str("circuit_segment_devices.breaker3.mrid")
                                        .unwrap(),
                                ),
                                None
                            ),
                            _ => {}
                        }
                    }
                }
                // //put battery into vsiiso mode
                // publish!(self.publisher,
                //     EssControlProfile::vsi_iso_msg(
                //         &self.cfg.get_str("circuit_segment_devices.abb_pcs100.mrid").unwrap(),
                //     )
                //     .into(),
                //     None,
                // );
                // close breaker3 but only in dev
                match self
                    .cfg
                    .get_str("coordinator.environment")
                    .unwrap()
                    .as_str()
                {
                    "dev" => publish!(
                        self.publisher,
                        BreakerDiscreteControlProfile::breaker_close_msg(
                            &self
                                .cfg
                                .get_str("circuit_segment_devices.breaker3.mrid")
                                .unwrap(),
                        ),
                        None
                    ),
                    _ => {}
                }
            }
            MicrogridControlState::Idle => {
                if self.microgrid_status.algorithm != NetZeroState::Disabled {
                    match self.microgrid_status.state() {
                        MicrogridState::Islanded => self.islanded_netzero(ctx),
                        MicrogridState::Connected => self.connected_netzero(ctx),
                        MicrogridState::Unknown => {
                            warn!("Ignoring netzero attempt while microgrid is in unknown state")
                        }
                    }
                }
            }
        }
        self.comms_check(ctx);
    }

    fn connected_netzero(&mut self, ctx: &Context<MicrogridMsg>) {
        let max_soc_hard = ctx
            .system
            .config()
            .get_float("battery.max_soc_hard")
            .unwrap() as f32;
        let max_soc_soft = ctx
            .system
            .config()
            .get_float("battery.max_soc_soft")
            .unwrap() as f32;
        let min_soc_hard = ctx
            .system
            .config()
            .get_float("battery.min_soc_hard")
            .unwrap() as f32;
        let min_soc_soft = ctx
            .system
            .config()
            .get_float("battery.min_soc_soft")
            .unwrap() as f32;
        match &self.microgrid_status.algorithm {
            NetZeroState::Disabled => {}
            NetZeroState::ShuttingDown => {
                //succesfully grid connected. now we get to shut the rest down
                publish!(
                    self.publisher,
                    GenerationControlProfile::generator_off_msg(
                        &self
                            .cfg
                            .get_str("circuit_segment_devices.turbine-array.mrid")
                            .unwrap(),
                    ),
                    None
                );

                let msg: SolarControlProfile = SolarControlProfile::solar_off_msg(
                    &self
                        .cfg
                        .get_str("circuit_segment_devices.parker-invert.mrid")
                        .unwrap(),
                );
                publish!(self.publisher, msg, None);

                publish!(
                    self.publisher,
                    LoadControlProfile::loadbank_off_msg(
                        &self
                            .cfg
                            .get_str("circuit_segment_devices.load-bank.mrid")
                            .unwrap(),
                    ),
                    None
                );
                let msg: EssControlProfile = EssControlProfile::soc_manage_msg(
                    &self
                        .cfg
                        .get_str("circuit_segment_devices.abb_pcs100.mrid")
                        .unwrap(),
                );
                publish!(self.publisher, msg, None);
            }
            NetZeroState::ESSDischarge => {}
            NetZeroState::ESSCharge => {
                // match self.microgrid_status.solar.to_float() {
                //     sol if sol > 50.0 => self.microgrid_status.algorithm = NetZeroState::Normal,
                //    sol if sol <= 50.0 =>
                match self.microgrid_status.battery.soc {
                    // If we reach >= 90% while grid connected
                    // we immediately turn off the turbines, turn on the load bank
                    // and island. We should island before this happens though.
                    soc if soc >= max_soc_hard => {
                        publish!(
                            self.publisher,
                            GenerationControlProfile::generator_off_msg(
                                &self
                                    .cfg
                                    .get_str("circuit_segment_devices.turbine-array.mrid")
                                    .unwrap(),
                            ),
                            None
                        );
                        publish!(
                            self.publisher,
                            LoadControlProfile::loadbank_on_msg(
                                &self
                                    .cfg
                                    .get_str("circuit_segment_devices.load-bank.mrid",)
                                    .unwrap(),
                                150000.0,
                            ),
                            None
                        );
                        self.microgrid_status.algorithm = NetZeroState::LoadDischarge;
                        self.island(ctx);
                    }
                    soc if soc >= max_soc_soft => {
                        publish!(
                            self.publisher,
                            GenerationControlProfile::generator_off_msg(
                                &self
                                    .cfg
                                    .get_str("circuit_segment_devices.turbine-array.mrid")
                                    .unwrap(),
                            ),
                            None
                        );
                        self.island(ctx);
                        self.microgrid_status.algorithm = NetZeroState::Normal;
                    }
                    // While under 80% and grid connected, tell the battery to charge
                    soc if soc < max_soc_soft => {
                        self.charge_battery(125_000.0, ctx);
                        self.microgrid_status.algorithm = ESSCharge;
                    }
                    _ => unreachable!(),
                }
            }
            NetZeroState::Normal => {
                match self.microgrid_status.loadbank.state {
                    Some(StateKind::On) => {
                        self.microgrid_status.algorithm = NetZeroState::LoadDischarge
                    }
                    Some(StateKind::Off) => {
                        self.microgrid_status.algorithm = NetZeroState::Normal;
                    }
                    Some(StateKind::Standby) => {
                        //FIXME is this correct?
                        self.microgrid_status.algorithm = NetZeroState::Normal;
                    }
                    None => {}
                }

                //if we ever get to min_soc_hard

                match self.microgrid_status.solar.to_float() {
                    sol if sol > 0.01 => {
                        match &self.microgrid_status.shop.reading.to_float()
                            > &self.microgrid_status.solar.to_float()
                        {
                            //Load is greater than solar
                            true => {
                                match self.microgrid_status.battery.soc {
                                    soc if soc >= max_soc_hard => {
                                        publish!(
                                            self.publisher,
                                            LoadControlProfile::loadbank_on_msg(
                                                &self
                                                    .cfg
                                                    .get_str(
                                                        "circuit_segment_devices.load-bank.mrid",
                                                    )
                                                    .unwrap(),
                                                150000.0,
                                            ),
                                            None
                                        );
                                        self.microgrid_status.algorithm =
                                            NetZeroState::LoadDischarge;
                                        self.island(ctx);
                                    }
                                    soc if soc <= min_soc_soft => {
                                        if !self.microgrid_status.generator.disabled {
                                            publish!(self.publisher,
                                                    GenerationControlProfile::generator_on_msg(
                                                        &self
                                                        .cfg
                                                        .get_str("circuit_segment_devices.turbine-array.mrid")
                                                        .unwrap(),130000.0,
                                                    ), None
                                                );
                                        }
                                        self.microgrid_status.algorithm = NetZeroState::ESSCharge;
                                    }
                                    soc if soc <= max_soc_hard => {
                                        self.microgrid_status.algorithm = NetZeroState::Normal;
                                        publish!(
                                            self.publisher,
                                            LoadControlProfile::loadbank_off_msg(
                                                &self
                                                    .cfg
                                                    .get_str(
                                                        "circuit_segment_devices.load-bank.mrid",
                                                    )
                                                    .unwrap(),
                                            ),
                                            None
                                        );
                                    }
                                    _ => todo!(),
                                }
                                self.microgrid_status.battery.charge_rate =
                                    -self.microgrid_status.way1.last_net; //FIXME
                                                                          //dbg!(
                                                                          //    "charging battery at rate {}",
                                                                          //    self.microgrid_status.battery.charge_rate
                                                                          //);
                                self.charge_battery(self.microgrid_status.battery.charge_rate, ctx);
                                //self.charge_battery()*/},
                                // info!("load < solar: setting battery charge rate to {}", self.last_battery_charge_rate);
                            }
                            //Load less than solar
                            false => {
                                //dbg!("soc: {}", self.microgrid_status.battery.soc);
                                match self.microgrid_status.battery.soc {
                                    soc if soc >= max_soc_hard => {
                                        publish!(
                                            self.publisher,
                                            LoadControlProfile::loadbank_on_msg(
                                                &self
                                                    .cfg
                                                    .get_str(
                                                        "circuit_segment_devices.load-bank.mrid",
                                                    )
                                                    .unwrap(),
                                                150000.0,
                                            ),
                                            None
                                        );
                                        self.microgrid_status.algorithm =
                                            NetZeroState::LoadDischarge;
                                        self.island(ctx);
                                    }
                                    soc if soc > max_soc_soft => {
                                        // self.island(ctx);
                                    }
                                    soc if soc > min_soc_hard => {
                                        publish!(self.publisher,
                                        GenerationControlProfile::generator_off_msg(
                                            &self
                                            .cfg
                                            .get_str("circuit_segment_devices.turbine-array.mrid")
                                            .unwrap(),
                                        ), None
                                    );
                                    }
                                    soc if soc < max_soc_hard => {
                                        self.microgrid_status.algorithm = NetZeroState::Normal;
                                        // dbg!("DISABLING LOADBANK");
                                        publish!(
                                            self.publisher,
                                            LoadControlProfile::loadbank_off_msg(
                                                &self
                                                    .cfg
                                                    .get_str(
                                                        "circuit_segment_devices.load-bank.mrid",
                                                    )
                                                    .unwrap(),
                                            ),
                                            None
                                        );
                                    }
                                    _ => todo!(),
                                }
                                //dbg!(
                                //    "charging battery at rate {} + {} = {}",
                                //    -self.microgrid_status.way1.last_net,
                                //    self.microgrid_status.battery.charge_rate,
                                //    -self.microgrid_status.way1.last_net
                                //        + self.microgrid_status.battery.charge_rate
                                //);
                                self.charge_battery(
                                    -self.microgrid_status.way1.last_net
                                        + self.microgrid_status.battery.charge_rate,
                                    ctx,
                                );
                                self.microgrid_status.battery.charge_rate =
                                    -self.microgrid_status.way1.last_net
                                        + self.microgrid_status.battery.charge_rate;
                            }
                        }
                    }
                    //zero or negative pv
                    _ => {
                        self.microgrid_status.algorithm = ESSCharge;
                        //dbg!("charging battery at rate {}", 125_000.0);

                        self.charge_battery(125_000.0, ctx)
                    }
                }
            }
            NetZeroState::LoadDischarge => {
                match self.microgrid_status.loadbank.state {
                    Some(StateKind::Standby) => {
                        self.microgrid_status.algorithm = NetZeroState::Normal
                    }
                    Some(StateKind::Off) => self.microgrid_status.algorithm = NetZeroState::Normal,
                    Some(StateKind::On) => {
                        self.microgrid_status.algorithm = NetZeroState::LoadDischarge
                    }
                    None => {}
                }
                //info!("discharging");
                match self.microgrid_status.battery.soc {
                    soc if soc <= max_soc_soft => {
                        publish!(
                            self.publisher,
                            LoadControlProfile::loadbank_off_msg(
                                &self
                                    .cfg
                                    .get_str("circuit_segment_devices.load-bank.mrid")
                                    .unwrap(),
                            ),
                            None
                        );
                        self.microgrid_status.algorithm = NetZeroState::Normal;
                    }
                    //keep discharging
                    _ => {}
                }
                //dbg!(
                //    "charging battery at rate {} + {} = {}",
                //    self.microgrid_status.way1.last_net,
                //    self.microgrid_status.battery.charge_rate,
                //    self.microgrid_status.way1.last_net + self.microgrid_status.battery.charge_rate
                //);

                self.charge_battery(
                    -self.microgrid_status.way1.last_net
                        + self.microgrid_status.battery.charge_rate,
                    ctx,
                );
                //self.charge_battery()*/},
            }
        }
    }

    fn islanded_netzero(&mut self, ctx: &Context<MicrogridMsg>) {
        let max_soc_hard = ctx
            .system
            .config()
            .get_float("battery.max_soc_hard")
            .unwrap() as f32;
        let max_soc_soft = ctx
            .system
            .config()
            .get_float("battery.max_soc_soft")
            .unwrap() as f32;
        let min_soc_hard = ctx
            .system
            .config()
            .get_float("battery.min_soc_hard")
            .unwrap() as f32;
        let min_soc_soft = ctx
            .system
            .config()
            .get_float("battery.min_soc_soft")
            .unwrap() as f32;
        info!("islanded state");
        match &self.microgrid_status.algorithm {
            NetZeroState::Disabled | NetZeroState::ShuttingDown => {}
            NetZeroState::Normal => {
                // match self.microgrid_status.loadbank.state {
                //     Some(StateKind::On) => warn!("shouldn't happen"),
                //     Some(StateKind::Off) => {}
                //     Some(StateKind::Standby) => {}
                //     None => {}
                // }
                match self.microgrid_status.shop.reading.to_float()
                    > self.microgrid_status.solar.to_float()
                {
                    //Load is greater than solar
                    true => {
                        match self.microgrid_status.battery.soc {
                            soc if soc > max_soc_hard => {
                                publish!(
                                    self.publisher,
                                    LoadControlProfile::loadbank_on_msg(
                                        &self
                                            .cfg
                                            .get_str("circuit_segment_devices.load-bank.mrid")
                                            .unwrap(),
                                        150000.0,
                                    ),
                                    None
                                );
                                self.microgrid_status.algorithm = NetZeroState::LoadDischarge;
                                self.island(ctx);
                            }
                            soc if soc <= min_soc_hard => {
                                self.reconnect(ctx);
                            }
                            soc if soc <= min_soc_soft => {
                                publish!(
                                    self.publisher,
                                    EssControlProfile::vsi_iso_msg(
                                        &self
                                            .cfg
                                            .get_str("circuit_segment_devices.abb_pcs100.mrid")
                                            .unwrap(),
                                    ),
                                    None
                                );
                                if !self.microgrid_status.generator.disabled {
                                    let msg: GenerationControlProfile =
                                        GenerationControlProfile::generator_on_msg(
                                            &self
                                                .cfg
                                                .get_str(
                                                    "circuit_segment_devices.turbine-array.mrid",
                                                )
                                                .unwrap(),
                                            130000.0,
                                        );
                                    publish!(self.publisher, msg, None);
                                }
                                self.microgrid_status.algorithm = NetZeroState::ESSCharge;
                                //continue operating in normal mode
                            } //would island but are already islanded //self.island(ctx),
                            // soc if soc <= 0.9 => {
                            //     self.microgrid_status.algorithm = NetZeroState::Normal;
                            //     publish!(self.publisher,
                            //         LoadControlProfile::loadbank_off_msg(
                            //             &self.cfg.get_str("circuit_segment_devices.load-bank.mrid").unwrap(),
                            //         )
                            //         .into(),
                            //         None,
                            //     );
                            // }
                            _ => {
                                warn!("doing nothing");
                            } //todo!(),
                        }
                        // dbg!(
                        //     "charging battery at rate {} + {} = {}",
                        //     -self.microgrid_status.way1.last_net,
                        //     self.microgrid_status.battery.charge_rate,
                        //     -self.microgrid_status.way1.last_net + self.microgrid_status.battery.charge_rate
                        // );
                        // self.charge_battery(
                        //     -self.microgrid_status.way1.last_net + self.microgrid_status.battery.charge_rate,
                        //     ctx,
                        // );
                        //self.charge_battery()*/},
                        // info!("load < solar: setting battery charge rate to {}", self.last_battery_charge_rate);
                    }
                    //Load less than solar
                    false => {
                        match self.microgrid_status.battery.soc {
                            soc if soc > max_soc_hard => {
                                publish!(
                                    self.publisher,
                                    LoadControlProfile::loadbank_on_msg(
                                        &self
                                            .cfg
                                            .get_str("circuit_segment_devices.load-bank.mrid")
                                            .unwrap(),
                                        150000.0,
                                    ),
                                    None
                                );
                                self.microgrid_status.algorithm = NetZeroState::LoadDischarge;
                            }
                            soc if soc <= min_soc_hard => {
                                self.reconnect(ctx);
                            }

                            soc if soc <= min_soc_soft => {
                                if !self.microgrid_status.generator.disabled {
                                    let msg: GenerationControlProfile =
                                        GenerationControlProfile::generator_on_msg(
                                            &self
                                                .cfg
                                                .get_str(
                                                    "circuit_segment_devices.turbine-array.mrid",
                                                )
                                                .unwrap(),
                                            130000.0,
                                        );
                                    publish!(self.publisher, msg, None);
                                }
                                self.microgrid_status.algorithm = NetZeroState::ESSCharge;
                            }

                            soc if soc <= max_soc_hard => {
                                self.microgrid_status.algorithm = NetZeroState::Normal;
                                // dbg!("DISABLING LOADBANK");
                                publish!(
                                    self.publisher,
                                    LoadControlProfile::loadbank_off_msg(
                                        &self
                                            .cfg
                                            .get_str("circuit_segment_devices.load-bank.mrid")
                                            .unwrap(),
                                    ),
                                    None
                                );
                            }
                            _ => todo!(),
                        }
                        //dbg!(
                        //    "charging battery at rate {} + {} = {}",
                        //    -self.microgrid_status.way1.last_net,
                        //    self.microgrid_status.battery.charge_rate,
                        //    -self.microgrid_status.way1.last_net
                        //        + self.microgrid_status.battery.charge_rate
                        //);
                        self.charge_battery(
                            -self.microgrid_status.way1.last_net
                                + self.microgrid_status.battery.charge_rate,
                            ctx,
                        );
                        //self.charge_battery()*/},
                        //                             info!("load < solar: setting battery charge rate to {}", self.last_battery_charge_rate);
                    }
                }
            }
            NetZeroState::ESSCharge => match self.microgrid_status.battery.soc {
                soc if soc > max_soc_soft => {
                    self.microgrid_status.algorithm = NetZeroState::Normal;
                    match self.microgrid_status.generator.state.unwrap() {
                        StateKind::On => {
                            warn!("generator turning off");
                            publish!(
                                self.publisher,
                                GenerationControlProfile::generator_off_msg(
                                    &self
                                        .cfg
                                        .get_str("circuit_segment_devices.turbine-array.mrid")
                                        .unwrap(),
                                ),
                                None
                            )
                        }
                        StateKind::Off | StateKind::Standby {} => {}
                    }
                    //dbg!(
                    //    "charging battery at rate {} + {} = {}",
                    //    -self.microgrid_status.way1.last_net,
                    //    self.microgrid_status.battery.charge_rate,
                    //    -self.microgrid_status.way1.last_net
                    //        + self.microgrid_status.battery.charge_rate
                    //);
                    self.charge_battery(
                        -self.microgrid_status.way1.last_net
                            + self.microgrid_status.battery.charge_rate,
                        ctx,
                    );
                }
                soc if soc <= min_soc_hard => {
                    self.reconnect(ctx);
                }
                _ => {}
            },
            NetZeroState::ESSDischarge => {
                match self.microgrid_status.battery.soc {
                    soc if soc > max_soc_soft => {
                        //Staty in ESSDischarge Mode
                        match self.microgrid_status.generator.state.unwrap() {
                            StateKind::Off | StateKind::Standby => {}
                            StateKind::On => publish!(
                                self.publisher,
                                GenerationControlProfile::generator_off_msg(
                                    &self
                                        .cfg
                                        .get_str("circuit_segment_devices.turbine-array.mrid")
                                        .unwrap(),
                                ),
                                None
                            ),
                        }
                        //dbg!(
                        //    "charging battery at rate {} + {} = {}",
                        //    -self.microgrid_status.way1.last_net,
                        //    self.microgrid_status.battery.charge_rate,
                        //    -self.microgrid_status.way1.last_net
                        //        + self.microgrid_status.battery.charge_rate
                        //);
                        self.charge_battery(
                            -self.microgrid_status.way1.last_net
                                + self.microgrid_status.battery.charge_rate,
                            ctx,
                        );
                    }
                    soc if soc <= max_soc_soft => {
                        publish!(
                            self.publisher,
                            LoadControlProfile::loadbank_off_msg(
                                &self
                                    .cfg
                                    .get_str("circuit_segment_devices.load-bank.mrid")
                                    .unwrap(),
                            ),
                            None
                        );
                        self.microgrid_status.battery.charge_rate = 125000.0;
                        //dbg!(
                        //    "charging battery at rate {} + {} = {}",
                        //    -self.microgrid_status.way1.last_net,
                        //    self.microgrid_status.battery.charge_rate,
                        //    -self.microgrid_status.way1.last_net
                        //        + self.microgrid_status.battery.charge_rate
                        //);
                        self.charge_battery(
                            -self.microgrid_status.way1.last_net
                                + self.microgrid_status.battery.charge_rate,
                            ctx,
                        );
                    }
                    soc if soc <= min_soc_hard => {
                        //                        dbg!("soc > 0.20");
                        publish!(
                            self.publisher,
                            LoadControlProfile::loadbank_off_msg(
                                &self
                                    .cfg
                                    .get_str("circuit_segment_devices.load-bank.mrid")
                                    .unwrap(),
                            ),
                            None
                        );
                        //dbg!("charging battery at rate {}", 125_000.0);
                        self.charge_battery(125_000.0, ctx);
                        self.microgrid_status.algorithm = ESSCharge;
                        match self.microgrid_status.generator.disabled {
                            true => self.reconnect(ctx),
                            false => {}
                        }
                    }
                    _ => unreachable!(),
                }
            }
            NetZeroState::LoadDischarge => {
                match self.microgrid_status.loadbank.state {
                    Some(StateKind::Off) | Some(StateKind::Standby) | None => {
                        self.microgrid_status.algorithm = NetZeroState::Normal
                    }
                    Some(StateKind::On) => {
                        self.microgrid_status.algorithm = NetZeroState::LoadDischarge
                    }
                }
                //info!("discharging");
                match self.microgrid_status.battery.soc {
                    soc if soc <= max_soc_soft => {
                        publish!(
                            self.publisher,
                            LoadControlProfile::loadbank_off_msg(
                                &self
                                    .cfg
                                    .get_str("circuit_segment_devices.load-bank.mrid")
                                    .unwrap(),
                            ),
                            None
                        );
                        self.microgrid_status.algorithm = NetZeroState::Normal;
                    }
                    //keep discharging
                    _ => {}
                }
                //dbg!(
                //    "charging battery at rate {} + {} = {}",
                //    -self.microgrid_status.way1.last_net,
                //    self.microgrid_status.battery.charge_rate,
                //    -self.microgrid_status.way1.last_net
                //        + self.microgrid_status.battery.charge_rate
                //);
                self.charge_battery(
                    -self.microgrid_status.way1.last_net
                        + self.microgrid_status.battery.charge_rate,
                    ctx,
                );
                //self.charge_battery()*/},
            }
        }
    }
}

impl Microgrid {
    fn enable_netzero(&mut self, _ctx: &Context<MicrogridMsg>) {
        self.microgrid_status.algorithm = NetZeroState::Normal;
    }

    fn disable_netzero(&mut self, _ctx: &Context<MicrogridMsg>) {
        self.microgrid_status.algorithm = NetZeroState::Disabled;
    }

    fn charge_battery(&mut self, rate_kw: f32, _ctx: &Context<MicrogridMsg>) {
        let grid_connect_mode = match self.microgrid_status.state() {
            MicrogridState::Islanded => Some(GridConnectModeKind::VsiIso),
            MicrogridState::Connected => Some(GridConnectModeKind::VsiPq),
            _ => None,
        };

        if let Some(grid_connect_mode) = grid_connect_mode {
            let ess_msg = EssControlProfile::build_charge_control_profile(
                &self
                    .cfg
                    .get_str("circuit_segment_devices.abb_pcs100.mrid")
                    .unwrap(),
                -rate_kw,
                SystemTime::now(),
                grid_connect_mode,
                1,
            );
            //dbg!(&ess_msg);
            //info!("setting battery charge rate to {}", -self.last_battery_charge_rate);
            publish!(self.publisher, ess_msg, None);
        } else {
            warn!("Attempting to charge battery in unknown microgrid state!");
        }
    }

    fn reconnect_pretest_one(&mut self, _ctx: &Context<MicrogridMsg>) {
        let msg = SwitchControlProfile::switch_synchro_msg(
            &self
                .cfg
                .get_str("circuit_segment_devices.way1.mrid")
                .unwrap(),
            true,
        );
        // dbg!(&msg);
        publish!(self.publisher, msg, None);
        //FIXME
        sleep(Duration::from_secs(2));
        let msg = SwitchControlProfile::switch_synchro_msg(
            &self
                .cfg
                .get_str("circuit_segment_devices.way1.mrid")
                .unwrap(),
            false,
        );
        //dbg!(&msg);
        publish!(self.publisher, msg, None);
    }

    fn reconnect_pretest_two(&mut self, _ctx: &Context<MicrogridMsg>) {
        let msg = EssControlProfile::ess_synchro_msg(
            &self
                .cfg
                .get_str("circuit_segment_devices.abb_pcs100.mrid")
                .unwrap(),
            true,
        );
        //dbg!(&msg);
        publish!(self.publisher, msg, None);
        sleep(Duration::from_secs(2));
        let msg = EssControlProfile::ess_synchro_msg(
            &self
                .cfg
                .get_str("circuit_segment_devices.abb_pcs100.mrid")
                .unwrap(),
            false,
        );
        //dbg!(&msg);
        publish!(self.publisher, msg, None);
    }

    fn reconnect(&mut self, _ctx: &Context<MicrogridMsg>) {
        if self.microgrid_status.ctl_state != MicrogridControlState::Idle {
            warn!("MicrogridControlState is not Idle, Ignoring Request to Reconnect");
            return;
        }

        let state = self.microgrid_status.state();
        if state != MicrogridState::Islanded {
            warn!(
                "MicrogridState is not Islanded, is currently {}, Proceeding to Reconnect Anyways",
                state
            );
        }

        info!("MicrogridControl - Reconnecting...");

        publish!(
            self.publisher,
            GenerationControlProfile::generator_off_msg(
                &self
                    .cfg
                    .get_str("circuit_segment_devices.turbine-array.mrid")
                    .unwrap(),
            ),
            None
        );
        publish!(
            self.publisher,
            SwitchControlProfile::switch_synchro_msg(
                &self
                    .cfg
                    .get_str("circuit_segment_devices.way1.mrid")
                    .unwrap(),
                true,
            ),
            None
        );

        self.microgrid_status
            .update_ctl_state(MicrogridControlState::Reconnecting(
                ReconnectingState::VerifySwitchSynchro,
            ));
    }

    fn reset_microgrid_devices(&mut self, _ctx: &Context<MicrogridMsg>) {
        info!("MicrogridControl - Resetting all microgrid devices!");

        // FIXME use traits here to open/close/enable/disable devices

        //way 1 closed
        publish!(
            self.publisher,
            SwitchControlProfile::switch_close_msg(
                &self
                    .cfg
                    .get_str("circuit_segment_devices.way1.mrid")
                    .unwrap(),
            ),
            None
        );

        //way 2 closed
        publish!(
            self.publisher,
            SwitchControlProfile::switch_close_msg(
                &self
                    .cfg
                    .get_str("circuit_segment_devices.way2.mrid")
                    .unwrap(),
            ),
            None
        );
        //breaker 3 open
        publish!(
            self.publisher,
            BreakerDiscreteControlProfile::breaker_open_msg(
                &self
                    .cfg
                    .get_str("circuit_segment_devices.breaker3.mrid")
                    .unwrap(),
            ),
            None
        );
        //way 4 closed
        publish!(
            self.publisher,
            SwitchControlProfile::switch_close_msg(
                &self
                    .cfg
                    .get_str("circuit_segment_devices.way4.mrid")
                    .unwrap(),
            ),
            None
        );

        //loadbank off
        publish!(
            self.publisher,
            LoadControlProfile::loadbank_off_msg(
                &self
                    .cfg
                    .get_str("circuit_segment_devices.load-bank.mrid")
                    .unwrap(),
            ),
            None
        );

        //solar off
        publish!(
            self.publisher,
            SolarControlProfile::solar_off_msg(
                &self
                    .cfg
                    .get_str("circuit_segment_devices.parker-invert.mrid")
                    .unwrap(),
            ),
            None
        );

        publish!(
            self.publisher,
            EssControlProfile::start_now_gridconnected_msg(
                &self
                    .cfg
                    .get_str("circuit_segment_devices.abb_pcs100.mrid")
                    .unwrap(),
                0.0,
            ),
            None
        );
    }

    fn island(&mut self, _ctx: &Context<MicrogridMsg>) {
        if self.microgrid_status.ctl_state != MicrogridControlState::Idle {
            warn!("MicrogridControlState is not Idle, Ignoring Request to Island, SHOULD ENQUEUE ISLAND REQUEST FOR LATER");
            return;
        }

        let state = self.microgrid_status.state();
        if state != MicrogridState::Connected {
            warn!(
                "MicrogridState is not Connected, is currently {}, Proceeding to Island Anyways",
                state
            );
        }

        info!("MicrogridControl - Islanding...");
        // FIXME use a trait interface for Switch (open/close)
        //open way1
        publish!(
            self.publisher,
            SwitchControlProfile::switch_open_msg(
                &self
                    .cfg
                    .get_str("circuit_segment_devices.way1.mrid")
                    .unwrap(),
            ),
            None
        );
        self.microgrid_status
            .update_ctl_state(MicrogridControlState::Islanding);
    }
}
