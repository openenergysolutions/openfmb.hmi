use circuit_segment_manager::messages::*;
use prost::Message;
use log::info;
use riker::actors::*;
use snafu::Snafu;
use std::sync::Arc;

use crate::handler::*;

#[derive(Debug, Clone)]
pub struct NatsMessage(Arc<nats::Message>);

#[actor(    
    OpenFMBMessage,
    MicrogridControl,
    DeviceControl
)]
pub struct NATSPublisher {
    pub message_count: u32,
    nats_broker: nats::Connection,
}

impl ActorFactoryArgs<nats::Connection> for NATSPublisher {
    fn create_args(args: nats::Connection) -> Self {
        NATSPublisher {
            message_count: 0,
            nats_broker: args,
        }
    }
}

impl Actor for NATSPublisher {
    type Msg = NATSPublisherMsg;

    fn pre_start(&mut self, _ctx: &Context<<NATSPublisher as Actor>::Msg>) {}

    fn post_start(&mut self, _ctx: &Context<Self::Msg>) {}

    fn post_stop(&mut self) {}

    fn supervisor_strategy(&self) -> Strategy {
        Strategy::Restart
    }

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

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Actor System Error"))]
    IOError { source: std::io::Error },
    #[snafu(display("Unsupported OpenFMB type"))]
    UnsupportedOpenFMBTypeError { fmb_type: String },
}

impl Receive<OpenFMBMessage> for NATSPublisher {
    type Msg = NATSPublisherMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: OpenFMBMessage, _sender: Sender) {
        //
    }
}

impl Receive<MicrogridControl> for NATSPublisher {
    type Msg = NATSPublisherMsg;    

    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: MicrogridControl, _sender: Sender) {      
        let subject = "microgridui.microgrid_control";
        info!("Sending {:?} to NATS topic {}", msg, subject);
        let mut buffer = Vec::<u8>::new();
        msg.message.encode(&mut buffer);
        self.nats_broker.publish(&subject, &mut buffer).unwrap();
    } 
}

impl Receive<DeviceControl> for NATSPublisher {
    type Msg = NATSPublisherMsg;    
   
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: DeviceControl, _sender: Sender) {           
        let subject = "microgridui.device_control";
        info!("Sending {:?} to NATS topic {}", msg, subject);
        let mut buffer = Vec::<u8>::new();
        let device_control_msg = microgrid_protobuf::DeviceControl { mrid: "".to_string(), msg: msg.message.into() };
        device_control_msg.encode(&mut buffer).unwrap();
        self.nats_broker.publish(&subject, &mut buffer).unwrap();    
    } 
}