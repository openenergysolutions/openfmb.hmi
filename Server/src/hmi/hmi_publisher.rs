use super::nats_publisher::{NATSPublisher, NATSPublisherMsg};
use circuit_segment_manager::messages::*;

use nats::Connection;

use riker::actors::*;
use std::fmt::Debug;

use crate::handler::*;

#[actor(
    OpenFMBMessage, 
    MicrogridControl,
    DeviceControl
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
 