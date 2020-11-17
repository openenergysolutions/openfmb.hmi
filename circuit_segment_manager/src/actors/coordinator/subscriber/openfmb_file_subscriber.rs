// use super::super::{CoordinatorMsg, Device, DeviceMsg, Publisher, PublisherMsg};
// use crate::{
//     actors::coordinator::subscriber::generic_openfmb_profile_subscriber::{
//         GenericOpenFMBProfileSubscriber, GenericOpenFMBProfileSubscriberMsg,
//     },
//     messages::*,
// };
//
// use log::info;
// use riker::actors::*;
// use snafu::{OptionExt, ResultExt, Snafu};
// use std::{
//     any::Any,
//     collections::HashMap,
//     fs::File,
//     io::{BufRead, BufReader},
//     thread,
//     time::{Duration, Instant},
// };
//
// // use crate::actors::coordinator::publisher::elasticsearch_publisher::{
// //     ElasticSearchPublisherMsg,
// // };
//
// use std::convert::TryInto;
// use crate::actors::coordinator::subscriber::generic_openfmb_device_subscriber::GenericOpenFMBDevicePersistorSubscriber;
//
// #[actor(StartProcessing, RequestActorStats, OpenFMBMessage)]
// #[derive(Debug)]
// pub struct OpenFMBFileSubscriber {
//     pub message_count: u32,
//     openfmb_profile_actors:
//         HashMap<OpenFMBProfileType, ActorRef<GenericOpenFMBProfileSubscriberMsg>>,
//     publisher: ActorRef<PublisherMsg>,
//     processor: ActorRef<DeviceMsg>,
// }
//
// impl OpenFMBFileSubscriber {
//     fn open_file(filename: String) -> Result<BufReader<File>, Error> {
//         let file = File::open(filename).context(IOError)?;
//         Ok(BufReader::new(file))
//     }
//
//     fn process_file_lines(
//         &mut self,
//         ctx: &Context<<OpenFMBFileSubscriber as Actor>::Msg>,
//     ) -> Result<(), Error> {
//         //        let reader = OpenFMBFileSubscriber::open_file("config/one_packet_duplicated.txt".to_string()).unwrap();
//         //let reader = OpenFMBFileSubscriber::open_file("datafiles/capture_10.txt".to_string()).unwrap();
//         //let reader = OpenFMBFileSubscriber::open_file("datafiles/100_packets.txt".to_string()).unwrap();
//         //let reader = OpenFMBFileSubscriber::open_file("datafiles/capture-hil-20200206.txt".to_string()).unwrap();
//         let reader =
//             OpenFMBFileSubscriber::open_file("datafiles/small_new.txt".to_string()).unwrap();
//         // let reader = OpenFMBFileSubscriber::open_file("datafiles/capture-10072019.txt".to_string()).unwrap();
//         //        let reader = OpenFMBFileSubscriber::open_file("datafiles/big_capture.txt".to_string()).unwrap();
//         info!("processing file");
//         for line in reader.lines() {
//             let line: String = line.unwrap();
//             let parts_vec: Vec<&str> = line.split(",").collect();
//             let (_number, profile, payload) = (
//                 parts_vec.get(0).unwrap().to_string(),
//                 parts_vec.get(1).unwrap().to_string(),
//                 parts_vec.get(2).unwrap().to_string(),
//             );
//             //Convert a tuple of a profile and base64 encoded openfmb message to it's native openfmb form
//             let openfmb_message: Result<OpenFMBMessage, _> =
//                 (profile.as_str(), payload.as_str()).try_into();
//
//             match openfmb_message {
//                 Ok(msg) => {
//                     ctx.myself.send_msg(msg.clone().into(), ctx.myself.clone());
//                 }
//                 Err(err) => {
//                     panic!("Error processing a line {:?}", err);
//                 }
//             };
//         }
//         Ok(())
//     }
//
//     fn ensure_actor(
//         &mut self,
//         ctx: &Context<OpenFMBFileSubscriberMsg>,
//         msg: &OpenFMBMessage,
//     ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
//         use OpenFMBMessage::*;
//         match msg {
//             GenerationReading(_msg) => self.ensure_generator_actor(ctx),
//             GenerationStatus(_msg) => self.ensure_generator_actor(ctx),
//             SwitchStatus(_msg) => self.ensure_switch_actor(ctx),
//             SwitchReading(_msg) => self.ensure_switch_actor(ctx),
//             MeterReading(_msg) => self.ensure_meter_actor(ctx),
//             SolarReading(_msg) => self.ensure_solar_actor(ctx),
//             SolarStatus(_msg) => self.ensure_solar_actor(ctx),
//             ESSReading(_msg) => self.ensure_ess_actor(ctx),
//             ESSStatus(_msg) => self.ensure_ess_actor(ctx),
//             LoadStatus(_msg) => self.ensure_load_actor(ctx),
//             LoadReading(_msg) => self.ensure_load_actor(ctx),
//             ShuntStatus(_msg) => self.ensure_shunt_actor(ctx),
//             ShuntReading(_msg) => self.ensure_shunt_actor(ctx),
//             RecloserStatus(_msg) => self.ensure_recloser_actor(ctx),
//             RecloserReading(_msg) => self.ensure_recloser_actor(ctx),
//             BreakerStatus(_msg) => self.ensure_breaker_actor(ctx),
//             BreakerReading(_msg) => self.ensure_breaker_actor(ctx),
//             RegulatorStatus(_msg) => self.ensure_regulator_actor(ctx),
//             RegulatorReading(_msg) => self.ensure_regulator_actor(ctx),
//             LoadControl(_msg) => self.ensure_load_actor(ctx),
//             SwitchControl(_msg) => self.ensure_switch_actor(ctx),
//             EssControl(_msg) => self.ensure_ess_actor(ctx),
//             SolarControl(_msg) => self.ensure_ess_actor(ctx),
//             GeneratorControl(_msg) => self.ensure_generator_actor(ctx),
//             BreakerControl(_msg) => self.ensure_generator_actor(ctx),
//             ResourceStatus(_msg) => self.ensure_resource_actor(ctx),
//         }
//     }
//
//     fn ensure_breaker_actor(
//         &mut self,
//         ctx: &Context<OpenFMBFileSubscriberMsg>,
//     ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
//         match self
//             .openfmb_profile_actors
//             .get(&OpenFMBProfileType::Breaker)
//         {
//             Some(&actor_ref) => actor_ref.clone(),
//             None => {
//                 let generation_actor_ref = ctx
//                     .actor_of_args::<OpenFMBFileSubscriber,ActorRef<GenericOpenFMBProfileSubscriber>>(
//                         "BreakerHandler",
//                             self.processor.clone(),
//                     )
//                     .unwrap();
//                 self.openfmb_profile_actors
//                     .insert(OpenFMBProfileType::Breaker, generation_actor_ref.clone());
//                 generation_actor_ref
//             }
//         }
//     }
//
//     fn ensure_regulator_actor(
//         &mut self,
//         ctx: &Context<OpenFMBFileSubscriberMsg>,
//     ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
//         match self
//             .openfmb_profile_actors
//             .get(&OpenFMBProfileType::Regulator)
//         {
//             Some(actor_ref) => actor_ref.clone(),
//             None => {
//                 let generation_actor_ref = ctx
//                     .actor_of(
//                         Props::new_args(
//                             GenericOpenFMBProfileSubscriber::actor,
//                             self.processor.clone(),
//                         ),
//                         "RegulatorHandler",
//                     )
//                     .unwrap();
//                 self.openfmb_profile_actors
//                     .insert(OpenFMBProfileType::Regulator, generation_actor_ref.clone());
//                 generation_actor_ref
//             }
//         }
//     }
//
//     fn ensure_load_actor(
//         &mut self,
//         ctx: &Context<OpenFMBFileSubscriberMsg>,
//     ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
//         match self.openfmb_profile_actors.get(&OpenFMBProfileType::Load) {
//             Some(actor_ref) => actor_ref.clone(),
//             None => {
//                 let generation_actor_ref = ctx
//                     .actor_of(
//                         Props::new_args(
//                             GenericOpenFMBProfileSubscriber::actor,
//                             self.processor.clone(),
//                         ),
//                         "LoadHandler",
//                     )
//                     .unwrap();
//                 self.openfmb_profile_actors
//                     .insert(OpenFMBProfileType::Load, generation_actor_ref.clone());
//                 generation_actor_ref
//             }
//         }
//     }
//
//     fn ensure_recloser_actor(
//         &mut self,
//         ctx: &Context<OpenFMBFileSubscriberMsg>,
//     ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
//         match self
//             .openfmb_profile_actors
//             .get(&OpenFMBProfileType::Recloser)
//         {
//             Some(actor_ref) => actor_ref.clone(),
//             None => {
//                 let generation_actor_ref = ctx
//                     .actor_of(
//                         Props::new_args(
//                             GenericOpenFMBProfileSubscriber::actor,
//                             self.processor.clone(),
//                         ),
//                         "RecloserHandler",
//                     )
//                     .unwrap();
//                 self.openfmb_profile_actors
//                     .insert(OpenFMBProfileType::Recloser, generation_actor_ref.clone());
//                 generation_actor_ref
//             }
//         }
//     }
//
//     fn ensure_shunt_actor(
//         &mut self,
//         ctx: &Context<OpenFMBFileSubscriberMsg>,
//     ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
//         match self.openfmb_profile_actors.get(&OpenFMBProfileType::Shunt) {
//             Some(actor_ref) => actor_ref.clone(),
//             None => {
//                 let generation_actor_ref = ctx
//                     .actor_of(
//                         Props::new_args(
//                             GenericOpenFMBProfileSubscriber::actor,
//                             self.processor.clone(),
//                         ),
//                         "ShuntHandler",
//                     )
//                     .unwrap();
//                 self.openfmb_profile_actors
//                     .insert(OpenFMBProfileType::Shunt, generation_actor_ref.clone());
//                 generation_actor_ref
//             }
//         }
//     }
//
//     fn ensure_ess_actor(
//         &mut self,
//         ctx: &Context<OpenFMBFileSubscriberMsg>,
//     ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
//         match self.openfmb_profile_actors.get(&OpenFMBProfileType::ESS) {
//             Some(actor_ref) => actor_ref.clone(),
//             None => {
//                 let generation_actor_ref = ctx
//                     .actor_of(
//                         Props::new_args(
//                             GenericOpenFMBProfileSubscriber::actor,
//                             self.processor.clone(),
//                         ),
//                         "ESSHandler",
//                     )
//                     .unwrap();
//                 self.openfmb_profile_actors
//                     .insert(OpenFMBProfileType::ESS, generation_actor_ref.clone());
//                 generation_actor_ref
//             }
//         }
//     }
//
//     fn ensure_solar_actor(
//         &mut self,
//         ctx: &Context<OpenFMBFileSubscriberMsg>,
//     ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
//         match self.openfmb_profile_actors.get(&OpenFMBProfileType::Solar) {
//             Some(actor_ref) => actor_ref.clone(),
//             None => {
//                 let generation_actor_ref = ctx
//                     .actor_of(
//                         Props::new_args(
//                             GenericOpenFMBProfileSubscriber::actor,
//                             self.processor.clone(),
//                         ),
//                         "SolarHandler",
//                     )
//                     .unwrap();
//                 self.openfmb_profile_actors
//                     .insert(OpenFMBProfileType::Solar, generation_actor_ref.clone());
//                 generation_actor_ref
//             }
//         }
//     }
//
//     fn ensure_meter_actor(
//         &mut self,
//         ctx: &Context<OpenFMBFileSubscriberMsg>,
//     ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
//         match self.openfmb_profile_actors.get(&OpenFMBProfileType::Meter) {
//             Some(actor_ref) => actor_ref.clone(),
//             None => {
//                 let generation_actor_ref = ctx
//                     .actor_of(
//                         Props::new_args(
//                             GenericOpenFMBProfileSubscriber::actor,
//                             self.processor.clone(),
//                         ),
//                         "MeterhHandler",
//                     )
//                     .unwrap();
//                 self.openfmb_profile_actors
//                     .insert(OpenFMBProfileType::Meter, generation_actor_ref.clone());
//                 generation_actor_ref
//             }
//         }
//     }
//
//     fn ensure_switch_actor(
//         &mut self,
//         ctx: &Context<OpenFMBFileSubscriberMsg>,
//     ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
//         match self.openfmb_profile_actors.get(&OpenFMBProfileType::Switch) {
//             Some(actor_ref) => actor_ref.clone(),
//             None => {
//                 let generation_actor_ref = ctx
//                     .actor_of(
//                         Props::new_args(
//                             GenericOpenFMBProfileSubscriber::actor,
//                             self.processor.clone(),
//                         ),
//                         "SwitchHandler",
//                     )
//                     .unwrap();
//                 self.openfmb_profile_actors
//                     .insert(OpenFMBProfileType::Switch, generation_actor_ref.clone());
//                 generation_actor_ref
//             }
//         }
//     }
//
//     fn ensure_generator_actor(
//         &mut self,
//         ctx: &Context<OpenFMBFileSubscriberMsg>,
//     ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
//         match self
//             .openfmb_profile_actors
//             .get(&OpenFMBProfileType::Generation)
//         {
//             Some(actor_ref) => actor_ref.clone(),
//             None => {
//                 let generation_actor_ref = ctx
//                     .actor_of(
//                         Props::new_args(
//                             GenericOpenFMBProfileSubscriber::actor,
//                             self.processor.clone(),
//                         ),
//                         "GenerationHandler",
//                     )
//                     .unwrap();
//                 self.openfmb_profile_actors
//                     .insert(OpenFMBProfileType::Generation, generation_actor_ref.clone());
//                 generation_actor_ref
//             }
//         }
//     }
//
//     fn ensure_resource_actor(
//         &mut self,
//         _ctx: &Context<OpenFMBFileSubscriberMsg>,
//     ) -> ActorRef<GenericOpenFMBProfileSubscriberMsg> {
//         unimplemented!()
//         // match self.openfmb_profile_actors.get(&OpenFMBProfileType::Generation) {
//         //     Some(actor_ref) => actor_ref.clone(),
//         //     None => {
//         //         let generation_actor_ref = ctx
//         //             .actor_of(
//         //                 Props::new_args(GenericOpenFMBProfileSubscriber::actor, self.processor.clone()),
//         //                 "GenerationHandler",
//         //             )
//         //             .unwrap();
//         //         self.openfmb_profile_actors
//         //             .insert(OpenFMBProfileType::Generation, generation_actor_ref.clone());
//         //         generation_actor_ref
//         //     }
//         //}
//     }
// }
//
// impl Actor for OpenFMBFileSubscriber {
//     type Msg = OpenFMBFileSubscriberMsg;
//
//     fn pre_start(&mut self, _ctx: &Context<<OpenFMBFileSubscriber as Actor>::Msg>) {}
//
//     fn post_start(&mut self, _ctx: &Context<Self::Msg>) {}
//
//     fn post_stop(&mut self) {}
//
//     fn supervisor_strategy(&self) -> Strategy {
//         Strategy::Restart
//     }
//
//     fn sys_recv(
//         &mut self,
//         _ctx: &Context<Self::Msg>,
//         _msg: SystemMsg,
//         _sender: Option<BasicActorRef>,
//     ) {
//     }
//
//     fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) {
//         self.message_count += 1;
//         self.receive(ctx, msg, sender);
//     }
// }
//
// impl Receive<StartProcessing> for OpenFMBFileSubscriber {
//     type Msg = OpenFMBFileSubscriberMsg;
//
//     fn receive(&mut self, ctx: &Context<Self::Msg>, _msg: StartProcessing, _sender: Sender) {
//         //   info!("openfmb subscriber received msg {:?}", msg.clone());
//         self.process_file_lines(ctx).unwrap();
//     }
// }
//
// #[derive(Debug, Snafu)]
// pub enum Error {
//     #[snafu(display("Actor System Error"))]
//     IOError { source: std::io::Error },
//     #[snafu(display("Unsupported OpenFMB type"))]
//     UnsupportedOpenFMBTypeError { fmb_type: String },
// }
//
// impl Receive<RequestActorStats> for OpenFMBFileSubscriber {
//     type Msg = OpenFMBFileSubscriberMsg;
//     fn receive(&mut self, ctx: &Context<Self::Msg>, msg: RequestActorStats, sender: Sender) {
//         let stats_msg: CoordinatorMsg = ActorStats {
//             message_count: self.message_count,
//             persisted_message_count: None,
//         }
//         .into();
//         sender
//             .clone()
//             .unwrap()
//             .try_tell(stats_msg, Some(ctx.myself.clone().into()))
//             .unwrap();
//         for child in self.openfmb_profile_actors.clone() {
//             child.1.send_msg(msg.clone().into(), sender.clone());
//         }
//         let _stats_msg: CoordinatorMsg = ActorStats {
//             message_count: self.message_count,
//             persisted_message_count: None,
//         }
//         .into();
//     }
// }
//
// impl Receive<OpenFMBMessage> for OpenFMBFileSubscriber {
//     type Msg = OpenFMBFileSubscriberMsg;
//     fn receive(&mut self, ctx: &Context<Self::Msg>, msg: OpenFMBMessage, _sender: Sender) {
//         let actor = self.ensure_actor(ctx, &msg);
//         actor.send_msg(msg.clone().into(), ctx.myself.clone());
//
//         // let elastic_publisher = ctx.system.select("/user/Publisher/ElasticPublisher").unwrap();
//         // let msg: ElasticSearchPublisherMsg = msg.into();
//         // elastic_publisher.try_tell(msg, None);
//     }
// }
