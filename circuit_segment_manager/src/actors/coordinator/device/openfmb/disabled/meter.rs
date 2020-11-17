use super::Error as OpenFMBError;
use crate::actors::openfmb_ts_to_timestamp;
use crate::actors::{Coordinator, CoordinatorMsg};
use crate::messages::{ActorStats, PartsMessage, RequestActorStats};
use openfmb_rs::openfmb;
use openfmb_rs::openfmb::commonmodule::{IdentifiedObject, StatusMessageInfo};
use prost::Message as ProstMessage;
use riker::actors::*;
#[actor(RequestActorStats)]
#[derive(Default)]
pub struct Meter {
    message_count: u32,
}

impl Actor for Meter {
    type Msg = MeterMsg;

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) { unimplemented!() }
    //    type Context = Context<Self>;
    //    fn started(&mut self, ctx: &mut Self::Context) {
    //        dbg!("Starting the Meter actor");
    //    }
    //    fn stopped(&mut self, ctx: &mut Context<Self>) {
    //        dbg!("Actor is stopped");
    //    }
}

impl Meter {
    pub fn reading_profile_handler(msg: PartsMessage) -> Result<(), OpenFMBError> {
        let bytes = base64::decode(&msg.payload).unwrap();
        let r = openfmb::metermodule::MeterReadingProfile::decode(bytes);
        match r {
            Ok(openfmb_message) => {
                let mrid: String;

                let rmi = openfmb_message.reading_message_info;
                // let ied = openfmb_message.ied;
                // let m = openfmb_message.meter;
                let _mr = openfmb_message.meter_reading;

                if rmi.is_some() {
                    let rmi_val = rmi.unwrap();
                    let mi = rmi_val.message_info.unwrap();
                    let io = mi.identified_object.unwrap();
                    mrid = io.m_rid.unwrap();

                    let ts = mi.message_time_stamp.as_ref().unwrap();

                    // Print the timestamp info
                    info!("Meter Reading detected at: {} from {}", openfmb_ts_to_timestamp(ts), mrid);
                }
            }
            Err(e) => {
                // Protobuf did not decode properly!
                info!("{}", e);
            }
        }
        Ok(())
    }
}

impl Receive<RequestActorStats> for Meter {
    type Msg = MeterMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: RequestActorStats, sender: Sender) {
        let stats_msg: CoordinatorMsg = ActorStats {
            message_count: self.message_count,
        }
        .into();
        sender.unwrap().try_tell(stats_msg, Some(ctx.myself.clone().into())).unwrap();
    }
}
