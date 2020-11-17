use riker::actors::*;

mod circuit_segment;
pub use circuit_segment::*;
//pub trait Stats<T:'static+Debug+Clone+Send>: Debug{
//    fn message_count(&self) -> u32;
//    fn receive(&mut self, ctx: &Context<T>, _msg: RequestActorStats, _sender: Sender);
//
//}

//impl<T:'static+Debug+Clone+Send> Receive<RequestActorStats> for Stats<T> {
//    type Msg = T;
//
////    fn receive(&mut self, ctx: &Context<Self::Msg>, _msg: RequestActorStats, _sender: Sender) {
////        warn!("receive of CollectActorStats unimplemented");
////        info!("Subscriber has seen {} messages", self.message_count());
////    }
//
//}

//let bytes = base64::decode(&msg.payload).unwrap();
//let r = openfmb::solarmodule::SolarStatusProfile::decode(bytes);
//match r {
//Ok(openfmb_message) => {
//let mrid: String;
//
//let rmi = openfmb_message.status_message_info;
//// let ied = openfmb_message.ied;
//// let m = openfmb_message.meter;
//let _sr: SolarStatus = openfmb_message.solar_status.unwrap();
//
//if rmi.is_some() {
//let rmi_val = rmi.unwrap();
//let mi = rmi_val.message_info.unwrap();
//let io = mi.identified_object.unwrap();
//mrid = io.m_rid.unwrap();
//
//let ts = mi.message_time_stamp.as_ref().unwrap();
//
//info!(
//"Solar Status detected at: {} from {}",
//openfmb_ts_to_timestamp(ts),
//mrid

//pub enum OpenFMBMessage {
//    GenerationReadingProfile(&'static GenerationReadingProfile),
//    SwitchReadingProfile(&'static SwitchReadingProfile),
//    SwitchStatusProfile(&'static SwitchStatusProfile),
//    MeterReadingProfile(&'static MeterReadingProfile),
//    SolarReadingProfile(&'static SolarReadingProfile),
//    SolarStatusProfile(&'static SolarStatusProfile),
//    ESSReadingProfile(&'static EssReadingProfile),
//    ESSStatusProfile(&'static EssStatusProfile),
//    LoadReadingProfile(&'static LoadReadingProfile),
//    LoadStatusProfile(&'static LoadStatusProfile),
//}

pub trait PersistorActor {}

pub trait CircuitSegmentActor: Actor {
    fn get_message_count(&self) -> u32;
}
