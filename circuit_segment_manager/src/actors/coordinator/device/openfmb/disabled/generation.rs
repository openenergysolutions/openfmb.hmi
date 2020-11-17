use super::super::super::CoordinatorMsg;
use super::Error as OpenFMBError;
use crate::actors::{openfmb_ts_to_timestamp, Processor};
use crate::messages::actor_stats::{ActorStats, RequestActorStats};
use crate::messages::PartsMessage;
use log::{info, warn};
use openfmb_rs::openfmb;
use openfmb_rs::openfmb::commonmodule::{IdentifiedObject, StatusMessageInfo};
use prost::Message as ProstMessage;
use riker::actors::*;
#[actor(RequestActorStats)]
#[derive(Default, Debug)]
pub struct Generator {
    message_count: u32,
    pub counter: usize,
}

impl Actor for Generator {
    type Msg = GeneratorMsg;

    fn pre_start(&mut self, ctx: &Context<Self::Msg>) { unimplemented!() }

    fn post_start(&mut self, ctx: &Context<Self::Msg>) { unimplemented!() }

    //    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) {
    //        self.receive(ctx, msg, sender)
    //    }

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) {
        info!("wtf");
        self.message_count += 1;
        self.receive(ctx, msg, sender);
    }
}

#[derive(Default, Debug, Clone)]
pub struct GeneratorStatus {}

impl Generator {
    pub fn reading_profile_handler(msg: PartsMessage) -> Result<(), OpenFMBError> {
        let bytes = base64::decode(&msg.payload).unwrap();
        let r = openfmb::generationmodule::GenerationReadingProfile::decode(bytes);
        //dbg!("what?");
        match r {
            Ok(openfmb_message) => {
                //            let foo:GenerationReadingProfile = openfmb_message;
                let mrid: String;

                let rmi = openfmb_message.reading_message_info.clone();
                // let ied = openfmb_message.ied;
                // let m = openfmb_message.meter;
                let _mr = openfmb_message.generation_reading.clone();

                if rmi.is_some() {
                    let rmi_val = rmi.unwrap();
                    let mi = rmi_val.message_info.unwrap();
                    let io = mi.identified_object.unwrap();
                    mrid = io.m_rid.unwrap();

                    let ts = mi.message_time_stamp.as_ref().unwrap();

                    info!(
                        "Generation Reading detected at: {} from {}",
                        openfmb_ts_to_timestamp(ts),
                        mrid
                    );
                }
                //  dbg!(openfmb_message);
            }
            Err(e) => {
                // Protobuf did not decode properly!
                info!("{}", e);
            }
        }
        Ok(())
    }

    pub fn status_profile_handler(msg: PartsMessage) -> Result<(), OpenFMBError> {
        let bytes = base64::decode(&msg.payload).unwrap();
        let r = openfmb::generationmodule::GenerationStatusProfile::decode(bytes);
        match r {
            Ok(openfmb_message) => {
                let mrid: String;

                let rmi_val = openfmb_message.clone();
                // let ied = openfmb_message.ied;
                // let m = openfmb_message.meter;
                let _mr = openfmb_message.status_message_info.clone();

                let mi = rmi_val.status_message_info.unwrap();
                let io = mi.message_info.unwrap();
                mrid = io.identified_object.unwrap().m_rid.unwrap();

                // let ts = mi.message_time_stamp.as_ref().unwrap();

                info!(
                    "Generation Status detected from {}",
                    //                    openfmb_ts_to_timestamp(ts),
                    mrid
                );

                //  dbg!(openfmb_message);
            }
            Err(e) => {
                // Protobuf did not decode properly!
                info!("{}", e);
            }
        }
        Ok(())
    }
}
pub struct Error {}

impl Receive<RequestActorStats> for Generator {
    type Msg = GeneratorMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: RequestActorStats, sender: Sender) {
        let stats_msg: CoordinatorMsg = ActorStats {
            message_count: self.message_count,
        }
        .into();
        sender.unwrap().try_tell(stats_msg, Some(ctx.myself.clone().into())).unwrap();
    }
}
