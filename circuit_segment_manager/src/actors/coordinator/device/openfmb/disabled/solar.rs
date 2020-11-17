use super::Error as OpenFMBError;
use crate::actors::coordinator::processor::openfmb::switch::{Switch, SwitchMsg};
use crate::actors::openfmb_ts_to_timestamp;
use crate::actors::{Coordinator, CoordinatorMsg};
use crate::messages::*;
use openfmb_ops_protobuf::openfmb;
use openfmb_ops_protobuf::openfmb::commonmodule::Timestamp;
use openfmb_ops_protobuf::openfmb::solarmodule::SolarStatus;
use prost::Message;
use riker::actors::*;

#[actor(RequestActorStats)]
#[derive(Default)]
pub struct Solar {
    message_count: u32,
}

impl Actor for Solar {
    type Msg = SolarMsg;

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) { unimplemented!() }
    //    type Context = Context<Self>;
    //    fn started(&mut self, ctx: &mut Self::Context) {
    //        info!("Starting the Solar actor");
    //    }
}

impl Solar {
    pub fn reading_profile_handler(msg: PartsMessage) -> Result<(), OpenFMBError> {
        let bytes = base64::decode(&msg.payload).unwrap();
        let r = openfmb::solarmodule::SolarReadingProfile::decode(bytes);
        match r {
            Ok(openfmb_message) => {
                let mrid: String;

                let rmi = openfmb_message.reading_message_info;
                // let ied = openfmb_message.ied;
                // let m = openfmb_message.meter;
                let _sr = openfmb_message.solar_reading.unwrap();

                if rmi.is_some() {
                    let rmi_val = rmi.unwrap();
                    let mi = rmi_val.message_info.unwrap();
                    let io = mi.identified_object.unwrap();
                    mrid = io.m_rid.unwrap();

                    let ts: &Timestamp = mi.message_time_stamp.as_ref().unwrap();

                    // Print the timestamp info
                    info!("Solar Reading detected at: {} from {}", openfmb_ts_to_timestamp(ts), mrid);
                }
            }
            Err(e) => {
                info!("{}", e);
            }
        }
        Ok(())
    }
    pub fn status_profile_handler(msg: PartsMessage) -> Result<(), OpenFMBError> {
        let bytes = base64::decode(&msg.payload).unwrap();
        let r = openfmb::solarmodule::SolarStatusProfile::decode(bytes);
        match r {
            Ok(openfmb_message) => {
                let mrid: String;

                let rmi = openfmb_message.status_message_info;
                // let ied = openfmb_message.ied;
                // let m = openfmb_message.meter;
                let _sr: SolarStatus = openfmb_message.solar_status.unwrap();

                if rmi.is_some() {
                    let rmi_val = rmi.unwrap();
                    let mi = rmi_val.message_info.unwrap();
                    let io = mi.identified_object.unwrap();
                    mrid = io.m_rid.unwrap();

                    let ts = mi.message_time_stamp.as_ref().unwrap();

                    info!("Solar Status detected at: {} from {}", openfmb_ts_to_timestamp(ts), mrid);
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

impl Receive<RequestActorStats> for Solar {
    type Msg = SolarMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: RequestActorStats, sender: Sender) {
        let stats_msg: CoordinatorMsg = ActorStats {
            message_count: self.message_count,
        }
        .into();
        sender.unwrap().try_tell(stats_msg, Some(ctx.myself.clone().into())).unwrap();
    }
}
