use circuit_segment_manager::{
    messages::*,
};
use super::processor::{ProcessorMsg};
use super::profile_subscriber::{ProfileSubscriber, ProfileSubscriberMsg};

use log::info;
use riker::actors::*;
use snafu::Snafu;
use std::{    
    collections::HashMap,       
    sync::Arc
};

use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct NatsMessage(Arc<nats::Message>);

#[actor(
    StartProcessing,
    OpenFMBMessage,
    NatsMessage
)]
#[allow(unused)]
pub struct NATSSubscriber {
    pub message_count: u32,
    nats_broker: nats::Connection,
    openfmb_profile_actors: HashMap<OpenFMBProfileType, ActorRef<ProfileSubscriberMsg>>,    
    processor: ActorRef<ProcessorMsg>    
}

impl
    ActorFactoryArgs<(
        ActorRef<ProcessorMsg>,              
        Connection,
    )> for NATSSubscriber
{
    fn create_args(
        args: (
            ActorRef<ProcessorMsg>,                      
            Connection,
        ),
    ) -> Self {
        NATSSubscriber {
            message_count: 0,
            openfmb_profile_actors: Default::default(),
            processor: args.0,                         
            nats_broker: args.1,          
        }
    }
}

impl Debug for NATSSubscriber {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        unimplemented!()
    }
}

impl NATSSubscriber {
    fn connect_to_nats_broker(&mut self, ctx: &Context<<NATSSubscriber as Actor>::Msg>) {
        info!("Subscriber subscribing to nats");

        let sub = self.nats_broker.subscribe("openfmb.*.*.*").unwrap();

        let myself = ctx.myself.clone();
        // dropping the returned Handler does not unsubscribe here
        sub.with_handler(move |msg| {
            let nats_msg = NatsMessage(Arc::new(msg));
            myself.send_msg(nats_msg.into(), None);
            Ok(())
        });       

        info!("Subscriber successfully subscribed");
    }

    fn ensure_actor(
        &mut self,
        ctx: &Context<NATSSubscriberMsg>,
        msg: &OpenFMBMessage,
    ) -> ActorRef<ProfileSubscriberMsg> {
        use OpenFMBMessage::*;
        match msg {
            GenerationReading(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Generation),
            GenerationStatus(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Generation),
            SwitchStatus(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Switch),
            SwitchReading(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Switch),
            MeterReading(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Meter),
            SolarReading(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Solar),
            SolarStatus(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Solar),
            ESSReading(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::ESS),
            ESSStatus(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::ESS),
            LoadStatus(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Load),
            LoadReading(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Load),            
            RecloserStatus(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Recloser),
            RecloserReading(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Recloser),
            BreakerStatus(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Breaker),
            BreakerReading(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Breaker),
            RegulatorStatus(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Regulator),
            RegulatorReading(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Regulator),
            ResourceStatus(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Resource),
            LoadControl(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Load),
            SwitchControl(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Switch),
            EssControl(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::ESS),
            SolarControl(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Solar),
            GeneratorControl(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Generation),
            BreakerControl(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Breaker), 
            ResourceControl(_msg) => self.ensure_actor_type(ctx, OpenFMBProfileType::Resource),
        }
    }

    fn ensure_actor_type(
        &mut self,
        ctx: &Context<NATSSubscriberMsg>,
        profile_type: OpenFMBProfileType,
    ) -> ActorRef<ProfileSubscriberMsg> {
        let actor = self
            .openfmb_profile_actors
            .get(&profile_type);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let actor_ref = ctx
                    .actor_of_args::<ProfileSubscriber, ActorRef<ProcessorMsg>>(
                        &format!("{:?}", profile_type),
                        self.processor.clone(),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(profile_type, actor_ref.clone());
                    actor_ref
            }
        }
    }    
}

impl Actor for NATSSubscriber {
    type Msg = NATSSubscriberMsg;

    fn pre_start(&mut self, _ctx: &Context<<NATSSubscriber as Actor>::Msg>) {}

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

impl Receive<StartProcessing> for NATSSubscriber {
    type Msg = NATSSubscriberMsg;

    fn receive(&mut self, ctx: &Context<Self::Msg>, _msg: StartProcessing, _sender: Sender) {
        //   info!("openfmb subscriber received msg {:?}", msg.clone());
        self.connect_to_nats_broker(ctx);
    }
}

impl Receive<OpenFMBMessage> for NATSSubscriber {
    type Msg = NATSSubscriberMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: OpenFMBMessage, _sender: Sender) {
        
        let actor = self.ensure_actor(ctx, &msg);
        actor.send_msg(msg.clone().into(), ctx.myself.clone());
    }
}

use serde::export::Formatter;

use nats::Connection;
use std::fmt::Debug;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Actor System Error"))]
    IOError { source: std::io::Error },
    #[snafu(display("Unsupported OpenFMB type"))]
    UnsupportedOpenFMBTypeError { fmb_type: String },
}

impl Receive<NatsMessage> for NATSSubscriber {
    type Msg = NATSSubscriberMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: NatsMessage, _sender: Sender) {
        match msg.0.subject.as_str() {            
            _ => {
                
                let result: Result<OpenFMBMessage, _> = msg.0.as_ref().try_into();
                if let Ok(msg) = result {                    
                    let actor = self.ensure_actor(ctx, &msg);
                    actor.send_msg(msg.clone().into(), ctx.myself.clone()); 
                }    
                else {
                    println!("Message is not supported.");
                }                              
            }
        }        
    }
}
