use crate::messages::*;

use super::processor::ProcessorMsg;

use super::{
    nats_subscriber::{NATSSubscriber, NATSSubscriberMsg}
};

use log::{info, warn};

use nats::Connection;
use riker::actors::*;
use std::fmt::Debug;

#[actor(StartProcessing)]
#[derive(Clone, Debug)]
pub struct HmiSubscriber {
    pub message_count: u32,
    pub nats_client: nats::Connection,    
    pub openfmb_nats_subscriber: Option<ActorRef<NATSSubscriberMsg>>,        
    pub processor: ActorRef<ProcessorMsg>,       
}
impl
    ActorFactoryArgs<(        
        ActorRef<ProcessorMsg>,        
        Connection,
    )> for HmiSubscriber
{
    fn create_args(
        args: (           
            ActorRef<ProcessorMsg>,            
            Connection,
        ),
    ) -> Self {
        HmiSubscriber {
            message_count: 0,                       
            openfmb_nats_subscriber: None,                        
            processor: args.0,            
            nats_client: args.1,           
        }
    }
}

impl Actor for HmiSubscriber {
    type Msg = HmiSubscriberMsg;

    fn pre_start(&mut self, ctx: &Context<Self::Msg>) {        
        self.openfmb_nats_subscriber = Some(
            ctx.actor_of_args::<NATSSubscriber, (
                ActorRef<ProcessorMsg>,                                
                Connection,
            )>(
                "HmiNATSSubscriber",
                (
                    self.processor.clone(),                                       
                    self.nats_client.clone(),
                ),
            )
            .unwrap(),
        );
    }

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
        self.receive(ctx, msg.clone(), sender);
    }
}

impl Receive<StartProcessing> for HmiSubscriber {
    type Msg = HmiSubscriberMsg;

    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: StartProcessing, sender: Sender) {
        info!(
            "subscriber got a start_processing message from {:?}",
            sender
        );
        match sender {
            Some(_sender) => {               
            }
            None => warn!("no sender"),
        };        

        self.openfmb_nats_subscriber
            .as_ref()
            .unwrap()
            .tell(StartProcessing, None);
    }
}