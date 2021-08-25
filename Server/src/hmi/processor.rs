// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::messages::*;
use crate::handler::*;

use super::hmi_publisher::HmiPublisherMsg;
use super::pubsub::PubSubStatus;

use riker::actors::*;
use serde_json::Value;
use std::collections::btree_map::BTreeMap;

use log::debug;

pub struct Node {
    name: String,
    path: String,
    value: String,
    nodes: Vec<Node>
}

impl Node {
    pub fn new(name: &str) -> Node {
        Node {
            nodes: vec![],
            name: String::from(name),
            path: String::from(""),
            value: String::from("")                                 
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
        if n.path == "" {
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
                    child.from_json(&v, data);
                    self.add_node(child);
                }
            },
            Value::Array(a) => {
                let original_path = self.path();
                for (i, v) in a.iter().enumerate() {                    
                    self.path = format!("{}[{}]", original_path, i);                                        
                    self.from_json(&v, data);
                }
            },
            Value::Bool(b) => {                
                self.set_value(b.to_string().as_str());
                data.insert(self.path().replace("_", "").to_lowercase(), DataValue::Bool(b.clone()));
            },
            Value::Number(n) => {                
                self.set_value(n.to_string().as_str());
                data.insert(self.path().replace("_", "").to_lowercase(), DataValue::Double(n.as_f64().unwrap()));
            },
            Value::String(s) => {                
                self.set_value(s.as_str());
                data.insert(self.path().replace("_", "").to_lowercase(), DataValue::String(s.clone()));
            },
            Value::Null => {
                // ignore
            }
        }
    }    
}

#[actor(OpenFMBMessage, PublisherRefWrap, MicrogridControl, DeviceControl, GenericControl, PubSubStatus)]
#[derive(Clone, Debug)]
pub struct Processor {
    message_count: u32,                
    publisher: ActorRef<HmiPublisherMsg>,
    clients: Clients,    
}

impl ActorFactoryArgs<(ActorRef<HmiPublisherMsg>, Clients)> for Processor
{
    fn create_args(
        args: (ActorRef<HmiPublisherMsg>, Clients),
    ) -> Self {
        Processor {
            message_count: 0,                                              
            publisher: args.0,
            clients: args.1          
        }
    }
}

impl Actor for Processor {
    type Msg = ProcessorMsg;

    fn pre_start(&mut self, _ctx: &Context<Self::Msg>) {}

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) {
        self.message_count += 1;
        self.receive(ctx, msg, sender);
    }
}

type PublisherRefWrap = ActorRefWrap<HmiPublisherMsg>;

impl Receive<ActorRefWrap<HmiPublisherMsg>> for Processor {
    type Msg = ProcessorMsg;
    fn receive(
        &mut self,
        _ctx: &Context<Self::Msg>,
        msg: ActorRefWrap<HmiPublisherMsg>,
        _sender: Sender,
    ) {
        self.publisher = msg.clone().0;        
    }
}

impl Receive<OpenFMBMessage> for Processor {
    type Msg = ProcessorMsg;    

    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: OpenFMBMessage, _sender: Sender) {      
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(handle_openfmb_message(&self.clients, msg));
    }  
}

impl Receive<MicrogridControl> for Processor {
    type Msg = ProcessorMsg;    

    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: MicrogridControl, sender: Sender) {      
        debug!("Received microgrid control message {:?}", msg);
        self.publisher.tell(msg, sender)
    } 
}

impl Receive<DeviceControl> for Processor {
    type Msg = ProcessorMsg;    

    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: DeviceControl, sender: Sender) {      
        debug!("Received device control message {:?}", msg);
        self.publisher.tell(msg, sender)
    } 
}

impl Receive<PubSubStatus> for Processor {
    type Msg = ProcessorMsg;    

    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: PubSubStatus, _sender: Sender) {              
        //debug!("Received pub/sub status {:?}", msg);
        let rt = tokio::runtime::Runtime::new().unwrap();
        let clients = self.clients.clone();
        rt.spawn(async {            
            let _ = send_status(msg, clients).await;
        });        
    } 
}

impl Receive<GenericControl> for Processor {
    type Msg = ProcessorMsg;    

    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: GenericControl, _sender: Sender) {      
        debug!("Received generic control message {:?}", msg);
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(handle_generic_control(self, msg));
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
                }
                $data_maps.insert($topic.mrid.clone(), d);
                $data_maps.get(&$topic.mrid).unwrap()
            }
        }    
    };
}

async fn handle_openfmb_message(clients: &Clients, msg: OpenFMBMessage) {
    let device_mrid = match msg.device_mrid() {
        Ok(mrid) => {
            mrid.to_hyphenated().to_string()
        },
        Err(_) => {
            "".to_string()
        }
    };
    if device_mrid.len() == 0 {
        return;
    }

    let mut update_messages: BTreeMap<String, Vec<UpdateMessage>> = BTreeMap::new();    

    let mut data_maps : BTreeMap<String, BTreeMap<String, DataValue>> = BTreeMap::new();
    let dummy : BTreeMap<String, DataValue> = BTreeMap::new();             

    clients
        .read()
        .await
        .iter()               
        .for_each(|(_, client)| {             
            for topic in &client.topics {
                if topic.name == "*" {
                    if topic.mrid == device_mrid {
                        let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                        if let Ok(my_msg_json) = serde_json::to_string(&msg) {
                            let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                            let mut root = Node::new("mapping");
                            root.path = format!("{}Profile.mapping", msg.message_type().to_string());
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
                                    },
                                };

                                update_messages.entry(client.session_id.clone()).or_insert(Vec::new()).push(update_msg);
                            }
                        }
                    }
                }
                else if topic.mrid == device_mrid {                      
                    let data : &BTreeMap<String, DataValue> = match &msg {
                        OpenFMBMessage::BreakerEvent(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)            
                        },
                        OpenFMBMessage::BreakerReading(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)
                        },
                        OpenFMBMessage::BreakerStatus(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)               
                        },
                        OpenFMBMessage::CapBankEvent(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                 
                        },
                        OpenFMBMessage::CapBankReading(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                 
                        },
                        OpenFMBMessage::CapBankStatus(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                 
                        },
                        OpenFMBMessage::CoordinationEvent(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                 
                        },
                        OpenFMBMessage::CoordinationStatus(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                 
                        },
                        OpenFMBMessage::ESSEvent(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                 
                        },
                        OpenFMBMessage::ESSReading(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                 
                        },
                        OpenFMBMessage::ESSStatus(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                 
                        },
                        OpenFMBMessage::GenerationReading(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                 
                        },
                        OpenFMBMessage::GenerationEvent(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)               
                        },
                        OpenFMBMessage::GenerationStatus(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)             
                        },
                        OpenFMBMessage::LoadEvent(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                 
                        },
                        OpenFMBMessage::LoadReading(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                
                        },
                        OpenFMBMessage::LoadStatus(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)               
                        },
                        OpenFMBMessage::MeterReading(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                 
                        },
                        OpenFMBMessage::RecloserEvent(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)              
                        },
                        OpenFMBMessage::RecloserReading(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                
                        },
                        OpenFMBMessage::RecloserStatus(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)               
                        },
                        OpenFMBMessage::RegulatorEvent(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                
                        },
                        OpenFMBMessage::RegulatorReading(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                 
                        },
                        OpenFMBMessage::RegulatorStatus(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                  
                        },
                        OpenFMBMessage::ResourceReading(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                 
                        },
                        OpenFMBMessage::ResourceEvent(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                
                        },
                        OpenFMBMessage::ResourceStatus(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                 
                        },
                        OpenFMBMessage::SolarEvent(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                 
                        },
                        OpenFMBMessage::SolarReading(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                 
                        },
                        OpenFMBMessage::SolarStatus(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                  
                        },
                        OpenFMBMessage::SwitchEvent(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                 
                        },
                        OpenFMBMessage::SwitchReading(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                  
                        },
                        OpenFMBMessage::SwitchStatus(message) => {
                            extract!(data_maps, message, format!("{}Profile", msg.message_type()), topic)                 
                        },                        
                        _ => {
                            &dummy
                        }
                    };

                    match data.get(&topic.name.to_lowercase()) {
                        Some(v) => {
                            let mut update_msg = UpdateMessage::create(topic.clone(), client.session_id.clone(), None);
                            update_msg.topic.value = Some(v.clone()); 
                            update_messages.entry(client.session_id.clone()).or_insert(Vec::new()).push(update_msg);
                        },
                        _ => {
                            // ignore
                        }
                    }
                }                
            }
        });

    if update_messages.len() > 0 {        
        for (key, value) in update_messages.into_iter() {
            let _ = send_updates(UpdateMessages::new(value, key.clone()), clients.clone()).await;
        }
    }    
}

async fn handle_generic_control(processor: &Processor, msg: GenericControl) {

    processor.publisher.send_msg(msg.into(), None);    
}