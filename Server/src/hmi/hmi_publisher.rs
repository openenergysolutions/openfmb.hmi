// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::messages::*;

use openfmb_messages::commonmodule::ScheduleParameterKind;

use openfmb_messages::breakermodule::BreakerDiscreteControlProfile;
use openfmb_messages_ext::breaker::BreakerControlExt;

use openfmb_messages::reclosermodule::RecloserDiscreteControlProfile;
use openfmb_messages_ext::recloser::RecloserControlExt;

use openfmb_messages::switchmodule::SwitchDiscreteControlProfile;
use openfmb_messages_ext::switch::SwitchControlExt;

use openfmb_messages::essmodule::EssControlProfile;
use openfmb_messages_ext::ess::EssControlExt;

use openfmb_messages::loadmodule::LoadControlProfile;
use openfmb_messages_ext::load::LoadControlExt;

use openfmb_messages::generationmodule::GenerationControlProfile;
use openfmb_messages_ext::generation::GenerationControlExt;

use openfmb_messages::solarmodule::SolarControlProfile;
use openfmb_messages_ext::solar::SolarControlExt;

use openfmb_messages::resourcemodule::ResourceDiscreteControlProfile;
use openfmb_messages_ext::resource::ResourceControlExt;

use openfmb_messages::regulatormodule::RegulatorDiscreteControlProfile;
use openfmb_messages_ext::regulator::RegulatorDiscreteControlExt;

use prost::Message;

use riker::actors::*;
use std::fmt::Debug;
use std::time::SystemTime;
use config::Config;
use log::{debug, info, warn, error};
use crate::handler::*;
use super::utils::*;

#[actor(
    OpenFMBMessage, 
    MicrogridControl,
    DeviceControl,
    GenericControl,
    StartProcessingMessages
)]
#[derive(Clone, Debug)]
pub struct HmiPublisher {
    pub message_count: u32,
    pub nats_client: Option<nats::Connection>,        
    pub cfg: Config,    
}

impl ActorFactoryArgs<Config> for HmiPublisher {
    fn create_args(config: Config) -> Self {
        HmiPublisher {
            message_count: 0,
            nats_client: None,                                                
            cfg: config,
        }
    }
}

impl HmiPublisher { 
    fn connect_to_nats_broker(&mut self, _ctx: &Context<<HmiPublisher as Actor>::Msg>, msg: &StartProcessingMessages) {               
        if let Some(c) = &msg.nats_client{
            self.nats_client = Some(c.clone());
        }
        else {
            info!("HmiPublisher connects to NATS with options: {:?}", msg.pubsub_options);
            match msg.pubsub_options.connect() {
                Ok(connection) => {
                    info!("****** HmiPublisher successfully connected");                

                    self.nats_client = Some(connection);
                }
                Err(e) => {
                    error!("Unable to connect to nats.  {:?}", e);
                }
            }  
        }   
    }

    fn get_device_type_by_mrid(&self, mrid: String) -> Option<String> {
        let devices = read_equipment_list().ok()?;
        for eq in devices.iter() {
           if eq.mrid == mrid {
               return eq.device_type.clone();
           }
        }
        None
    }
    fn get_common_control_profile(&self, mrid: String) -> Option<String> {

        let t = match self.get_device_type_by_mrid(mrid) {
            Some(device_type) => {
                match device_type.as_str() {
                    "switch" => Some("SwitchDiscreteControlProfile".to_string()),
                    "breaker" => Some("BreakerDiscreteControlProfile".to_string()),
                    "recloser" => Some("RecloserDiscreteControlProfile".to_string()),
                    "ess" => Some("EssControlProfile".to_string()),
                    "solar" => Some("SolarControlProfile".to_string()),
                    "generator" => Some("GenerationDiscreteControlProfile".to_string()),
                    "regulator" => Some("RegulatorDiscreteControlProfile".to_string()),
                    "load" => Some("LoadControlProfile".to_string()),
                    _ => {
                        info!("Unable to get common control profile for device type: {}", device_type);
                        None
                    }
                }
            },
            None => {
                None
            }
        };
        t
    }
    fn publish(&self, subject: &str, buffer: &mut Vec::<u8>) {
        if let Some(connnection) = &self.nats_client {
            match connnection.publish(subject, buffer) {
                Ok(_) => {
                    debug!("Message with subject {} published succesfully.", subject)
                },
                Err(e) => {
                    error!("Error publishing message: {:?}", e);
                }
            }
        }        
        else {
            error!("NATS connection is not available.");
        }
    }
}
impl Actor for HmiPublisher {
    type Msg = HmiPublisherMsg;

    fn pre_start(&mut self, _ctx: &Context<Self::Msg>) {}

    fn post_start(&mut self, _ctx: &Context<Self::Msg>) {}

    fn post_stop(&mut self) {}

    fn sys_recv(
        &mut self,
        _ctx: &Context<Self::Msg>,
        _msg: SystemMsg,
        _sender: Option<BasicActorRef>,
    ) {
    }

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) {
        self.message_count += 1;
        self.receive(ctx, msg, sender);
    }
}

impl Receive<GetChildActors> for HmiPublisher {
    type Msg = HmiPublisherMsg;

    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: GetChildActors, _sender: Sender) {
        self.message_count += 1;        
    }
}

impl Receive<OpenFMBMessage> for HmiPublisher {
    type Msg = HmiPublisherMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: OpenFMBMessage, _sender: Sender) {
        
    }
}

impl Receive<MicrogridControl> for HmiPublisher {
    type Msg = HmiPublisherMsg;    

    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: MicrogridControl, _sender: Sender) {      
        let subject = "openfmb.microgridui.microgrid_control";
        info!("Sending {:?} to NATS topic {}", msg, subject);
        let mut buffer = Vec::<u8>::new();
        msg.message.encode(&mut buffer);
        self.publish(&subject, &mut buffer);        
    } 
}

impl Receive<DeviceControl> for HmiPublisher {
    type Msg = HmiPublisherMsg;    

    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: DeviceControl, _sender: Sender) {           
        let subject = "openfmb.microgridui.device_control";
        info!("Sending {:?} to NATS topic {}", msg, subject);
        let mut buffer = Vec::<u8>::new();
        let device_control_msg = microgrid_protobuf::DeviceControl { mrid: "".to_string(), msg: msg.message.into() };
        device_control_msg.encode(&mut buffer).unwrap();        
        self.publish(&subject, &mut buffer);
    } 
}

impl Receive<GenericControl> for HmiPublisher {
    type Msg = HmiPublisherMsg;    
   
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: GenericControl, _sender: Sender) {           
        let token:  Vec<&str> = msg.text.split(".").collect();                    
        let mut profile_name = String::from("");
        if let Some(p) = msg.profile_name.clone() {
            profile_name = p.clone();
        }
        else if token.len() > 1 && token[0].ends_with("Profile") {
            profile_name = token[0].to_string()
        }
        else if let Some(p) = self.get_common_control_profile(msg.mrid.clone().to_string()){
            profile_name = p.clone();
        }        
        
        use microgrid_protobuf as microgrid;
        match profile_name.as_str() {
            "BreakerDiscreteControlProfile" => {
                let subject = format!("openfmb.breakermodule.BreakerDiscreteControlProfile.{}", &msg.mrid);
                match msg.message {
                    microgrid::generic_control::ControlType::Open => {
                        let profile = BreakerDiscreteControlProfile::breaker_open_msg(
                            &msg.mrid
                        );
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                
                        self.publish(&subject, &mut buffer);
                    }                    
                    microgrid::generic_control::ControlType::Close => {
                        let profile = BreakerDiscreteControlProfile::breaker_close_msg(
                            &msg.mrid
                        );  
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                              
                        self.publish(&subject, &mut buffer);
                    }
                    _ => {
                        warn!("Unsupport control type: {:?}", msg.message)
                    }
                }
            },
            "RecloserDiscreteControlProfile" => {
                let subject = format!("openfmb.reclosermodule.RecloserDiscreteControlProfile.{}", &msg.mrid);
                match msg.message {
                    microgrid::generic_control::ControlType::Open => {
                        let profile = RecloserDiscreteControlProfile::recloser_open_msg(
                            &msg.mrid
                        );
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                
                        self.publish(&subject, &mut buffer);
                    }
                    microgrid::generic_control::ControlType::Close => {
                        let profile = RecloserDiscreteControlProfile::recloser_close_msg(
                            &msg.mrid
                        );  
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                      
                        self.publish(&subject, &mut buffer);
                    }
                    _ => {
                        warn!("Unsupport control type: {:?}", msg.message)
                    }
                }
            },
            "SwitchDiscreteControlProfile" => {
                let subject = format!("openfmb.switchmodule.SwitchDiscreteControlProfile.{}", &msg.mrid);
                match msg.message {
                    microgrid::generic_control::ControlType::Open => {
                        let profile = SwitchDiscreteControlProfile::switch_open_msg(
                            &msg.mrid
                        );
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                              
                        self.publish(&subject, &mut buffer);
                    }
                    microgrid::generic_control::ControlType::Close => {
                        let profile = SwitchDiscreteControlProfile::switch_close_msg(
                            &msg.mrid
                        );  
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                
                        self.publish(&subject, &mut buffer);
                    }
                    microgrid::generic_control::ControlType::SetModBlkOn => {
                        let profile = SwitchDiscreteControlProfile::switch_modblk_msg(
                            &msg.mrid,
                            true
                        ); 
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                  
                        self.publish(&subject, &mut buffer);
                    }
                    microgrid::generic_control::ControlType::SetModBlkOff => {
                        let profile = SwitchDiscreteControlProfile::switch_modblk_msg(
                            &msg.mrid,
                            false
                        );
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                  
                        self.publish(&subject, &mut buffer);
                    },
                    _ => {
                        warn!("Unsupport control type: {:?}", msg.message)
                    }                    
                }            
            },
            "ESSControlProfile" | "EssControlProfile" => {
                let subject = format!("openfmb.essmodule.ESSControlProfile.{}", &msg.mrid);
                match msg.message {                    
                    microgrid::generic_control::ControlType::SetModBlkOn => {
                        let profile = EssControlProfile::ess_modblk_msg(
                            &msg.mrid,
                            true
                        ); 
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                  
                        self.publish(&subject, &mut buffer);
                    }
                    microgrid::generic_control::ControlType::SetModBlkOff => {
                        let profile = EssControlProfile::ess_modblk_msg(
                            &msg.mrid,
                            false
                        );
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                   
                        self.publish(&subject, &mut buffer);
                    },
                    microgrid::generic_control::ControlType::SetValue => {                        
                        let profile = EssControlProfile::discharge_now_msg(
                            &msg.mrid,
                            msg.args.unwrap()
                        ); 
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                 
                        self.publish(&subject, &mut buffer);
                    }
                    microgrid::generic_control::ControlType::SetWNetMag => {                        
                        let profile = schedule_ess_control(&msg.mrid, ScheduleParameterKind::WNetMag, msg.args.unwrap(), SystemTime::now());
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                 
                        self.publish(&subject, &mut buffer);
                    }
                    microgrid::generic_control::ControlType::SetVarNetMag => {                        
                        let profile = schedule_ess_control(&msg.mrid, ScheduleParameterKind::VArNetMag, msg.args.unwrap(), SystemTime::now());
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                 
                        self.publish(&subject, &mut buffer);
                    }                    
                    _ => {

                        match set_ess_csg(&msg.mrid, msg.message, SystemTime::now()) {
                            Some(profile) => {
                                let mut buffer = Vec::<u8>::new();                        
                                profile.encode(&mut buffer).unwrap();                                                                 
                                self.publish(&subject, &mut buffer);
                            }
                            None => {
                                warn!("Unsupport control type: {:?}", msg.message)
                            }
                        }                        
                    }                  
                } 
            },
            "SolarControlProfile" => {
                let subject = format!("openfmb.solarmodule.SolarControlProfile.{}", &msg.mrid);
                match msg.message {                    
                    microgrid::generic_control::ControlType::SetModBlkOn => {
                        let profile = SolarControlProfile::solar_modblk_msg(
                            &msg.mrid,
                            true
                        ); 
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                 
                        self.publish(&subject, &mut buffer);
                    }
                    microgrid::generic_control::ControlType::SetModBlkOff => {
                        let profile = SolarControlProfile::solar_modblk_msg(
                            &msg.mrid,
                            false
                        );
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                   
                        self.publish(&subject, &mut buffer);
                    },
                    _ => {
                        warn!("Unsupport control type: {:?}", msg.message)
                    }                 
                } 
            },
            "LoadControlProfile" => {
                let subject = format!("openfmb.loadmodule.LoadControlProfile.{}", &msg.mrid);
                match msg.message {                    
                    microgrid::generic_control::ControlType::StateOn => {
                        let profile = LoadControlProfile::loadbank_on_msg(
                            &msg.mrid,
                            125000.0
                        ); 
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                  
                        self.publish(&subject, &mut buffer);
                    }
                    microgrid::generic_control::ControlType::StateOff => {
                        let profile = LoadControlProfile::loadbank_off_msg(
                            &msg.mrid                            
                        );
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                   
                        self.publish(&subject, &mut buffer);
                    },
                    microgrid::generic_control::ControlType::SetWNetMag | microgrid::generic_control::ControlType::SetValue => {                        
                        let profile = LoadControlProfile::loadbank_on_msg(
                            &msg.mrid,
                            msg.args.unwrap().abs(),
                        ); 
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                  
                        self.publish(&subject, &mut buffer);
                    }
                    _ => {
                        warn!("Unsupport control type: {:?}", msg.message)
                    }                   
                } 
            },
            "ResourceDiscreteControlProfile" => {
                let subject = format!("openfmb.resourcemodule.ResourceDiscreteControlProfile.{}", &msg.mrid);
                match msg.message {                    
                    microgrid::generic_control::ControlType::SetValue => {
                        let profile = ResourceDiscreteControlProfile::set_analog_msg(
                            &msg.mrid,
                            msg.args.unwrap()
                        ); 
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                               
                        self.publish(&subject, &mut buffer);
                    }
                    _ => {
                        warn!("Unsupport control type: {:?}", msg.message)
                    }                   
                } 
            }
            "RegulatorDiscreteControlProfile" => {
                let subject = format!("openfmb.regulatormodule.RegulatorDiscreteControlProfile.{}", &msg.mrid);
                match msg.message {                    
                    microgrid::generic_control::ControlType::TapChangeLowerPhs3 => {
                        let profile = RegulatorDiscreteControlProfile::regulator_tap_lower_phs3_msg(
                            &msg.mrid,
                            false
                        ); 
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                  
                        self.publish(&subject, &mut buffer);
                    }
                    microgrid::generic_control::ControlType::TapChangeRaisePhs3 => {
                        let profile = RegulatorDiscreteControlProfile::regulator_tap_raise_phs3_msg(
                            &msg.mrid,
                            true
                        ); 
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                  
                        self.publish(&subject, &mut buffer);
                    }
                    microgrid::generic_control::ControlType::TapChangeLowerPhsA => {
                        let profile = RegulatorDiscreteControlProfile::regulator_tap_lower_phs_a_msg(
                            &msg.mrid,
                            false
                        ); 
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                  
                        self.publish(&subject, &mut buffer);
                    }
                    microgrid::generic_control::ControlType::TapChangeRaisePhsA => {
                        let profile = RegulatorDiscreteControlProfile::regulator_tap_raise_phs_a_msg(
                            &msg.mrid,
                            true
                        ); 
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                  
                        self.publish(&subject, &mut buffer);
                    }
                    microgrid::generic_control::ControlType::TapChangeLowerPhsB => {
                        let profile = RegulatorDiscreteControlProfile::regulator_tap_lower_phs_b_msg(
                            &msg.mrid,
                            false
                        ); 
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                 
                        self.publish(&subject, &mut buffer);
                    }
                    microgrid::generic_control::ControlType::TapChangeRaisePhsB => {
                        let profile = RegulatorDiscreteControlProfile::regulator_tap_raise_phs_b_msg(
                            &msg.mrid,
                            true
                        ); 
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                  
                        self.publish(&subject, &mut buffer);
                    }
                    microgrid::generic_control::ControlType::TapChangeLowerPhsC => {
                        let profile = RegulatorDiscreteControlProfile::regulator_tap_lower_phs_c_msg(
                            &msg.mrid,
                            false
                        ); 
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                
                        self.publish(&subject, &mut buffer);
                    }
                    microgrid::generic_control::ControlType::TapChangeRaisePhsC => {
                        let profile = RegulatorDiscreteControlProfile::regulator_tap_raise_phs_c_msg(
                            &msg.mrid,
                            true
                        ); 
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                  
                        self.publish(&subject, &mut buffer);
                    }
                    _ => {
                        warn!("Unsupport control type: {:?}", msg.message)
                    }                   
                } 
            },
            "GenerationControlProfile" => {
                let subject = format!("openfmb.generationmodule.GenerationControlProfile.{}", &msg.mrid);
                match msg.message {                   
                    microgrid::generic_control::ControlType::SetWNetMag | microgrid::generic_control::ControlType::SetValue => {                             
                        let profile = GenerationControlProfile::generator_on_msg(
                            &msg.mrid,
                            msg.args.unwrap().abs(),
                        ); 
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                                                                  
                        self.publish(&subject, &mut buffer);
                    }
                    _ => {
                        warn!("Unsupport control type: {:?}", msg.message)
                    }                   
                }
            }
            _ => {
                println!("Unsupported generic control for profile {}", profile_name);                
            }
        }
    } 
}

impl Receive<StartProcessingMessages> for HmiPublisher {
    type Msg = HmiPublisherMsg;

    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: StartProcessingMessages, _sender: Sender) {        
        self.connect_to_nats_broker(ctx, &msg);
    }
}

 