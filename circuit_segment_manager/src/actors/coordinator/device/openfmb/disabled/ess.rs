use crate::actors::coordinator::openfmb_ts_to_timestamp;
use crate::actors::{Coordinator, CoordinatorMsg};
use crate::messages::PartsMessage;
use crate::messages::*;
use openfmb_ops_protobuf::openfmb;
use openfmb_ops_protobuf::openfmb::commonmodule::Timestamp;
use openfmb_ops_protobuf::openfmb::solarmodule::SolarStatus;
use prost::Message;
use riker::actors::*;

#[actor(RequestActorStats)]
#[derive(Default, Clone, Debug)]
pub struct ESS {
    message_count: u32,
    mrid: String,
}

impl Actor for ESS {
    type Msg = ESSMsg;

    fn post_start(&mut self, ctx: &Context<Self::Msg>) { unimplemented!() }

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) { unimplemented!() }
}

impl ESS {
    pub fn status_profile_handler(msg: PartsMessage) -> Result<(), Error> {
        let bytes = base64::decode(&msg.payload).unwrap();
        let r = openfmb::essmodule::EssStatusProfile::decode(bytes);
        match r {
            Ok(openfmb_message) => {
                let mrid: String;

                let rmi = openfmb_message.status_message_info;
                // let ied = openfmb_message.ied;
                // let m = openfmb_message.meter;
                //            let sr: StatusMessageInfo = openfmb_message.status_message_info.unwrap();

                if rmi.is_some() {
                    let rmi_val = rmi.unwrap();
                    let mi = rmi_val.message_info.unwrap();
                    let io = mi.identified_object.unwrap();
                    mrid = io.m_rid.unwrap();

                    let ts = mi.message_time_stamp.as_ref().unwrap();

                    info!("ESS Status detected at: {} from {}", openfmb_ts_to_timestamp(ts), mrid);
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

pub enum Error {}

impl Receive<RequestActorStats> for ESS {
    type Msg = ESSMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: RequestActorStats, sender: Sender) {
        let stats_msg: CoordinatorMsg = ActorStats {
            message_count: self.message_count,
        }
        .into();
        sender.unwrap().try_tell(stats_msg, Some(ctx.myself.clone().into())).unwrap();
    }
}
