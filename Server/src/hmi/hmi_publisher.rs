use super::nats_publisher::{NATSPublisher, NATSPublisherMsg};
use circuit_segment_manager::messages::*;

use openfmb_messages::breakermodule::BreakerDiscreteControlProfile;
use openfmb_messages_ext::breaker::BreakerControlExt;

use openfmb_messages::switchmodule::SwitchDiscreteControlProfile;
use openfmb_messages_ext::switch::SwitchControlExt;
use prost::Message;
use nats::Connection;

use riker::actors::*;
use std::fmt::Debug;

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
}

impl ActorFactoryArgs<Connection> for HmiPublisher {
    fn create_args(args: Connection) -> Self {
        HmiPublisher {
            message_count: 0,
            nats_client: args,            
            openfmb_nats_publisher: None,            
        }
    }
}

impl HmiPublisher {
    pub fn get_child_actors(
        &mut self,
    ) ->  Option<ActorRef<NATSPublisherMsg>>        
    {                   
        self.openfmb_nats_publisher.clone()                    
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
                            
        let profile_name = msg.profile_name.clone().unwrap();
        use microgrid_protobuf as microgrid;
        match profile_name.clone().as_str() {
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
                    }                    
                }            
            },
            _ => {
                println!("Unsupported generic control for profile {}", profile_name);                
            }
        }
    } 
}

 