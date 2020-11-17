use super::Error as OpenFMBError;
use crate::actors::openfmb_ts_to_timestamp;
use crate::actors::{Coordinator, CoordinatorMsg};
use crate::messages::{ActorStats, PartsMessage, RequestActorStats};
use openfmb_rs::openfmb;
use openfmb_rs::openfmb::commonmodule::{MessageInfo, StatusMessageInfo, Timestamp};
use openfmb_rs::openfmb::loadmodule::LoadStatusProfile;
use openfmb_rs::openfmb::solarmodule::SolarStatus;
use prost::Message;
use riker::actors::*;
#[actor(RequestActorStats)]
#[derive(Default, Debug)]
pub struct Load {
    message_count: u32,
}

impl Actor for Load {
    type Msg = LoadMsg;

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) { unimplemented!() }
}

impl Load {
    pub fn reading_profile_handler(msg: PartsMessage) -> Result<(), OpenFMBError> {
        let bytes = base64::decode(&msg.payload).unwrap();
        let r = openfmb::loadmodule::LoadReadingProfile::decode(bytes);
        match r {
            Ok(openfmb_message) => {
                let mrid: String;

                let rmi = openfmb_message.reading_message_info;
                // let ied = openfmb_message.ied;
                // let m = openfmb_message.meter;
                let _mr = openfmb_message.load_reading;

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

    pub fn status_profile_handler(msg: PartsMessage) -> Result<(), OpenFMBError> {
        let bytes = base64::decode(&msg.payload).unwrap();
        let openfmb_message: LoadStatusProfile = openfmb::loadmodule::LoadStatusProfile::decode(bytes).unwrap();
        let rmi: StatusMessageInfo = openfmb_message.status_message_info.unwrap();
        //           let rmi: StatusMessageInfo = openfmb_message.status_message_info.clone().unwrap();
        //dbg!(rmi);
        // let ied = openfmb_message.ied;
        // let m = openfmb_message.meter;
        //            let mr = openfmb_message.meter_reading;

        //            if rmi.is_some() {
        //                let rmi_val = rmi.unwrap();
        let mi: MessageInfo = rmi.message_info.unwrap();
        let io = mi.identified_object.unwrap();
        //                mrid = io.m_rid.unwrap();

        let ts = mi.message_time_stamp.as_ref().unwrap();

        // Print the timestamp info
        let mrid = io.m_rid.unwrap();
        info!("Load Status detected at: {} from {}", openfmb_ts_to_timestamp(ts), mrid);

        Ok(())
    }
}

impl Receive<RequestActorStats> for Load {
    type Msg = LoadMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: RequestActorStats, sender: Sender) {
        let stats_msg: CoordinatorMsg = ActorStats {
            message_count: self.message_count,
        }
        .into();
        sender.unwrap().try_tell(stats_msg, Some(ctx.myself.clone().into())).unwrap();
    }
}
