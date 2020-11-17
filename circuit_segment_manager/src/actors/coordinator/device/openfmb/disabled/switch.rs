use super::Error as OpenFMBError;
use crate::actors::openfmb_ts_to_timestamp;
use crate::actors::*;
use crate::messages::{ActorStats, PartsMessage, RequestActorStats};
use openfmb_ops_protobuf::openfmb;
use openfmb_ops_protobuf::openfmb::commonmodule::{IdentifiedObject, StatusMessageInfo, Timestamp};
use openfmb_ops_protobuf::openfmb::switchmodule::SwitchStatus;
use prost::Message as ProstMessage;
use riker::actors::*;
#[actor(RequestActorStats)]
#[derive(Default)]
pub struct Switch {
    pub message_count: u32,
}

impl Actor for Switch {
    type Msg = SwitchMsg;

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) { unimplemented!() }
    //    type Context = Context<Self>;
    //    fn started(&mut self, ctx: &mut Self::Context) {
    //        info!("Starting the Switch actor");
    //    }
}

//impl Handler<SwitchStatusMessage> for SwitchActor {
//    type Result = ();
//
//    fn handle(&mut self, msg: SwitchStatusMessage, ctx: &mut Self::Context) -> Self::Result {
//        info!("handling switch status message");
//    }
//}

#[derive(Debug, Clone)]
pub struct SwitchStatusMessage {}

impl Switch {
    pub fn status_profile_handler(msg: PartsMessage) -> Result<(), OpenFMBError> {
        let bytes = base64::decode(&msg.payload).unwrap();
        match openfmb::generationmodule::GenerationStatusProfile::decode(bytes) {
            Ok(openfmb_message) => {
                //            let foo:GenerationReadingProfile = openfmb_message;
                let mrid: String;

                let rmi: StatusMessageInfo = openfmb_message.status_message_info.clone().unwrap();
                // let ied = openfmb_message.ied;
                // let m = openfmb_message.meter;
                let _mr = openfmb_message.generation_status.clone();

                let mi = rmi.message_info.unwrap();
                let io: IdentifiedObject = mi.identified_object.unwrap();
                mrid = io.m_rid.unwrap();

                let ts = mi.message_time_stamp.as_ref().unwrap();

                // Print the timestamp info
                info!("Switch Status detected at: {} from {}", openfmb_ts_to_timestamp(ts), mrid);
            }
            Err(e) => {
                // Protobuf did not decode properly!
                info!("{}", e);
            }
        }
        Ok(())
    }

    pub fn reading_profile_handler(msg: PartsMessage) -> Result<(), OpenFMBError> {
        let bytes = base64::decode(&msg.payload).unwrap();
        let r = openfmb::switchmodule::SwitchReadingProfile::decode(bytes);
        match r {
            Ok(openfmb_message) => {
                let mrid: String;

                let rmi = openfmb_message.reading_message_info;
                // let ied = openfmb_message.ied;
                // let m = openfmb_message.meter;
                let _sr = openfmb_message.switch_reading;

                if rmi.is_some() {
                    let rmi_val = rmi.unwrap();
                    let mi = rmi_val.message_info.unwrap();
                    let io = mi.identified_object.unwrap();
                    mrid = io.m_rid.unwrap();

                    let ts: &Timestamp = mi.message_time_stamp.as_ref().unwrap();

                    // Print the timestamp info
                    info!("Switch Reading detected at: {} from {}", openfmb_ts_to_timestamp(ts), mrid);
                }
            }
            Err(e) => {
                info!("{}", e);
            }
        }
        Ok(())
    }
}
//    pub fn switch_status_profile_handler(&mut self, msg: PartsMessage) -> Result<(), subscribing::openfmb_file::Error> {
//        let bytes = base64::decode(&msg.payload).unwrap();
//        let r = openfmb::switchmodule::SwitchStatusProfile::decode(bytes);
//        match r {
//            Ok(openfmb_message) => {
//                let mrid: String;
//
//                let rmi = openfmb_message.status_message_info;
//                let _sr: SwitchStatus = openfmb_message.switch_status.unwrap();
//
//                if rmi.is_some() {
//                    let rmi_val = rmi.unwrap();
//                    let mi = rmi_val.message_info.unwrap();
//                    let io = mi.identified_object.unwrap();
//                    mrid = io.m_rid.unwrap();
//                    let ts = mi.message_time_stamp.as_ref().unwrap();
//
//                    self.counter+=1;
//                    info!(
//                        "Switch Status detected at: {} from {}. Switch event #{}",
//                        openfmb_ts_to_timestamp(ts),
//                        mrid,
//                        self.counter
//                    );
//                }
//            }
//            Err(e) => {
//                // Protobuf did not decode properly!
//                info!("{}", e);
//            }
//        }
//        Ok(())
//    }
//
//
//
//}
pub struct Error {}

impl Receive<RequestActorStats> for Switch {
    type Msg = SwitchMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: RequestActorStats, sender: Sender) {
        let stats_msg: CoordinatorMsg = ActorStats {
            message_count: self.message_count,
        }
        .into();
        sender.unwrap().try_tell(stats_msg, Some(ctx.myself.clone().into())).unwrap();
    }
}
