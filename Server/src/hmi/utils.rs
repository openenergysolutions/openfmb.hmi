// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::time::SystemTime;

use openfmb::messages::commonmodule::*;
use openfmb::messages::essmodule::*;
use openfmb_messages_ext::ControlProfileExt;

use microgrid_protobuf::generic_control::ControlType;

// TODO:: move to openfmb-rs library
pub fn schedule_ess_control(
    m_rid: &str,
    schedule_parameter_type: ScheduleParameterKind,
    value: f64,
    schedule_time: SystemTime,
) -> EssControlProfile {
    let msg_info: ControlMessageInfo = EssControlProfile::build_control_message_info();
    EssControlProfile {
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
                ess_control_schedule_fsch: None,
                control_fscc: Some(ControlFscc {
                    logical_node_for_control: None,
                    island_control_schedule_fsch: None,
                    control_schedule_fsch: Some(ControlScheduleFsch {
                        val_acsg: Some(ScheduleCsg {
                            sch_pts: vec![SchedulePoint {
                                schedule_parameter: vec![EngScheduleParameter {
                                    schedule_parameter_type: schedule_parameter_type as i32,
                                    value: value,
                                }],
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
                            }],
                        }),
                    }),
                }),
            }),
        }),
    }
}

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

fn create_ess_point(schedule_time: SystemTime) -> EssPoint {
    let mut pt = EssPoint::default();
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

fn ess_point(generic_control: ControlType, schedule_time: SystemTime) -> Option<EssPoint> {
    match generic_control {
        ControlType::BlackStartEnable => {
            let mut pt = create_ess_point(schedule_time);
            pt.black_start_enabled = Some(ControlSpc { ctl_val: true });
            return Some(pt);
        }
        ControlType::BlackStartDisable => {
            let mut pt = create_ess_point(schedule_time);
            pt.black_start_enabled = Some(ControlSpc { ctl_val: false });
            return Some(pt);
        }
        ControlType::FrequencySetPointEnable => {
            let mut pt = create_ess_point(schedule_time);
            pt.frequency_set_point_enabled = Some(ControlSpc { ctl_val: true });
            return Some(pt);
        }
        ControlType::FrequencySetPointDisable => {
            let mut pt = create_ess_point(schedule_time);
            pt.frequency_set_point_enabled = Some(ControlSpc { ctl_val: false });
            return Some(pt);
        }
        ControlType::ReactivePowerSetPointEnable => {
            let mut pt = create_ess_point(schedule_time);
            pt.reactive_pwr_set_point_enabled = Some(ControlSpc { ctl_val: true });
            return Some(pt);
        }
        ControlType::ReactivePowerSetPointDisable => {
            let mut pt = create_ess_point(schedule_time);
            pt.reactive_pwr_set_point_enabled = Some(ControlSpc { ctl_val: false });
            return Some(pt);
        }
        ControlType::RealPowerSetPointEnable => {
            let mut pt = create_ess_point(schedule_time);
            pt.real_pwr_set_point_enabled = Some(ControlSpc { ctl_val: true });
            return Some(pt);
        }
        ControlType::RealPowerSetPointDisable => {
            let mut pt = create_ess_point(schedule_time);
            pt.real_pwr_set_point_enabled = Some(ControlSpc { ctl_val: false });
            return Some(pt);
        }
        ControlType::TransToIslandOnGridLossEnable => {
            let mut pt = create_ess_point(schedule_time);
            pt.trans_to_islnd_on_grid_loss_enabled = Some(ControlSpc { ctl_val: true });
            return Some(pt);
        }
        ControlType::TransToIslandOnGridLossDisable => {
            let mut pt = create_ess_point(schedule_time);
            pt.trans_to_islnd_on_grid_loss_enabled = Some(ControlSpc { ctl_val: false });
            return Some(pt);
        }
        ControlType::VoltageSetPointEnable => {
            let mut pt = create_ess_point(schedule_time);
            pt.voltage_set_point_enabled = Some(ControlSpc { ctl_val: true });
            return Some(pt);
        }
        ControlType::VoltageSetPointDisable => {
            let mut pt = create_ess_point(schedule_time);
            pt.voltage_set_point_enabled = Some(ControlSpc { ctl_val: false });
            return Some(pt);
        }
        ControlType::SetStateKindUndefined => {
            let mut pt = create_ess_point(schedule_time);
            pt.state = Some(OptionalStateKind {
                value: StateKind::Undefined as i32,
            });
            return Some(pt);
        }
        ControlType::SetStateKindOff => {
            let mut pt = create_ess_point(schedule_time);
            pt.state = Some(OptionalStateKind {
                value: StateKind::Off as i32,
            });
            return Some(pt);
        }
        ControlType::SetStateKindOn => {
            let mut pt = create_ess_point(schedule_time);
            pt.state = Some(OptionalStateKind {
                value: StateKind::On as i32,
            });
            return Some(pt);
        }
        ControlType::SetStateKindStandBy => {
            let mut pt = create_ess_point(schedule_time);
            pt.state = Some(OptionalStateKind {
                value: StateKind::Standby as i32,
            });
            return Some(pt);
        }
        _ => {}
    }
    None
}
