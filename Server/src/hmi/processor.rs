// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::handler::*;
use crate::set_ess_csg;
use crate::set_generation_csg;
use crate::set_solar_csg;
use crate::Publisher;
use log::warn;
use openfmb::prelude::Message;
use openfmb_messages_ext::OpenFMBMessage;

use openfmb::messages::commonmodule::ScheduleParameterKind;

use openfmb::messages::breakermodule::BreakerDiscreteControlProfile;
use openfmb_messages_ext::breaker::BreakerControlExt;

use openfmb::messages::capbankmodule::CapBankControlProfile;
use openfmb_messages_ext::capbank::CapBankControlExt;

use openfmb::messages::reclosermodule::RecloserDiscreteControlProfile;
use openfmb_messages_ext::recloser::RecloserControlExt;

use openfmb::messages::switchmodule::SwitchDiscreteControlProfile;
use openfmb_messages_ext::switch::SwitchControlExt;

use openfmb::messages::essmodule::EssControlProfile;
use openfmb_messages_ext::ess::EssControlExt;

use openfmb::messages::loadmodule::LoadControlProfile;
use openfmb_messages_ext::load::LoadControlExt;

use openfmb::messages::generationmodule::GenerationControlProfile;
use openfmb_messages_ext::generation::GenerationControlExt;

use openfmb::messages::solarmodule::SolarControlProfile;
use openfmb_messages_ext::solar::SolarControlExt;

use openfmb::messages::resourcemodule::ResourceDiscreteControlProfile;
use openfmb_messages_ext::resource::ResourceControlExt;

use openfmb::messages::regulatormodule::{
    RegulatorControlProfile, RegulatorDiscreteControlProfile,
};
use openfmb_messages_ext::regulator::{RegulatorControlExt, RegulatorDiscreteControlExt};

use serde_json::Value;
use std::collections::btree_map::BTreeMap;
use std::time::SystemTime;

pub struct Node {
    name: String,
    path: String,
    value: String,
    nodes: Vec<Node>,
}

impl Node {
    pub fn new(name: &str) -> Node {
        Node {
            nodes: vec![],
            name: String::from(name),
            path: String::from(""),
            value: String::from(""),
        }
    }
    pub fn path(&self) -> String {
        self.path.clone()
    }
    pub fn get_value(&self) -> String {
        self.value.clone()
    }
    pub fn set_value(&mut self, value: &str) {
        self.value = String::from(value);
    }
    fn add_node(&mut self, node: Node) {
        let mut n = node;
        if n.path.is_empty() {
            n.path = format!("{}.{}", self.path, n.name);
        }
        self.nodes.push(n);
    }
    pub fn from_json(&mut self, json_value: &Value, data: &mut BTreeMap<String, DataValue>) {
        match json_value {
            Value::Object(m) => {
                for (k, v) in m.iter() {
                    let mut child = Node::new(k);
                    child.path = format!("{}.{}", self.path, k);
                    child.from_json(v, data);
                    self.add_node(child);
                }
            }
            Value::Array(a) => {
                let original_path = self.path();
                for (i, v) in a.iter().enumerate() {
                    self.path = format!("{}[{}]", original_path, i);
                    self.from_json(v, data);
                }
            }
            Value::Bool(b) => {
                self.set_value(b.to_string().as_str());
                data.insert(
                    self.path().replace("_", "").to_lowercase(),
                    DataValue::Bool(*b),
                );
            }
            Value::Number(n) => {
                self.set_value(n.to_string().as_str());
                data.insert(
                    self.path().replace("_", "").to_lowercase(),
                    DataValue::Double(n.as_f64().unwrap()),
                );
            }
            Value::String(s) => {
                self.set_value(s.as_str());
                data.insert(
                    self.path().replace("_", "").to_lowercase(),
                    DataValue::String(s.clone()),
                );
            }
            Value::Null => {
                // ignore
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Processor {
    publisher: Publisher,
    clients: Clients,
}

impl Processor {
    pub fn new(publisher: Publisher, clients: Clients) -> Self {
        Processor { publisher, clients }
    }

    pub async fn receive_openfmb_message(&self, msg: OpenFMBMessage) {
        handle_openfmb_message(&self.clients, msg).await;
    }

    pub async fn receive_microgrid_control_message(&self, msg: MicrogridControl) {
        log::debug!("Received microgrid control message {:?}", msg);

        let subject = "openfmb.microgridui.microgrid_control";
        log::info!("Sending {:?} to NATS topic {}", msg, subject);
        let mut buffer = Vec::<u8>::new();
        msg.message.encode(&mut buffer);
        self.publisher
            .publish(subject.to_string(), &mut buffer)
            .await;
    }

    pub async fn receive_device_control(&self, msg: DeviceControl) {
        log::debug!("Received device control message {:?}", msg);

        let subject = "openfmb.microgridui.device_control";
        log::info!("Sending {:?} to NATS topic {}", msg, subject);
        let mut buffer = Vec::<u8>::new();
        let device_control_msg = microgrid_protobuf::DeviceControl {
            mrid: msg.text,
            msg: msg.message.into(),
        };
        device_control_msg.encode(&mut buffer).unwrap();
        self.publisher
            .publish(subject.to_string(), &mut buffer)
            .await;
    }

    pub async fn receive_generic_control_message(&self, msg: GenericControl) {
        log::debug!("Received generic control message {:?}", msg);
        handle_generic_control(self, msg).await;
    }
}

macro_rules! extract {
    ($data_maps:expr, $message:expr, $profile_name:expr, $topic:expr) => {
        match $data_maps.get(&$topic.mrid) {
            Some(d) => d,
            _ => {
                let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                if let Ok(my_msg_json) = serde_json::to_string($message) {
                    let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                    let mut root = Node::new("mapping");
                    root.path = format!("{}.mapping", $profile_name);
                    root.from_json(&json, &mut d);
                } else {
                    log::debug!("Unable to save message to json string: {:?}", $message);
                }

                $data_maps.insert($topic.mrid.clone(), d);
                $data_maps.get(&$topic.mrid).unwrap()
            }
        }
    };
}

async fn handle_openfmb_message(clients: &Clients, msg: OpenFMBMessage) {
    let device_mrid = match msg.device_mrid() {
        Ok(mrid) => mrid.as_hyphenated().to_string(),
        Err(_) => "".to_string(),
    };
    if device_mrid.is_empty() {
        log::trace!("Missing device MRID in OpenFMB message.");
        return;
    }

    let mut update_messages: BTreeMap<String, Vec<UpdateMessage>> = BTreeMap::new();

    let mut data_maps: BTreeMap<String, BTreeMap<String, DataValue>> = BTreeMap::new();
    let dummy: BTreeMap<String, DataValue> = BTreeMap::new();

    clients.read().await.iter().for_each(|(_, client)| {
        for topic in &client.topics {
            if topic.name == "*" {
                if topic.mrid == device_mrid {
                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                    if let Ok(my_msg_json) = serde_json::to_string(&msg) {
                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                        let mut root = Node::new("mapping");
                        root.path = format!("{}Profile.mapping", msg.message_type());
                        root.from_json(&json, &mut d);

                        for (key, value) in d.into_iter() {
                            let update_msg = UpdateMessage {
                                profile: Some(msg.message_type().to_string()),
                                session_id: Some(client.session_id.clone()),
                                topic: crate::handler::Topic {
                                    name: key.clone(),
                                    mrid: device_mrid.clone(),
                                    value: Some(value.clone()),
                                    action: None,
                                    args: None,
                                    args2: None,
                                },
                            };

                            update_messages
                                .entry(client.session_id.clone())
                                .or_default()
                                .push(update_msg);
                        }
                    }
                }
            } else if topic.mrid == device_mrid {
                let data: &BTreeMap<String, DataValue> = match &msg {
                    OpenFMBMessage::BreakerEvent(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::BreakerReading(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::BreakerStatus(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::CapBankEvent(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::CapBankReading(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::CapBankStatus(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::CircuitSegmentEvent(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::CircuitSegmentStatus(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::EnvironmentReading(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::ESSEvent(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::ESSReading(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::ESSStatus(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::ESSCapability(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::ESSCapabilityOverride(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::EVSECapability(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::EVSECapabilityOverride(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::EVSEReading(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::EVSEStatus(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::EVSEEvent(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::GenerationReading(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::GenerationEvent(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::GenerationStatus(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::GenerationCapability(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::GenerationCapabilityOverride(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::LoadEvent(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::LoadReading(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::LoadStatus(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::MeterReading(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::RecloserEvent(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::RecloserReading(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::RecloserStatus(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::RegulatorEvent(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::RegulatorReading(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::RegulatorStatus(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::ResourceReading(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::ResourceEvent(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::ResourceStatus(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::SolarEvent(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::SolarReading(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::SolarStatus(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::SolarCapability(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::SolarCapabilityOverride(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::SwitchEvent(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::SwitchReading(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    OpenFMBMessage::SwitchStatus(message) => {
                        extract!(
                            data_maps,
                            message,
                            format!("{}Profile", msg.message_type()),
                            topic
                        )
                    }
                    _ => {
                        log::trace!("Unsupported message type: {:?}", msg);
                        &dummy
                    }
                };

                match data.get(&topic.name.to_lowercase()) {
                    Some(v) => {
                        let mut update_msg =
                            UpdateMessage::create(topic.clone(), client.session_id.clone(), None);
                        update_msg.topic.value = Some(v.clone());
                        update_messages
                            .entry(client.session_id.clone())
                            .or_default()
                            .push(update_msg);
                    }
                    _ => {
                        // ignore
                        log::trace!(
                            "Ignore message for topic {:?} and message {:?}",
                            topic,
                            &msg
                        );
                    }
                }
            }
        }
    });

    if !update_messages.is_empty() {
        for (key, value) in update_messages.into_iter() {
            let _ = send_updates(UpdateMessages::new(value, key.clone()), clients.clone()).await;
        }
    }
}

fn get_device_type_by_mrid(mrid: String) -> Option<String> {
    let devices = read_equipment_list().ok()?;
    for eq in devices.iter() {
        if eq.mrid == mrid {
            return eq.device_type.clone();
        }
    }
    None
}
fn get_common_control_profile(mrid: String) -> Option<String> {
    let t = match get_device_type_by_mrid(mrid) {
        Some(device_type) => match device_type.as_str() {
            "switch" => Some("SwitchDiscreteControlProfile".to_string()),
            "breaker" => Some("BreakerDiscreteControlProfile".to_string()),
            "recloser" => Some("RecloserDiscreteControlProfile".to_string()),
            "ess" => Some("EssControlProfile".to_string()),
            "solar" => Some("SolarControlProfile".to_string()),
            "generator" => Some("GenerationDiscreteControlProfile".to_string()),
            "regulator" => Some("RegulatorDiscreteControlProfile".to_string()),
            "load" => Some("LoadControlProfile".to_string()),
            "resource" => Some("ResourceDiscreteControlProfile".to_string()),
            _ => {
                log::info!(
                    "Unable to get common control profile for device type: {}",
                    device_type
                );
                None
            }
        },
        None => None,
    };
    t
}

async fn handle_generic_control(processor: &Processor, msg: GenericControl) {
    let token: Vec<&str> = msg.text.split('.').collect();
    let mut profile_name = String::from("");
    if let Some(p) = msg.profile_name.clone() {
        profile_name = p.clone();
    } else if token.len() > 1 && token[0].ends_with("Profile") {
        profile_name = token[0].to_string()
    } else if let Some(p) = get_common_control_profile(msg.mrid.clone().to_string()) {
        profile_name = p.clone();
    }

    use microgrid_protobuf as microgrid;
    match profile_name.as_str() {
        "BreakerDiscreteControlProfile" => {
            let subject = format!(
                "openfmb.breakermodule.BreakerDiscreteControlProfile.{}",
                &msg.mrid
            );
            match msg.message {
                microgrid::generic_control::ControlType::Open => {
                    let profile = BreakerDiscreteControlProfile::breaker_open_msg(&msg.mrid);
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::Close => {
                    let profile = BreakerDiscreteControlProfile::breaker_close_msg(&msg.mrid);
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::ResetBreaker => {
                    let profile = BreakerDiscreteControlProfile::breaker_reset_msg(&msg.mrid);
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                _ => {
                    warn!("Unsupport control type: {:?}", msg.message)
                }
            }
        }
        "RecloserDiscreteControlProfile" => {
            let subject = format!(
                "openfmb.reclosermodule.RecloserDiscreteControlProfile.{}",
                &msg.mrid
            );
            match msg.message {
                microgrid::generic_control::ControlType::Open => {
                    let profile = RecloserDiscreteControlProfile::recloser_open_msg(&msg.mrid);
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::Close => {
                    let profile = RecloserDiscreteControlProfile::recloser_close_msg(&msg.mrid);
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                _ => {
                    warn!("Unsupport control type: {:?}", msg.message)
                }
            }
        }
        "SwitchDiscreteControlProfile" => {
            let subject = format!(
                "openfmb.switchmodule.SwitchDiscreteControlProfile.{}",
                &msg.mrid
            );
            match msg.message {
                microgrid::generic_control::ControlType::Open => {
                    let profile = SwitchDiscreteControlProfile::switch_open_msg(&msg.mrid);
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::Close => {
                    let profile = SwitchDiscreteControlProfile::switch_close_msg(&msg.mrid);
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::SetModBlkOn => {
                    let profile = SwitchDiscreteControlProfile::switch_modblk_msg(&msg.mrid, true);
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::SetModBlkOff => {
                    let profile = SwitchDiscreteControlProfile::switch_modblk_msg(&msg.mrid, false);
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                _ => {
                    warn!("Unsupport control type: {:?}", msg.message)
                }
            }
        }
        "ESSControlProfile" | "EssControlProfile" => {
            let subject = format!("openfmb.essmodule.ESSControlProfile.{}", &msg.mrid);
            match msg.message {
                microgrid::generic_control::ControlType::SetModBlkOn => {
                    let profile = EssControlProfile::ess_modblk_msg(&msg.mrid, true);
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::SetModBlkOff => {
                    let profile = EssControlProfile::ess_modblk_msg(&msg.mrid, false);
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::SetValue => {
                    let profile =
                        EssControlProfile::discharge_now_msg(&msg.mrid, msg.args.unwrap());
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::ANetMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::ANetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::ANeutMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::ANeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::APhsAMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::APhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::APhsBMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::APhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::APhsCMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::APhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::HzMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::HzMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfNetMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfNeutMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfPhsAMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfPhsBMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfPhsCMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNetAng => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNetAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNetMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNeutAng => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNeutAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNeutMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsAAng => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsAAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsAMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsBAng => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsBAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsBMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsCAng => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsCAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsCMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsAbAng => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsAbAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsAbMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsAbMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsBcAng => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsBcAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsBcMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsBcMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsCaAng => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsCaAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsCaMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsCaMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaNetMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaNeutMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaPhsAMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaPhsBMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaPhsCMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArNetMag
                | microgrid::generic_control::ControlType::SetVarNetMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArNeutMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArPhsAMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArPhsBMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArPhsCMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WNetMag
                | microgrid::generic_control::ControlType::SetWNetMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::WNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WNeutMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::WNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WPhsAMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::WPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WPhsBMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::WPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WPhsCMag => {
                    let profile = EssControlProfile::schedule_ess_control(
                        &msg.mrid,
                        ScheduleParameterKind::WPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }

                microgrid::generic_control::ControlType::ResetEss => {
                    let profile = EssControlProfile::ess_reset_msg(&msg.mrid);
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                _ => match set_ess_csg(&msg.mrid, msg.message, SystemTime::now()) {
                    Some(profile) => {
                        let mut buffer = Vec::<u8>::new();
                        profile.encode(&mut buffer).unwrap();
                        processor.publisher.publish(subject, &mut buffer).await;
                    }
                    None => {
                        warn!("Unsupport control type: {:?}", msg.message)
                    }
                },
            }
        }
        "SolarControlProfile" => {
            let subject = format!("openfmb.solarmodule.SolarControlProfile.{}", &msg.mrid);
            match msg.message {
                microgrid::generic_control::ControlType::SetModBlkOn => {
                    let profile = SolarControlProfile::solar_modblk_msg(&msg.mrid, true);
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::SetModBlkOff => {
                    let profile = SolarControlProfile::solar_modblk_msg(&msg.mrid, false);
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::ANetMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::ANetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::ANeutMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::ANeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::APhsAMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::APhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::APhsBMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::APhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::APhsCMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::APhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::HzMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::HzMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfNetMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfNeutMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfPhsAMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfPhsBMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfPhsCMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNetAng => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNetAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNetMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNeutAng => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNeutAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNeutMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsAAng => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsAAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsAMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsBAng => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsBAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsBMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsCAng => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsCAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsCMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsAbAng => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsAbAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsAbMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsAbMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsBcAng => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsBcAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsBcMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsBcMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsCaAng => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsCaAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsCaMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsCaMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaNetMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaNeutMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaPhsAMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaPhsBMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaPhsCMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArNetMag
                | microgrid::generic_control::ControlType::SetVarNetMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArNeutMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArPhsAMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArPhsBMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArPhsCMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WNetMag
                | microgrid::generic_control::ControlType::SetWNetMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::WNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WNeutMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::WNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WPhsAMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::WPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WPhsBMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::WPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WPhsCMag => {
                    let profile = SolarControlProfile::schedule_solar_control(
                        &msg.mrid,
                        ScheduleParameterKind::WPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }

                microgrid::generic_control::ControlType::ResetSolar => {
                    let profile = SolarControlProfile::solar_reset_msg(&msg.mrid);
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                _ => match set_solar_csg(&msg.mrid, msg.message, SystemTime::now()) {
                    Some(profile) => {
                        let mut buffer = Vec::<u8>::new();
                        profile.encode(&mut buffer).unwrap();
                        processor.publisher.publish(subject, &mut buffer).await;
                    }
                    None => {
                        warn!("Unsupport control type: {:?}", msg.message)
                    }
                },
            }
        }
        "LoadControlProfile" => {
            let subject = format!("openfmb.loadmodule.LoadControlProfile.{}", &msg.mrid);
            match msg.message {
                microgrid::generic_control::ControlType::StateOn => {
                    let profile = LoadControlProfile::loadbank_on_msg(&msg.mrid, 125000.0);
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::StateOff => {
                    let profile = LoadControlProfile::loadbank_off_msg(&msg.mrid);
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::SetWNetMag
                | microgrid::generic_control::ControlType::SetValue => {
                    let profile =
                        LoadControlProfile::loadbank_on_msg(&msg.mrid, msg.args.unwrap().abs());
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::ANetMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::ANetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::ANeutMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::ANeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::APhsAMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::APhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::APhsBMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::APhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::APhsCMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::APhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::HzMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::HzMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfNetMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfNeutMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfPhsAMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfPhsBMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfPhsCMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNetAng => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNetAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNetMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNeutAng => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNeutAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNeutMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsAAng => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsAAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsAMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsBAng => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsBAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsBMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsCAng => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsCAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsCMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsAbAng => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsAbAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsAbMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsAbMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsBcAng => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsBcAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsBcMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsBcMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsCaAng => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsCaAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsCaMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsCaMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaNetMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaNeutMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaPhsAMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaPhsBMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaPhsCMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArNetMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArNeutMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArPhsAMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArPhsBMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArPhsCMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WNetMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::WNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WNeutMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::WNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WPhsAMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::WPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WPhsBMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::WPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WPhsCMag => {
                    let profile = LoadControlProfile::schedule_load_control(
                        &msg.mrid,
                        ScheduleParameterKind::WPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }

                microgrid::generic_control::ControlType::ResetLoad => {
                    let profile = LoadControlProfile::load_reset_msg(&msg.mrid);
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                _ => {
                    warn!("Unsupport control type: {:?}", msg.message)
                }
            }
        }
        "ResourceDiscreteControlProfile" => {
            let subject = format!(
                "openfmb.resourcemodule.ResourceDiscreteControlProfile.{}",
                &msg.mrid
            );
            match msg.message {
                microgrid::generic_control::ControlType::SetValue
                | microgrid::generic_control::ControlType::SetGgioValueAnalog => {
                    let profile = ResourceDiscreteControlProfile::set_double_msg(
                        &msg.mrid,
                        msg.args.unwrap(),
                        msg.args2.unwrap_or_default() as usize,
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::SetGgioValueInteger => {
                    let profile = ResourceDiscreteControlProfile::set_int_msg(
                        &msg.mrid,
                        msg.args.unwrap() as i32,
                        msg.args2.unwrap_or_default() as usize,
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::SetGgioValueBool => {
                    let profile = ResourceDiscreteControlProfile::set_bool_msg(
                        &msg.mrid,
                        msg.args.unwrap_or_default() > 0.0,
                        msg.args2.unwrap_or_default() as usize,
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::StartTransaction => {
                    let profile = ResourceDiscreteControlProfile::start_transaction(
                        &msg.mrid,
                        msg.args2.unwrap_or_default() as i32,
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::StopTransaction => {
                    let profile = ResourceDiscreteControlProfile::stop_transaction(
                        &msg.mrid,
                        msg.args2.map(|f| f as i32),
                        None,
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                _ => {
                    warn!("Unsupport control type: {:?}", msg.message)
                }
            }
        }
        "RegulatorDiscreteControlProfile" => {
            let subject = format!(
                "openfmb.regulatormodule.RegulatorDiscreteControlProfile.{}",
                &msg.mrid
            );
            match msg.message {
                microgrid::generic_control::ControlType::TapChangeLowerPhs3 => {
                    let profile = RegulatorDiscreteControlProfile::regulator_tap_lower_phs3_msg(
                        &msg.mrid, true,
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::TapChangeRaisePhs3 => {
                    let profile = RegulatorDiscreteControlProfile::regulator_tap_raise_phs3_msg(
                        &msg.mrid, true,
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::TapChangeLowerPhsA => {
                    let profile = RegulatorDiscreteControlProfile::regulator_tap_lower_phs_a_msg(
                        &msg.mrid, true,
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::TapChangeRaisePhsA => {
                    let profile = RegulatorDiscreteControlProfile::regulator_tap_raise_phs_a_msg(
                        &msg.mrid, true,
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::TapChangeLowerPhsB => {
                    let profile = RegulatorDiscreteControlProfile::regulator_tap_lower_phs_b_msg(
                        &msg.mrid, true,
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::TapChangeRaisePhsB => {
                    let profile = RegulatorDiscreteControlProfile::regulator_tap_raise_phs_b_msg(
                        &msg.mrid, true,
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::TapChangeLowerPhsC => {
                    let profile = RegulatorDiscreteControlProfile::regulator_tap_lower_phs_c_msg(
                        &msg.mrid, true,
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::TapChangeRaisePhsC => {
                    let profile = RegulatorDiscreteControlProfile::regulator_tap_raise_phs_c_msg(
                        &msg.mrid, true,
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::ANetMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::ANetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::ANeutMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::ANeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::APhsAMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::APhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::APhsBMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::APhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::APhsCMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::APhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::HzMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::HzMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfNetMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfNeutMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfPhsAMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfPhsBMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfPhsCMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNetAng => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNetAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNetMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNeutAng => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNeutAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNeutMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsAAng => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsAAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsAMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsBAng => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsBAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsBMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsCAng => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsCAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsCMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsAbAng => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsAbAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsAbMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsAbMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsBcAng => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsBcAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsBcMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsBcMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsCaAng => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsCaAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsCaMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsCaMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaNetMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaNeutMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaPhsAMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaPhsBMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaPhsCMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArNetMag
                | microgrid::generic_control::ControlType::SetVarNetMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArNeutMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArPhsAMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArPhsBMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArPhsCMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WNetMag
                | microgrid::generic_control::ControlType::SetWNetMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::WNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WNeutMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::WNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WPhsAMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::WPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WPhsBMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::WPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WPhsCMag => {
                    let profile = RegulatorControlProfile::schedule_regulator_control(
                        &msg.mrid,
                        ScheduleParameterKind::WPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                _ => {
                    warn!("Unsupport control type: {:?}", msg.message)
                }
            }
        }
        "GenerationControlProfile" => {
            let subject = format!(
                "openfmb.generationmodule.GenerationControlProfile.{}",
                &msg.mrid
            );
            match msg.message {
                microgrid::generic_control::ControlType::SetValue => {
                    let profile = GenerationControlProfile::generator_on_msg(
                        &msg.mrid,
                        msg.args.unwrap().abs(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::ANetMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::ANetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::ANeutMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::ANeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::APhsAMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::APhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::APhsBMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::APhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::APhsCMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::APhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::HzMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::HzMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfNetMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfNeutMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfPhsAMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfPhsBMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfPhsCMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNetAng => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNetAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNetMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNeutAng => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNeutAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNeutMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsAAng => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsAAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsAMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsBAng => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsBAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsBMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsCAng => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsCAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsCMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsAbAng => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsAbAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsAbMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsAbMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsBcAng => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsBcAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsBcMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsBcMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsCaAng => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsCaAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsCaMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsCaMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaNetMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaNeutMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaPhsAMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaPhsBMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaPhsCMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArNetMag
                | microgrid::generic_control::ControlType::SetVarNetMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArNeutMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArPhsAMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArPhsBMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArPhsCMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WNetMag
                | microgrid::generic_control::ControlType::SetWNetMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::WNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WNeutMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::WNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WPhsAMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::WPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WPhsBMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::WPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WPhsCMag => {
                    let profile = GenerationControlProfile::schedule_generation_control(
                        &msg.mrid,
                        ScheduleParameterKind::WPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }

                microgrid::generic_control::ControlType::ResetSolar => {
                    let profile = GenerationControlProfile::generation_reset_msg(&msg.mrid);
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                _ => match set_generation_csg(&msg.mrid, msg.message, SystemTime::now()) {
                    Some(profile) => {
                        let mut buffer = Vec::<u8>::new();
                        profile.encode(&mut buffer).unwrap();
                        processor.publisher.publish(subject, &mut buffer).await;
                    }
                    None => {
                        warn!("Unsupport control type: {:?}", msg.message)
                    }
                },
            }
        }
        "CapBankControlProfile" => {
            let subject = format!("openfmb.capbankmodule.CapBankControlProfile.{}", &msg.mrid);
            match msg.message {
                microgrid::generic_control::ControlType::ANetMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::ANetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::ANeutMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::ANeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::APhsAMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::APhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::APhsBMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::APhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::APhsCMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::APhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::HzMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::HzMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfNetMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfNeutMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfPhsAMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfPhsBMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PfPhsCMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::PfPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNetAng => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNetAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNetMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNeutAng => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNeutAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVNeutMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsAAng => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsAAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsAMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsBAng => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsBAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsBMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsCAng => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsCAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PhVPhsCMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::PhVPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsAbAng => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsAbAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsAbMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsAbMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsBcAng => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsBcAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsBcMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsBcMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsCaAng => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsCaAng,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::PpvPhsCaMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::PpvPhsCaMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaNetMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaNeutMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaPhsAMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaPhsBMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VaPhsCMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::VaPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArNetMag
                | microgrid::generic_control::ControlType::SetVarNetMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArNeutMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArPhsAMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArPhsBMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::VArPhsCMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::VArPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WNetMag
                | microgrid::generic_control::ControlType::SetWNetMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::WNetMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WNeutMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::WNeutMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WPhsAMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::WPhsAMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WPhsBMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::WPhsBMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                microgrid::generic_control::ControlType::WPhsCMag => {
                    let profile = CapBankControlProfile::schedule_capbank_control(
                        &msg.mrid,
                        ScheduleParameterKind::WPhsCMag,
                        msg.args.unwrap(),
                        SystemTime::now(),
                    );
                    let mut buffer = Vec::<u8>::new();
                    profile.encode(&mut buffer).unwrap();
                    processor.publisher.publish(subject, &mut buffer).await;
                }
                _ => {
                    warn!("Unsupport control type: {:?}", msg.message)
                }
            }
        }
        _ => {
            println!("Unsupported generic control for profile {}", profile_name);
        }
    }
}
