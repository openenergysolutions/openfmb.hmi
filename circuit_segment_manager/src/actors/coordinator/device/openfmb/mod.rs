pub mod openfmb;
pub use openfmb::*;

//mod ess;
//mod generation;
//mod load;
//mod meter;
//mod solar;
//mod switch;
//pub mod generic_openfmb_profile;
//pub use generic_openfmb_profile::*;
//
//pub mod openfmb_device_profile;
//pub use openfmb_device_profile::*;

//use super::super::super::super::messages::RequestActorStats;
//use super::super::super::CoordinatorMsg;
//use crate::{
//    actors::coordinator::{openfmb_file_subscriber::OpenFMBFileSubscriber, openfmb_ts_to_timestamp},
//    messages::{PartsMessage, StartProcessing},
//};
//use log::{info, warn};
//use prost::Message;
//use riker::actors::*;
//use openfmb_rs::{
//    openfmb,
//    openfmb::{commonmodule::Timestamp, solarmodule::SolarStatus},
//};
//
////use crate::actors::coordinator::device::openfmb::ess::ESS;
////use crate::actors::coordinator::device::openfmb::generation::*;
////use crate::actors::coordinator::device::openfmb::meter::MeterMsg;
////use crate::actors::coordinator::device::openfmb::solar::SolarMsg;
//use crate::actors::coordinator::subscriber::openfmb_file_subscriber::OpenFMBFileSubscriberMsg;
//use crate::actors::coordinator::subscriber::openfmb_file_subscriber::OpenFMBProfileType;
//use crate::actors::coordinator::subscriber::openfmb_file_subscriber::OpenFMBProfileType::Generation;
//use crate::messages::*;
////use load::{Load, LoadMsg};
//use std::collections::HashMap;
////use switch::{Switch, SwitchMsg};
//
//
//
//pub enum Error {}
