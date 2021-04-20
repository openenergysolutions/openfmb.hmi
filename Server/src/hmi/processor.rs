use crate::messages::*;

use super::hmi_publisher::HmiPublisherMsg;
use crate::handler::*;

use riker::actors::*;
use serde_json::Value;
use std::collections::btree_map::BTreeMap;

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

#[actor(OpenFMBMessage, PublisherRefWrap, MicrogridControl, DeviceControl, GenericControl)]
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

    fn pre_start(&mut self, _ctx: &Context<Self::Msg>) {   
    }

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
        println!("Received microgrid control message {:?}", msg);
        self.publisher.tell(msg, sender)
    } 
}

impl Receive<DeviceControl> for Processor {
    type Msg = ProcessorMsg;    

    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: DeviceControl, sender: Sender) {      
        println!("Received device control message {:?}", msg);
        self.publisher.tell(msg, sender)
    } 
}

impl Receive<GenericControl> for Processor {
    type Msg = ProcessorMsg;    

    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: GenericControl, _sender: Sender) {      
        println!("Received generic control message {:?}", msg);
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(handle_generic_control(self, msg));
    } 
}

async fn handle_openfmb_message(clients: &Clients, msg: OpenFMBMessage) {

    let mut update_messages: BTreeMap<String, Vec<UpdateMessage>> = BTreeMap::new();    

    let mut data_maps : BTreeMap<String, BTreeMap<String, DataValue>> = BTreeMap::new();
    let dummy : BTreeMap<String, DataValue> = BTreeMap::new();

    let device_mrid = format!("{}", msg.device_mrid().unwrap());

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
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "BreakerEventProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::BreakerReading(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "BreakerReadingProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::BreakerStatus(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "BreakerStatusProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::CapBankEvent(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "CapBankEventProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::CapBankReading(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "CapBankReadingProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::CapBankStatus(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "CapBankStatusProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::CoordinationEvent(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "CoordinationEventProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::CoordinationStatus(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "CoordinationStatusProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::ESSEvent(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "ESSEventProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::ESSReading(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "ESSReadingProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::ESSStatus(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "ESSStatusProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::GenerationReading(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "GenerationReadingProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::GenerationEvent(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "GenerationEventProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::GenerationStatus(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "GenerationStatusProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::LoadEvent(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "LoadEventProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::LoadReading(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "LoadReadingProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::LoadStatus(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "LoadStatusProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::MeterReading(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "MeterReadingProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::RecloserEvent(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "RecloserEventProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::RecloserReading(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "RecloserReadingProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::RecloserStatus(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "RecloserStatusProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::RegulatorEvent(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "RegulatorEventProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::RegulatorReading(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "RegulatorReadingProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::RegulatorStatus(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "RegulatorStatusProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::ResourceReading(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "ResourceReadingProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::ResourceEvent(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "ResourceEventProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::ResourceStatus(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "ResourceStatusProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::SolarEvent(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "SolarEventProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::SolarReading(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "SolarReadingProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::SolarStatus(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "SolarStatusProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::SwitchEvent(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "SwitchEventProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::SwitchReading(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "SwitchReadingProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
                        },
                        OpenFMBMessage::SwitchStatus(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: BTreeMap<String, DataValue> = BTreeMap::new();
                                    if let Ok(my_msg_json) = serde_json::to_string(message) {
                                        let json: Value = serde_json::from_str(&my_msg_json).unwrap();
                                        let mut root = Node::new("mapping");
                                        root.path = "SwitchStatusProfile.mapping".to_string();
                                        root.from_json(&json, &mut d);  
                                    }
                                    data_maps.insert(topic.mrid.clone(), d);
                                    data_maps.get(&topic.mrid).unwrap()
                                }
                            }                  
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