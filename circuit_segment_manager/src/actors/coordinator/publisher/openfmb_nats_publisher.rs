use super::super::CoordinatorMsg;

use crate::messages::*;
use microgrid_protobuf::CoordinationStatus;

use log::info;
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

use OpenFMBCommon;

use prost::Message;

#[derive(Debug, Clone)]
pub struct NatsMessage(Arc<nats::Message>);

#[actor(
    StartProcessing,
    RequestActorStats,
    OpenFMBMessage,
    NatsMessage,
    LoadControlProfile,
    SwitchControlProfile,
    EssControlProfile,
    SolarControlProfile,
    GenerationControlProfile,
    BreakerDiscreteControlProfile,
    CoordinationStatus
)]
pub struct OpenFMBNATSPublisher {
    pub message_count: u32,
    //openfmb_profile_actors: HashMap<OpenFMBProfileType, ActorRef<GenericOpenFMBProfileSubscriberMsg>>,
    // publisher: ActorRef<PublisherMsg>,
    // processor: ActorRef<DeviceMsg>,
    nats_broker: nats::Connection,
}

impl ActorFactoryArgs<Connection> for OpenFMBNATSPublisher {
    fn create_args(args: Connection) -> Self {
        OpenFMBNATSPublisher {
            message_count: 0,
            nats_broker: args,
        }
    }
}

impl OpenFMBNATSPublisher {
    fn connect_to_nats_broker(&mut self, ctx: &Context<<OpenFMBNATSPublisher as Actor>::Msg>) {
        info!("Publisher subscribing to nats");

        let sub = self.nats_broker.subscribe("openfmb.*.*.*").unwrap();

        let myself = ctx.myself.clone();
        // dropping the returned Handler does not unsubscribe here
        sub.with_handler(move |msg: nats::Message| {
            let nats_msg = NatsMessage(Arc::new(msg));
            myself.send_msg(nats_msg.into(), None);
            Ok(())
        });

        info!("Publisher successfully subscribed");
    }
}

impl Actor for OpenFMBNATSPublisher {
    type Msg = OpenFMBNATSPublisherMsg;

    fn pre_start(&mut self, ctx: &Context<<OpenFMBNATSPublisher as Actor>::Msg>) {
        self.connect_to_nats_broker(ctx);
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
        self.receive(ctx, msg, sender);
    }
}

impl Receive<StartProcessing> for OpenFMBNATSPublisher {
    type Msg = OpenFMBNATSPublisherMsg;

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

impl Receive<RequestActorStats> for OpenFMBNATSPublisher {
    type Msg = OpenFMBNATSPublisherMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, _msg: RequestActorStats, sender: Sender) {
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
        let _stats_msg: CoordinatorMsg = ActorStats {
            message_count: self.message_count,
            persisted_message_count: None,
        }
        .into();
    }
}

impl Receive<OpenFMBMessage> for OpenFMBNATSPublisher {
    type Msg = OpenFMBNATSPublisherMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: OpenFMBMessage, _sender: Sender) {
        //dbg!(msg.clone());
    }
}

use openfmb_ops_protobuf::openfmb::{
    essmodule::EssControlProfile, generationmodule::GenerationControlProfile,
    loadmodule::LoadControlProfile, solarmodule::SolarControlProfile,
    switchmodule::SwitchControlProfile,
};

use nats::Connection;
use openfmb_ops_protobuf::openfmb::breakermodule::BreakerDiscreteControlProfile;

impl Receive<NatsMessage> for OpenFMBNATSPublisher {
    type Msg = OpenFMBNATSPublisherMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: NatsMessage, _sender: Sender) {
        // use OpenFMBError::InvalidOpenFMBMessage;
        // let msg: OpenFMBMessage = msg.try_into().context(InvalidOpenFMBMessage)

        //        let elastic_publisher = ctx.system.select("/user/Publisher/ElasticPublisher").unwrap();
        // let msg: ElasticSearchPublisherMsg = msg.into();
        // elastic_publisher.try_tell(msg, None);
    }
}

impl Receive<LoadControlProfile> for OpenFMBNATSPublisher {
    type Msg = OpenFMBNATSPublisherMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: LoadControlProfile, _sender: Sender) {
        //dbg!(msg.clone());
        //   let mut buffer = BufMut::default();
        //  buffer
        let mut buffer = vec![];
        msg.encode(&mut buffer).unwrap();
        let mrid = msg.device_mrid().unwrap();
        let mut subject = String::default();
        subject.push_str("openfmb.loadmodule.LoadControlProfile.");
        subject.push_str(&mrid.to_string());
        self.nats_broker.publish(&subject, &mut buffer).unwrap();
        //    info!("publishing load control to {:?}", mrid);
        //  dbg!(msg);
        //info!("{:?}", subject);
    }
}

impl Receive<SwitchControlProfile> for OpenFMBNATSPublisher {
    type Msg = OpenFMBNATSPublisherMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: SwitchControlProfile, _sender: Sender) {
        let mut buffer = vec![];
        msg.encode(&mut buffer).unwrap();
        let mrid = msg.device_mrid().unwrap();
        let mut subject = String::default();
        subject.push_str("openfmb.switchmodule.SwitchControlProfile.");
        subject.push_str(&mrid.to_string());
        self.nats_broker.publish(&subject, &mut buffer).unwrap();

        //     info!("publishing switch control to {:?}", mrid);
        //   dbg!(msg);
        //  info!("{:?}", subject);
    }
}

impl Receive<SolarControlProfile> for OpenFMBNATSPublisher {
    type Msg = OpenFMBNATSPublisherMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: SolarControlProfile, _sender: Sender) {
        let mut buffer = vec![];
        msg.encode(&mut buffer).unwrap();
        let mrid = msg.device_mrid().unwrap();
        let mut subject = String::default();
        subject.push_str("openfmb.solarmodule.SolarControlProfile.");
        subject.push_str(&mrid.to_string());
        self.nats_broker.publish(&subject, &mut buffer).unwrap();

        //     info!("publishing switch control to {:?}", mrid);
        //   dbg!(msg);
        //info!("{:?}", subject);
    }
}

impl Receive<BreakerDiscreteControlProfile> for OpenFMBNATSPublisher {
    type Msg = OpenFMBNATSPublisherMsg;
    fn receive(
        &mut self,
        _ctx: &Context<Self::Msg>,
        msg: BreakerDiscreteControlProfile,
        _sender: Sender,
    ) {
        let mut buffer = vec![];
        msg.encode(&mut buffer).unwrap();
        let mrid = msg.device_mrid().unwrap();
        let mut subject = String::default();
        subject.push_str("openfmb.breakermodule.BreakerDiscreteControlProfile.");
        subject.push_str(&mrid.to_string());
        self.nats_broker.publish(&subject, &mut buffer).unwrap();

        //     info!("publishing switch control to {:?}", mrid);
        // info!("{:?}", msg);
        // info!("{:?}", subject);
    }
}

impl Receive<EssControlProfile> for OpenFMBNATSPublisher {
    type Msg = OpenFMBNATSPublisherMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: EssControlProfile, _sender: Sender) {
        let mut buffer = vec![];
        msg.encode(&mut buffer).unwrap();
        let mrid = msg.device_mrid().unwrap();
        let mut subject = String::default();
        subject.push_str("openfmb.essmodule.ESSControlProfile.");
        subject.push_str(&mrid.to_string());
        self.nats_broker.publish(&subject, &mut buffer).unwrap();

        //     info!("publishing switch control to {:?}", mrid);
        // info!("{:?}", msg);
        // info!("{:?}", subject);
    }
}

impl Receive<GenerationControlProfile> for OpenFMBNATSPublisher {
    type Msg = OpenFMBNATSPublisherMsg;
    fn receive(
        &mut self,
        _ctx: &Context<Self::Msg>,
        msg: GenerationControlProfile,
        _sender: Sender,
    ) {
        let mut buffer = vec![];
        msg.encode(&mut buffer).unwrap();
        let mrid = msg.device_mrid().unwrap();
        let mut subject = String::default();
        subject.push_str("openfmb.generationmodule.GenerationControlProfile.");
        subject.push_str(&mrid.to_string());
        self.nats_broker.publish(&subject, &mut buffer).unwrap();

        //     info!("publishing switch control to {:?}", mrid);
        // info!("{:?}", msg);
        // info!("{:?}", subject);
    }
}

impl Receive<CoordinationStatus> for OpenFMBNATSPublisher {
    type Msg = OpenFMBNATSPublisherMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: CoordinationStatus, _sender: Sender) {
        let mut buffer = vec![];
        msg.encode(&mut buffer).unwrap();
        let mut subject = String::default();
        subject.push_str("coordination.status");
        self.nats_broker.publish(&subject, &mut buffer).unwrap();
    }
}
