// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::time::SystemTime;

use openfmb::messages::commonmodule::*;
use openfmb::messages::essmodule::*;
use openfmb::messages::generationmodule::*;
use openfmb::messages::regulatormodule::*;
use openfmb::messages::solarmodule::*;
use openfmb_messages_ext::ControlProfileExt;

use microgrid_protobuf::generic_control::ControlType;

// ESS

pub fn set_ess_csg(
    m_rid: &str,
    generic_control: ControlType,
    schedule_time: SystemTime,
) -> Option<EssControlProfile> {
    match ess_point(generic_control, schedule_time) {
        Some(point) => {
            let msg_info: ControlMessageInfo = EssControlProfile::build_control_message_info();

            Some(EssControlProfile {
                control_message_info: Some(msg_info),
                ess: Some(Ess {
                    conducting_equipment: Some(ConductingEquipment {
                        m_rid: m_rid.to_string(),
                        named_object: None,
                    }),
                }),
                ess_control: Some(EssControl {
                    check: None,
                    control_value: None,
                    ess_control_fscc: Some(EssControlFscc {
                        control_fscc: None,
                        ess_control_schedule_fsch: Some(EssControlScheduleFsch {
                            val_dcsg: Some(Esscsg {
                                crv_pts: vec![point],
                            }),
                        }),
                    }),
                }),
            })
        }
        None => None,
    }
}

fn create_ess_point(schedule_time: SystemTime) -> EssCurvePoint {
    EssCurvePoint {
        start_time: Some(ControlTimestamp {
            nanoseconds: schedule_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos(),
            seconds: schedule_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }),
        control: None,
    }
}

fn ess_point(generic_control: ControlType, schedule_time: SystemTime) -> Option<EssCurvePoint> {
    match generic_control {
        ControlType::BlackStartEnable => {
            let mut pt = create_ess_point(schedule_time);
            pt.control = Some(EssPoint {
                black_start_enabled: Some(ControlSpc { ctl_val: true }),
                ..Default::default()
            });
            return Some(pt);
        }
        ControlType::BlackStartDisable => {
            let mut pt = create_ess_point(schedule_time);
            pt.control = Some(EssPoint {
                black_start_enabled: Some(ControlSpc { ctl_val: false }),
                ..Default::default()
            });
            return Some(pt);
        }
        ControlType::TransToIslandOnGridLossEnable => {
            let mut pt = create_ess_point(schedule_time);
            pt.control = Some(EssPoint {
                trans_to_islnd_on_grid_loss_enabled: Some(ControlSpc { ctl_val: true }),
                ..Default::default()
            });
            return Some(pt);
        }
        ControlType::TransToIslandOnGridLossDisable => {
            let mut pt = create_ess_point(schedule_time);
            pt.control = Some(EssPoint {
                trans_to_islnd_on_grid_loss_enabled: Some(ControlSpc { ctl_val: false }),
                ..Default::default()
            });
            return Some(pt);
        }
        ControlType::SetStateKindUndefined => {
            let mut pt = create_ess_point(schedule_time);
            pt.control = Some(EssPoint {
                state: Some(OptionalStateKind {
                    value: StateKind::Undefined as i32,
                }),
                ..Default::default()
            });
            return Some(pt);
        }
        ControlType::SetStateKindOff => {
            let mut pt = create_ess_point(schedule_time);
            pt.control = Some(EssPoint {
                state: Some(OptionalStateKind {
                    value: StateKind::Off as i32,
                }),
                ..Default::default()
            });
            return Some(pt);
        }
        ControlType::SetStateKindOn => {
            let mut pt = create_ess_point(schedule_time);
            pt.control = Some(EssPoint {
                state: Some(OptionalStateKind {
                    value: StateKind::On as i32,
                }),
                ..Default::default()
            });
            return Some(pt);
        }
        ControlType::SetStateKindStandBy => {
            let mut pt = create_ess_point(schedule_time);
            pt.control = Some(EssPoint {
                state: Some(OptionalStateKind {
                    value: StateKind::Standby as i32,
                }),
                ..Default::default()
            });
            return Some(pt);
        }
        _ => {}
    }
    None
}

// Solar

pub fn set_solar_csg(
    m_rid: &str,
    generic_control: ControlType,
    schedule_time: SystemTime,
) -> Option<SolarControlProfile> {
    match solar_point(generic_control, schedule_time) {
        Some(point) => {
            let msg_info: ControlMessageInfo = SolarControlProfile::build_control_message_info();

            Some(SolarControlProfile {
                control_message_info: Some(msg_info),
                solar_inverter: Some(SolarInverter {
                    conducting_equipment: Some(ConductingEquipment {
                        m_rid: m_rid.to_string(),
                        named_object: None,
                    }),
                }),
                solar_control: Some(SolarControl {
                    check: None,
                    control_value: None,
                    solar_control_fscc: Some(SolarControlFscc {
                        control_fscc: None,
                        solar_control_schedule_fsch: Some(SolarControlScheduleFsch {
                            val_dcsg: Some(SolarCsg {
                                crv_pts: vec![point],
                            }),
                        }),
                    }),
                }),
            })
        }
        None => None,
    }
}

fn create_solar_point(schedule_time: SystemTime) -> SolarCurvePoint {
    SolarCurvePoint {
        start_time: Some(ControlTimestamp {
            nanoseconds: schedule_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos(),
            seconds: schedule_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }),
        control: None,
    }
}

fn solar_point(generic_control: ControlType, schedule_time: SystemTime) -> Option<SolarCurvePoint> {
    match generic_control {
        ControlType::SetStateKindUndefined => {
            let mut pt = create_solar_point(schedule_time);
            pt.control = Some(SolarPoint {
                state: Some(OptionalStateKind {
                    value: StateKind::Undefined as i32,
                }),
                ..Default::default()
            });
            return Some(pt);
        }
        ControlType::SetStateKindOff => {
            let mut pt = create_solar_point(schedule_time);
            pt.control = Some(SolarPoint {
                state: Some(OptionalStateKind {
                    value: StateKind::Off as i32,
                }),
                ..Default::default()
            });
            return Some(pt);
        }
        ControlType::SetStateKindOn => {
            let mut pt = create_solar_point(schedule_time);
            pt.control = Some(SolarPoint {
                state: Some(OptionalStateKind {
                    value: StateKind::On as i32,
                }),
                ..Default::default()
            });
            return Some(pt);
        }
        ControlType::SetStateKindStandBy => {
            let mut pt = create_solar_point(schedule_time);
            pt.control = Some(SolarPoint {
                state: Some(OptionalStateKind {
                    value: StateKind::Standby as i32,
                }),
                ..Default::default()
            });
            return Some(pt);
        }
        _ => {
            log::error!(
                "Unsupported control type for solar point: {:?}",
                generic_control
            );
        }
    }
    None
}

// Generation

pub fn set_generation_csg(
    m_rid: &str,
    generic_control: ControlType,
    schedule_time: SystemTime,
) -> Option<GenerationControlProfile> {
    match generation_point(generic_control, schedule_time) {
        Some(point) => {
            let msg_info: ControlMessageInfo =
                GenerationControlProfile::build_control_message_info();

            Some(GenerationControlProfile {
                control_message_info: Some(msg_info),
                generating_unit: Some(GeneratingUnit {
                    conducting_equipment: Some(ConductingEquipment {
                        m_rid: m_rid.to_string(),
                        named_object: None,
                    }),
                    ..Default::default()
                }),
                generation_control: Some(GenerationControl {
                    check: None,
                    control_value: None,
                    generation_control_fscc: Some(GenerationControlFscc {
                        control_fscc: None,
                        generation_control_schedule_fsch: Some(GenerationControlScheduleFsch {
                            val_dcsg: Some(GenerationCsg {
                                crv_pts: vec![point],
                            }),
                        }),
                    }),
                }),
            })
        }
        None => None,
    }
}

fn create_generation_point(schedule_time: SystemTime) -> GenerationPoint {
    let mut pt = GenerationPoint::default();
    pt.start_time = Some(ControlTimestamp {
        nanoseconds: schedule_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos(),
        seconds: schedule_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    });
    pt
}

fn generation_point(
    generic_control: ControlType,
    schedule_time: SystemTime,
) -> Option<GenerationPoint> {
    match generic_control {
        ControlType::FrequencySetPointEnable => {
            let mut pt = create_generation_point(schedule_time);
            pt.frequency_set_point_enabled = Some(ControlSpc { ctl_val: true });
            return Some(pt);
        }
        ControlType::FrequencySetPointDisable => {
            let mut pt = create_generation_point(schedule_time);
            pt.frequency_set_point_enabled = Some(ControlSpc { ctl_val: false });
            return Some(pt);
        }
        ControlType::ReactivePowerSetPointEnable => {
            let mut pt = create_generation_point(schedule_time);
            pt.reactive_pwr_set_point_enabled = Some(ControlSpc { ctl_val: true });
            return Some(pt);
        }
        ControlType::ReactivePowerSetPointDisable => {
            let mut pt = create_generation_point(schedule_time);
            pt.reactive_pwr_set_point_enabled = Some(ControlSpc { ctl_val: false });
            return Some(pt);
        }
        ControlType::RealPowerSetPointEnable => {
            let mut pt = create_generation_point(schedule_time);
            pt.real_pwr_set_point_enabled = Some(ControlSpc { ctl_val: true });
            return Some(pt);
        }
        ControlType::RealPowerSetPointDisable => {
            let mut pt = create_generation_point(schedule_time);
            pt.real_pwr_set_point_enabled = Some(ControlSpc { ctl_val: false });
            return Some(pt);
        }
        ControlType::VoltageSetPointEnable => {
            let mut pt = create_generation_point(schedule_time);
            pt.voltage_set_point_enabled = Some(ControlSpc { ctl_val: true });
            return Some(pt);
        }
        ControlType::VoltageSetPointDisable => {
            let mut pt = create_generation_point(schedule_time);
            pt.voltage_set_point_enabled = Some(ControlSpc { ctl_val: false });
            return Some(pt);
        }
        ControlType::SetStateKindUndefined => {
            let mut pt = create_generation_point(schedule_time);
            pt.state = Some(OptionalStateKind {
                value: StateKind::Undefined as i32,
            });
            return Some(pt);
        }
        ControlType::SetStateKindOff => {
            let mut pt = create_generation_point(schedule_time);
            pt.state = Some(OptionalStateKind {
                value: StateKind::Off as i32,
            });
            return Some(pt);
        }
        ControlType::SetStateKindOn => {
            let mut pt = create_generation_point(schedule_time);
            pt.state = Some(OptionalStateKind {
                value: StateKind::On as i32,
            });
            return Some(pt);
        }
        ControlType::SetStateKindStandBy => {
            let mut pt = create_generation_point(schedule_time);
            pt.state = Some(OptionalStateKind {
                value: StateKind::Standby as i32,
            });
            return Some(pt);
        }
        _ => {
            log::error!(
                "Unsupported control type for generation point: {:?}",
                generic_control
            );
        }
    }
    None
}

// Regulator

pub fn set_regulator_csg(
    m_rid: &str,
    generic_control: ControlType,
    schedule_time: SystemTime,
) -> Option<RegulatorControlProfile> {
    match regulator_point(generic_control, schedule_time) {
        Some(point) => {
            let msg_info: ControlMessageInfo =
                RegulatorControlProfile::build_control_message_info();

            Some(RegulatorControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        m_rid: m_rid.to_string(),
                        named_object: None,
                    }),
                }),
                regulator_control: Some(RegulatorControl {
                    check: None,
                    control_value: None,
                    regulator_control_fscc: Some(RegulatorControlFscc {
                        control_fscc: None,
                        regulator_control_schedule_fsch: Some(RegulatorControlScheduleFsch {
                            val_dcsg: Some(RegulatorCsg {
                                crv_pts: vec![point],
                            }),
                        }),
                    }),
                }),
            })
        }
        None => None,
    }
}

fn create_regulator_point(schedule_time: SystemTime) -> RegulatorPoint {
    let mut pt = RegulatorPoint::default();
    pt.start_time = Some(Timestamp {
        nanoseconds: schedule_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos(),
        seconds: schedule_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        tq: None,
    });
    pt.control = Some(RegulatorControlAtcc::default());
    pt
}

fn regulator_point(
    generic_control: ControlType,
    schedule_time: SystemTime,
) -> Option<RegulatorPoint> {
    let _pt = create_regulator_point(schedule_time);
    // TODO
    match generic_control {
        _ => {}
    }
    None
}
