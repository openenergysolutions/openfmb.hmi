use crate::actors::microgrid::MicrogridMsg;
use std::env;

use super::super::{CoordinatorMsg, Device, DeviceMsg, Publisher, PublisherMsg};
use crate::{
    actors::coordinator::subscriber::generic_openfmb_profile_subscriber::{
        GenericOpenFMBProfileSubscriber, GenericOpenFMBProfileSubscriberMsg,
    },
    messages::*,
};

use log::{debug, error, info, warn};
use prost::Message;
use riker::actors::*;
use snafu::{OptionExt, ResultExt, Snafu};
use std::{
    any::Any,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

// use crate::actors::coordinator::publisher::elasticsearch_publisher::{
//     ElasticSearchPublisherMsg,
// };
use bytes::BytesMut;

use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct NatsMessage(Arc<nats::Message>);

#[actor(
    StartProcessing,
    RequestActorStats,
    OpenFMBMessage,
    NatsMessage,
    LoadControlProfile
)]
#[allow(unused)]
pub struct OpenFMBNATSSubscriber {
    pub message_count: u32,
    nats_broker: nats::Connection,
    openfmb_profile_actors:
        HashMap<OpenFMBProfileType, ActorRef<GenericOpenFMBProfileSubscriberMsg>>,
    publisher: ActorRef<PublisherMsg>,
    processor: ActorRef<DeviceMsg>,
    microgrid: ActorRef<MicrogridMsg>,
}

impl
    ActorFactoryArgs<(
        ActorRef<DeviceMsg>,
        ActorRef<PublisherMsg>,
        ActorRef<MicrogridMsg>,
        Connection,
    )> for OpenFMBNATSSubscriber
{
    fn create_args(
        args: (
            ActorRef<DeviceMsg>,
            ActorRef<PublisherMsg>,
            ActorRef<MicrogridMsg>,
            Connection,
        ),
    ) -> Self {
        OpenFMBNATSSubscriber {
            message_count: 0,
            nats_broker: args.3,
            openfmb_profile_actors: Default::default(),
            publisher: args.1,
            processor: args.0,
            microgrid: args.2,
        }
    }
}

impl Debug for OpenFMBNATSSubscriber {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        unimplemented!()
    }
}

impl OpenFMBNATSSubscriber {
    fn connect_to_nats_broker(&mut self, ctx: &Context<<OpenFMBNATSSubscriber as Actor>::Msg>) {
        info!("Subscriber subscribing to nats");

        let sub = self.nats_broker.subscribe("openfmb.*.*.*").unwrap();

        let myself = ctx.myself.clone();
        // dropping the returned Handler does not unsubscribe here
        sub.with_handler(move |msg| {
            let nats_msg = NatsMessage(Arc::new(msg));
            myself.send_msg(nats_msg.into(), None);
            Ok(())
        });

        let sub = self.nats_broker.subscribe("microgridui.*").unwrap();

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
        ctx: &Context<OpenFMBNATSSubscriberMsg>,
        msg: &OpenFMBMessage,
    ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
        use OpenFMBMessage::*;
        match msg {
            GenerationReading(_msg) => self.ensure_generator_actor(ctx),
            GenerationStatus(_msg) => self.ensure_generator_actor(ctx),
            SwitchStatus(_msg) => self.ensure_switch_actor(ctx),
            SwitchReading(_msg) => self.ensure_switch_actor(ctx),
            MeterReading(_msg) => self.ensure_meter_actor(ctx),
            SolarReading(_msg) => self.ensure_solar_actor(ctx),
            SolarStatus(_msg) => self.ensure_solar_actor(ctx),
            ESSReading(_msg) => self.ensure_ess_actor(ctx),
            ESSStatus(_msg) => self.ensure_ess_actor(ctx),
            LoadStatus(_msg) => self.ensure_load_actor(ctx),
            LoadReading(_msg) => self.ensure_load_actor(ctx),
            // ShuntStatus(_msg) => self.ensure_shunt_actor(ctx),
            // ShuntReading(_msg) => self.ensure_shunt_actor(ctx),
            RecloserStatus(_msg) => self.ensure_recloser_actor(ctx),
            RecloserReading(_msg) => self.ensure_recloser_actor(ctx),
            BreakerStatus(_msg) => self.ensure_breaker_actor(ctx),
            BreakerReading(_msg) => self.ensure_breaker_actor(ctx),
            RegulatorStatus(_msg) => self.ensure_regulator_actor(ctx),
            RegulatorReading(_msg) => self.ensure_regulator_actor(ctx),
            LoadControl(_msg) => self.ensure_load_actor(ctx),
            SwitchControl(_msg) => self.ensure_switch_actor(ctx),
            EssControl(_msg) => self.ensure_ess_actor(ctx),
            SolarControl(_msg) => self.ensure_solar_actor(ctx),
            GeneratorControl(_msg) => self.ensure_generator_actor(ctx),
            BreakerControl(_msg) => self.ensure_generator_actor(ctx),
            // ResourceStatus(_msg) => self.ensure_resource_actor(ctx),
        }
    }

    fn ensure_breaker_actor(
        &mut self,
        ctx: &Context<OpenFMBNATSSubscriberMsg>,
    ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
        let actor = self
            .openfmb_profile_actors
            .get(&OpenFMBProfileType::Breaker);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let breaker_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfileSubscriber, ActorRef<DeviceMsg>>(
                        "BreakerHandler",
                        self.processor.clone(),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::Breaker, breaker_actor_ref.clone());
                breaker_actor_ref
            }
        }
    }

    fn ensure_regulator_actor(
        &mut self,
        ctx: &Context<OpenFMBNATSSubscriberMsg>,
    ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
        let actor = self
            .openfmb_profile_actors
            .get(&OpenFMBProfileType::Regulator);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let regulator_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfileSubscriber, ActorRef<DeviceMsg>>(
                        "RegulatorHandler",
                        self.processor.clone(),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::Regulator, regulator_actor_ref.clone());
                regulator_actor_ref
            }
        }
    }

    fn ensure_load_actor(
        &mut self,
        ctx: &Context<OpenFMBNATSSubscriberMsg>,
    ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
        let actor = self.openfmb_profile_actors.get(&OpenFMBProfileType::Load);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let load_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfileSubscriber, ActorRef<DeviceMsg>>(
                        "LoadHandler",
                        self.processor.clone(),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::Load, load_actor_ref.clone());
                load_actor_ref
            }
        }
    }

    fn ensure_recloser_actor(
        &mut self,
        ctx: &Context<OpenFMBNATSSubscriberMsg>,
    ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
        let actor = self
            .openfmb_profile_actors
            .get(&OpenFMBProfileType::Recloser);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let load_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfileSubscriber, ActorRef<DeviceMsg>>(
                        "RecloserHandler",
                        self.processor.clone(),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::Recloser, load_actor_ref.clone());
                load_actor_ref
            }
        }
    }

    fn ensure_shunt_actor(
        &mut self,
        ctx: &Context<OpenFMBNATSSubscriberMsg>,
    ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
        let actor = self.openfmb_profile_actors.get(&OpenFMBProfileType::Shunt);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let shunt_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfileSubscriber, ActorRef<DeviceMsg>>(
                        "ShuntHandler",
                        self.processor.clone(),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::Shunt, shunt_actor_ref.clone());
                shunt_actor_ref
            }
        }
    }

    fn ensure_ess_actor(
        &mut self,
        ctx: &Context<OpenFMBNATSSubscriberMsg>,
    ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
        let actor = self.openfmb_profile_actors.get(&OpenFMBProfileType::ESS);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let ess_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfileSubscriber, ActorRef<DeviceMsg>>(
                        "ESSHandler",
                        self.processor.clone(),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::ESS, ess_actor_ref.clone());
                ess_actor_ref
            }
        }
    }

    fn ensure_solar_actor(
        &mut self,
        ctx: &Context<OpenFMBNATSSubscriberMsg>,
    ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
        let actor = self.openfmb_profile_actors.get(&OpenFMBProfileType::Solar);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let solar_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfileSubscriber, ActorRef<DeviceMsg>>(
                        "SolarHandler",
                        self.processor.clone(),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::Solar, solar_actor_ref.clone());
                solar_actor_ref
            }
        }
    }

    fn ensure_meter_actor(
        &mut self,
        ctx: &Context<OpenFMBNATSSubscriberMsg>,
    ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
        let actor = self.openfmb_profile_actors.get(&OpenFMBProfileType::Meter);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let meter_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfileSubscriber, ActorRef<DeviceMsg>>(
                        "MeterHandler",
                        self.processor.clone(),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::Meter, meter_actor_ref.clone());
                meter_actor_ref
            }
        }
    }

    fn ensure_switch_actor(
        &mut self,
        ctx: &Context<OpenFMBNATSSubscriberMsg>,
    ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
        let actor = self.openfmb_profile_actors.get(&OpenFMBProfileType::Switch);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let switch_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfileSubscriber, ActorRef<DeviceMsg>>(
                        "SwitchHandler",
                        self.processor.clone(),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::Switch, switch_actor_ref.clone());
                switch_actor_ref
            }
        }
    }

    fn ensure_generator_actor(
        &mut self,
        ctx: &Context<OpenFMBNATSSubscriberMsg>,
    ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
        let actor = self
            .openfmb_profile_actors
            .get(&OpenFMBProfileType::Generation);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let generation_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfileSubscriber, ActorRef<DeviceMsg>>(
                        "GenerationHandler",
                        self.processor.clone(),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::Generation, generation_actor_ref.clone());
                generation_actor_ref
            }
        }
    }

    fn ensure_resource_actor(
        &mut self,
        ctx: &Context<OpenFMBNATSSubscriberMsg>,
    ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
        let actor = self
            .openfmb_profile_actors
            .get(&OpenFMBProfileType::Resource);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let resource_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfileSubscriber, ActorRef<DeviceMsg>>(
                        "ResourceHandler",
                        self.processor.clone(),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::Resource, resource_actor_ref.clone());
                resource_actor_ref
            }
        }
    }

    // fn ensure_cnc_actor(
    //     &mut self,
    //     ctx: &Context<OpenFMBNATSSubscriberMsg>,
    // ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
    //     match self
    //         .openfmb_profile_actors
    //         .get(&OpenFMBProfileType::Resource)
    //     {
    //         Some(&actor_ref) => actor_ref.clone(),
    //         None => {
    //             let load_actor_ref = ctx
    //                 .actor_of_args::<GenericOpenFMBProfileSubscriber,ActorRef<DeviceMsg>>(
    //                     "LoadrHandler",
    //                     self.processor.clone(),
    //                 )
    //                 .unwrap();
    //             self.openfmb_profile_actors
    //                 .insert(OpenFMBProfileType::Resource, resource_actor_ref.clone());
    //             resource_actor_ref
    //         }
    //     }
    // }
}

impl Actor for OpenFMBNATSSubscriber {
    type Msg = OpenFMBNATSSubscriberMsg;

    fn pre_start(&mut self, _ctx: &Context<<OpenFMBNATSSubscriber as Actor>::Msg>) {}

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

impl Receive<StartProcessing> for OpenFMBNATSSubscriber {
    type Msg = OpenFMBNATSSubscriberMsg;

    fn receive(&mut self, ctx: &Context<Self::Msg>, _msg: StartProcessing, _sender: Sender) {
        //   info!("openfmb subscriber received msg {:?}", msg.clone());
        self.connect_to_nats_broker(ctx);
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Actor System Error"))]
    IOError { source: std::io::Error },
    #[snafu(display("Unsupported OpenFMB type"))]
    UnsupportedOpenFMBTypeError { fmb_type: String },
}

impl Receive<RequestActorStats> for OpenFMBNATSSubscriber {
    type Msg = OpenFMBNATSSubscriberMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: RequestActorStats, sender: Sender) {
        let stats_msg: CoordinatorMsg = ActorStats {
            message_count: self.message_count,
            persisted_message_count: None,
        }
        .into();
        sender
            .clone()
            .unwrap()
            .try_tell(stats_msg, Some(ctx.myself.clone().into()))
            .unwrap();
        for child in self.openfmb_profile_actors.clone() {
            child.1.send_msg(msg.clone().into(), sender.clone());
        }
        let _stats_msg: CoordinatorMsg = ActorStats {
            message_count: self.message_count,
            persisted_message_count: None,
        }
        .into();
    }
}

impl Receive<LoadControlProfile> for OpenFMBNATSSubscriber {
    type Msg = OpenFMBNATSSubscriberMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: LoadControlProfile, _sender: Sender) {
        //dbg!(msg.clone());
        //   let mut buffer = BufMut::default();
        //  buffer
        let mut buffer = BytesMut::with_capacity(4096);
        msg.encode(&mut buffer).unwrap();
        // self.nats_broker
        //     .as_ref()
        //     .unwrap()
        //     .publish("?", &mut buffer, None)
        //     .unwrap();
    }
}

impl Receive<OpenFMBMessage> for OpenFMBNATSSubscriber {
    type Msg = OpenFMBNATSSubscriberMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: OpenFMBMessage, _sender: Sender) {
        //dbg!(msg.clone());
        let actor = self.ensure_actor(ctx, &msg);
        actor.send_msg(msg.clone().into(), ctx.myself.clone());

        // let elastic_publisher = ctx.system.select("/user/Publisher/ElasticPublisher").unwrap();
        // let msg: ElasticSearchPublisherMsg = msg.into();
        // elastic_publisher.try_tell(msg, None);
    }
}

use openfmb_messages::loadmodule::LoadControlProfile;
use serde::export::Formatter;

use microgrid_protobuf::MicrogridControl;
use nats::Connection;
use std::fmt::Debug;
//use crate::actors::SwitchState::Open;

impl Receive<NatsMessage> for OpenFMBNATSSubscriber {
    type Msg = OpenFMBNATSSubscriberMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: NatsMessage, _sender: Sender) {
        match msg.0.subject.as_str() {
            "openfmb.microgrid.control.reset" => {
                std::thread::sleep(Duration::from_millis(500));
                let bytes = base64::decode(&msg.0.data).unwrap();
                let r: MicrogridControl = MicrogridControl::decode(bytes.as_slice()).unwrap();
                // dbg!(&r);
                self.microgrid.send_msg(r.into(), ctx.myself.clone());
            }
            "microgridui.microgrid_control" => {
                let buf: &[u8] = &msg.0.as_ref().data;
                match microgrid_protobuf::MicrogridControl::decode(buf) {
                    Ok(ctl_msg) => {
                        debug!("Received ui microgrid_control msg {:?}", ctl_msg);
                        self.microgrid.tell(ctl_msg, None);
                    }
                    Err(err) => {
                        error!(
                            "Error received message type for micgrogrid_control topic, {:?}",
                            err
                        );
                    }
                }
            }
            "microgridui.device_control" => {
                let buf: &[u8] = &msg.0.as_ref().data;
                match microgrid_protobuf::DeviceControl::decode(buf) {
                    Ok(ctl_msg) => {
                        debug!("Received ui microgrid_control msg {:?}", ctl_msg);
                        if let Some(variant) =
                            microgrid_protobuf::device_control::DeviceControlMessage::from_i32(
                                ctl_msg.msg,
                            )
                        {
                            self.microgrid.tell(variant, None);
                        } else {
                            warn!(
                                "Unknown enum variant for device control message {:?}",
                                ctl_msg
                            );
                        }
                    }
                    Err(err) => {
                        error!(
                            "Error received message type for device_control topic, {:?}",
                            err
                        );
                    }
                }
            }
            _val => {
                let msg: OpenFMBMessage = msg.0.as_ref().try_into().unwrap();
                let actor = self.ensure_actor(ctx, &msg);
                actor.send_msg(msg.clone().into(), ctx.myself.clone());
                // if let Some(gui) = &self.gui {
                //     gui.send_msg(msg.clone().into(), ctx.myself.clone());
                // }
            }
        }
        //        let elastic_publisher = ctx.system.select("/user/Publisher/ElasticPublisher").unwrap();
        // let msg: ElasticSearchPublisherMsg = msg.into();
        // elastic_publisher.try_tell(msg, None);
    }
}
