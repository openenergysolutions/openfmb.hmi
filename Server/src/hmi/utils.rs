// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::time::SystemTime;

use openfmb_messages::essmodule::*;
use openfmb_messages_ext::ControlProfileExt;
use openfmb_messages::commonmodule::*;

// TODO:: move to openfmb-rs library
pub fn schedule_ess_control(
    m_rid: &str,
    schedule_parameter_type: ScheduleParameterKind,
    value: f64,     
    schedule_time: SystemTime) -> EssControlProfile{

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
                            sch_pts: vec![
                                SchedulePoint {
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
                                },                                
                            ],
                        }),
                    }),                    
                }),                
            }),            
        }),        
    }
}