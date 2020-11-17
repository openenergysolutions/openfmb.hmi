use super::debug_stdout_publisher::{DebugStdoutPublisher, DebugStdoutPublisherMsg};
//use super::elasticsearch_publisher::{ElasticSearchPublisher, ElasticSearchPublisherMsg};
use super::{
    openfmb_file_publisher::{OpenFMBFilePublisher, OpenFMBFilePublisherMsg},
    openfmb_nats_publisher::{OpenFMBNATSPublisher, OpenFMBNATSPublisherMsg},
};
use crate::{
    actors::{
        coordinator::publisher::gui_publisher::GuiPublisher, gui_publisher::GuiPublisherMsg,
        CoordinatorMsg,
    },
    messages::*,
};

use microgrid_protobuf::CoordinationStatus;

use nats::Connection;
use openfmb_messages::{
    breakermodule::BreakerDiscreteControlProfile, essmodule::EssControlProfile,
    generationmodule::GenerationControlProfile, loadmodule::LoadControlProfile,
    solarmodule::SolarControlProfile, switchmodule::SwitchDiscreteControlProfile,
};
use riker::actors::*;
use std::fmt::Debug;

#[actor(
    RequestActorStats,
    OpenFMBMessage,
    LoadControlProfile,
    SwitchDiscreteControlProfile,
    EssControlProfile,
    SolarControlProfile,
    GenerationControlProfile,
    BreakerDiscreteControlProfile,
    CoordinationStatus
)]
#[derive(Clone, Debug)]
pub struct Publisher {
    pub message_count: u32,
    pub nats_client: nats::Connection,
    pub openfmb_file_publisher: Option<ActorRef<OpenFMBFilePublisherMsg>>,
    pub openfmb_nats_publisher: Option<ActorRef<OpenFMBNATSPublisherMsg>>,
    pub debug_stdout_publisher: Option<ActorRef<DebugStdoutPublisherMsg>>,
    pub gui_publisher: Option<ActorRef<GuiPublisherMsg>>,
    // pub elasticsearch_publisher: Option<ActorRef<ElasticSearchPublisherMsg>>,
}

impl ActorFactoryArgs<Connection> for Publisher {
    fn create_args(args: Connection) -> Self {
        Publisher {
            message_count: 0,
            nats_client: args,
            openfmb_file_publisher: None,
            openfmb_nats_publisher: None,
            debug_stdout_publisher: None,
            gui_publisher: None,
        }
    }
}

impl Publisher {
    pub fn get_child_actors(
        &mut self,
    ) -> (
        Option<ActorRef<OpenFMBFilePublisherMsg>>,
        Option<ActorRef<OpenFMBNATSPublisherMsg>>,
        Option<ActorRef<DebugStdoutPublisherMsg>>,
        // Option<ActorRef<ElasticSearchPublisherMsg>>,
        Option<ActorRef<GuiPublisherMsg>>,
    ) {
        (
            self.openfmb_file_publisher.clone(),
            self.openfmb_nats_publisher.clone(),
            self.debug_stdout_publisher.clone(),
            // self.elasticsearch_publisher.clone(),
            self.gui_publisher.clone(),
        )
    }
}
impl Actor for Publisher {
    type Msg = PublisherMsg;

    fn pre_start(&mut self, ctx: &Context<Self::Msg>) {
        self.openfmb_file_publisher = Some(
            ctx.actor_of::<OpenFMBFilePublisher>("OpenFMBFilePublisher")
                .unwrap(),
        );
        self.openfmb_nats_publisher = Some(
            ctx.actor_of_args::<OpenFMBNATSPublisher, Connection>(
                "OpenFMBNATSPublisher",
                self.nats_client.clone(),
            )
            .unwrap(),
        );
        self.debug_stdout_publisher = Some(
            ctx.actor_of::<DebugStdoutPublisher>("StdoutPublisher")
                .unwrap(),
        );
        // self.elasticsearch_publisher = Some(
        //     ctx.actor_of(Props::new(ElasticSearchPublisher::actor), "ElasticPublisher")
        //         .unwrap(),
        // );
        self.gui_publisher = Some(ctx.actor_of::<GuiPublisher>("GuiPublisher").unwrap());
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

impl Receive<RequestActorStats> for Publisher {
    type Msg = PublisherMsg;
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

        match &self.debug_stdout_publisher {
            Some(child) => {
                child.tell(msg.clone(), sender.clone());
            }
            None => todo!(),
        }
        match &self.openfmb_file_publisher {
            Some(child) => {
                child.tell(msg.clone(), sender.clone());
            }
            None => {}
        }
        match &self.openfmb_nats_publisher {
            Some(child) => {
                child.tell(msg.clone(), sender.clone());
            }
            None => {}
        }
        // match &self.elasticsearch_publisher {
        //     Some(child) => {
        //         child.tell(msg, sender);
        //     }
        //     None => {}
        // }
        //info!("Publisher has seen {} messages", self.message_count);
    }
}

impl Receive<GetChildActors> for Publisher {
    type Msg = PublisherMsg;

    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: GetChildActors, _sender: Sender) {
        self.message_count += 1;
        // sender.unwrap().try_tell(self.debug_stdout_publisher.clone(), None);
    }
}

// impl Receive<TextLine> for Publisher {
//     type Msg = PublisherMsg;
//
//     fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: TextLine, sender: Sender) {
//         self.debug_stdout_publisher.clone().unwrap().tell(msg.clone(), sender.clone());
//         self.elasticsearch_publisher.clone().unwrap().tell(msg, sender);
//         //        println!("{}", msg.line);
//     }
// }

//impl Stats<PublisherMsg> for Publisher{
//    fn message_count(&self) -> u32 {
//        self.message_count
//    }
//    fn receive(&mut self, ctx: &Context<PublisherMsg>, _msg: RequestActorStats, _sender: Sender) {
//        info!("Publisher has seen {} messages", self.message_count);
//    }
//}

impl Receive<OpenFMBMessage> for Publisher {
    type Msg = PublisherMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: OpenFMBMessage, _sender: Sender) {
        // self.elasticsearch_publisher
        //     .as_ref()
        //     .unwrap()
        //     .send_msg(msg.into(), ctx.myself.clone())
    }
}

impl Receive<LoadControlProfile> for Publisher {
    type Msg = PublisherMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: LoadControlProfile, _sender: Sender) {
        // dbg!("got load control message: {:?}", msg.clone());
        self.openfmb_nats_publisher
            .as_ref()
            .unwrap()
            .send_msg(msg.into(), None);
    }
}
impl Receive<EssControlProfile> for Publisher {
    type Msg = PublisherMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: EssControlProfile, _sender: Sender) {
        // dbg!("got load control message: {:?}", msg.clone());
        self.openfmb_nats_publisher
            .as_ref()
            .unwrap()
            .send_msg(msg.into(), None);
    }
}

impl Receive<GenerationControlProfile> for Publisher {
    type Msg = PublisherMsg;
    fn receive(
        &mut self,
        _ctx: &Context<Self::Msg>,
        msg: GenerationControlProfile,
        _sender: Sender,
    ) {
        // dbg!("got load control message: {:?}", msg.clone());
        self.openfmb_nats_publisher
            .as_ref()
            .unwrap()
            .send_msg(msg.into(), None);
    }
}

impl Receive<SwitchDiscreteControlProfile> for Publisher {
    type Msg = PublisherMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: SwitchDiscreteControlProfile, _sender: Sender) {
        // dbg!("got load control message: {:?}", msg.clone());
        self.openfmb_nats_publisher
            .as_ref()
            .unwrap()
            .send_msg(msg.into(), None);
    }
}

impl Receive<BreakerDiscreteControlProfile> for Publisher {
    type Msg = PublisherMsg;
    fn receive(
        &mut self,
        _ctx: &Context<Self::Msg>,
        msg: BreakerDiscreteControlProfile,
        _sender: Sender,
    ) {
        // dbg!("got load control message: {:?}", msg.clone());
        self.openfmb_nats_publisher
            .as_ref()
            .unwrap()
            .send_msg(msg.into(), None);
    }
}

impl Receive<SolarControlProfile> for Publisher {
    type Msg = PublisherMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: SolarControlProfile, _sender: Sender) {
        // dbg!("got load control message: {:?}", msg.clone());
        self.openfmb_nats_publisher
            .as_ref()
            .unwrap()
            .send_msg(msg.into(), None);
    }
}

impl Receive<CoordinationStatus> for Publisher {
    type Msg = PublisherMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: CoordinationStatus, _sender: Sender) {
        // dbg!("got load control message: {:?}", msg.clone());
        self.openfmb_nats_publisher
            .as_ref()
            .unwrap()
            .send_msg(msg.into(), None);
    }
}
