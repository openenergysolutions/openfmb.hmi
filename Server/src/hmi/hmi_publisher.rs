use super::nats_publisher::{NATSPublisher, NATSPublisherMsg};
use circuit_segment_manager::messages::*;

use openfmb_messages::breakermodule::BreakerDiscreteControlProfile;
use openfmb_messages_ext::breaker::BreakerControlExt;

use openfmb_messages::switchmodule::SwitchDiscreteControlProfile;
use openfmb_messages_ext::switch::SwitchControlExt;

use openfmb_messages::essmodule::EssControlProfile;
use openfmb_messages_ext::ess::EssControlExt;

use openfmb_messages::loadmodule::LoadControlProfile;
use openfmb_messages_ext::load::LoadControlExt;

use openfmb_messages::solarmodule::SolarControlProfile;
use openfmb_messages_ext::solar::SolarControlExt;

use openfmb_messages::resourcemodule::ResourceDiscreteControlProfile;
use openfmb_messages_ext::resource::ResourceControlExt;

use prost::Message;
use nats::Connection;

use riker::actors::*;
use std::fmt::Debug;
use config::Config;
use log::info;
use crate::handler::*;

#[actor(
    OpenFMBMessage, 
    MicrogridControl,
    DeviceControl,
    GenericControl
)]
#[derive(Clone, Debug)]
pub struct HmiPublisher {
    pub message_count: u32,
    pub nats_client: nats::Connection,    
    pub openfmb_nats_publisher: Option<ActorRef<NATSPublisherMsg>>,
    pub cfg: Config,
    pub devices: Vec<Equipment>,
}

impl ActorFactoryArgs<(Connection, Config)> for HmiPublisher {
    fn create_args((args, config): (Connection, Config)) -> Self {
        HmiPublisher {
            message_count: 0,
            nats_client: args,            
            openfmb_nats_publisher: None,             
            devices: initial_devices(&config),
            cfg: config,
        }
    }
}

fn initial_devices(config: &Config) -> Vec<Equipment> {          
    let mut equipment_list = vec![];

    if let Ok(list) = config.get_array("circuit_segment_devices.all_devices") {
        for item in list {
            let device_name = item.into_str().unwrap();
            
            let eq = Equipment {
                name: config.get_str(&format!("circuit_segment_devices.{}.name", device_name)).unwrap(),
                mrid: config.get_str(&format!("circuit_segment_devices.{}.mrid", device_name)).unwrap(),
                device_type: match config.get_str(&format!("circuit_segment_devices.{}.type", device_name)) {
                    Ok(t) => Some(t),
                    _ => None,
                },
            };            
            equipment_list.push(eq);
        }
    }        
    equipment_list
}

impl HmiPublisher {
    pub fn get_child_actors(
        &mut self,
    ) ->  Option<ActorRef<NATSPublisherMsg>>        
    {                   
        self.openfmb_nats_publisher.clone()                    
    }
    fn get_device_type_by_mrid(&self, mrid: String) -> Option<String> {
        for eq in self.devices.iter() {
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
}
impl Actor for HmiPublisher {
    type Msg = HmiPublisherMsg;

    fn pre_start(&mut self, ctx: &Context<Self::Msg>) {       
        self.openfmb_nats_publisher = Some(
            ctx.actor_of_args::<NATSPublisher, Connection>(
                "HmiNATSPublisher",
                self.nats_client.clone(),
            )
            .unwrap(),
        );        
    }

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

    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: MicrogridControl, sender: Sender) {      
        println!("Publisher received microgrid control message {:?}", msg);    
        match &self.openfmb_nats_publisher {
            Some(child) => {
                child.tell(msg.clone(), sender.clone());
            }
            None => {}
        }
    } 
}

impl Receive<DeviceControl> for HmiPublisher {
    type Msg = HmiPublisherMsg;    

    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: DeviceControl, sender: Sender) {      
        println!("Publisher received device control message {:?}", msg); 
        match &self.openfmb_nats_publisher {
            Some(child) => {
                child.tell(msg.clone(), sender.clone());
            }
            None => {}
        }       
    } 
}

impl Receive<GenericControl> for HmiPublisher {
    type Msg = HmiPublisherMsg;    
   
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: GenericControl, _sender: Sender) {           
                            
        let mut profile_name = String::from("");
        if let Some(p) = msg.profile_name.clone() {
            profile_name = p.clone();
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
                        self.nats_client.publish(&subject, &mut buffer).unwrap();
                    }
                    microgrid::generic_control::ControlType::Close => {
                        let profile = BreakerDiscreteControlProfile::breaker_close_msg(
                            &msg.mrid
                        );  
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                        
                        self.nats_client.publish(&subject, &mut buffer).unwrap();                      
                    }
                    _ => {}
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
                        self.nats_client.publish(&subject, &mut buffer).unwrap();
                    }
                    microgrid::generic_control::ControlType::Close => {
                        let profile = SwitchDiscreteControlProfile::switch_close_msg(
                            &msg.mrid
                        );  
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                        
                        self.nats_client.publish(&subject, &mut buffer).unwrap();                 
                    }
                    microgrid::generic_control::ControlType::SetModBlkOn => {
                        let profile = SwitchDiscreteControlProfile::switch_modblk_msg(
                            &msg.mrid,
                            true
                        ); 
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                        
                        self.nats_client.publish(&subject, &mut buffer).unwrap();                  
                    }
                    microgrid::generic_control::ControlType::SetModBlkOff => {
                        let profile = SwitchDiscreteControlProfile::switch_modblk_msg(
                            &msg.mrid,
                            false
                        );
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                        
                        self.nats_client.publish(&subject, &mut buffer).unwrap();                   
                    },
                    _ => {}                    
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
                        self.nats_client.publish(&subject, &mut buffer).unwrap();                  
                    }
                    microgrid::generic_control::ControlType::SetModBlkOff => {
                        let profile = EssControlProfile::ess_modblk_msg(
                            &msg.mrid,
                            false
                        );
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                        
                        self.nats_client.publish(&subject, &mut buffer).unwrap();                   
                    },
                    microgrid::generic_control::ControlType::SetValue => {                        
                        let profile = EssControlProfile::discharge_now_msg(
                            &msg.mrid,
                            msg.args.unwrap()
                        ); 
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                        
                        self.nats_client.publish(&subject, &mut buffer).unwrap();                  
                    }
                    _ => {}                   
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
                        self.nats_client.publish(&subject, &mut buffer).unwrap();                  
                    }
                    microgrid::generic_control::ControlType::SetModBlkOff => {
                        let profile = SolarControlProfile::solar_modblk_msg(
                            &msg.mrid,
                            false
                        );
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                        
                        self.nats_client.publish(&subject, &mut buffer).unwrap();                   
                    },
                    _ => {}                   
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
                        self.nats_client.publish(&subject, &mut buffer).unwrap();                  
                    }
                    microgrid::generic_control::ControlType::StateOff => {
                        let profile = LoadControlProfile::loadbank_off_msg(
                            &msg.mrid                            
                        );
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                        
                        self.nats_client.publish(&subject, &mut buffer).unwrap();                   
                    },
                    microgrid::generic_control::ControlType::SetWNetMag | microgrid::generic_control::ControlType::SetValue => {                        
                        let profile = LoadControlProfile::loadbank_on_msg(
                            &msg.mrid,
                            msg.args.unwrap().abs(),
                        ); 
                        let mut buffer = Vec::<u8>::new();                        
                        profile.encode(&mut buffer).unwrap();                        
                        self.nats_client.publish(&subject, &mut buffer).unwrap();                  
                    }
                    _ => {}                   
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
                        self.nats_client.publish(&subject, &mut buffer).unwrap();                  
                    }
                    _ => {}                   
                } 
            }
            _ => {
                println!("Unsupported generic control for profile {}", profile_name);                
            }
        }
    } 
}

 