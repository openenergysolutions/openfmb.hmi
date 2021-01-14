use circuit_segment_manager::actors::coordinator::openfmb::openfmb::OpenFMBDeviceMsg;
use circuit_segment_manager::messages::*;

use super::hmi_publisher::HmiPublisherMsg;
use crate::handler::*;

use riker::actors::*;
use serde_json::Value;
use log::info;
use std::collections::HashMap;

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
    pub fn from_json(&mut self, json_value: &Value, data: &mut HashMap<String, DataValue>) {
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
    openfmb_device: Option<ActorRef<OpenFMBDeviceMsg>>,            
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
            openfmb_device: None,                                   
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
        let mut rt = tokio::runtime::Runtime::new().unwrap();
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
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(handle_generic_control(self, msg));
    } 
}

async fn handle_openfmb_message(clients: &Clients, msg: OpenFMBMessage) {    

    let mut update_messages = vec![];

    let mut data_maps : HashMap<String, HashMap<String, DataValue>> = HashMap::new();
    let dummy : HashMap<String, DataValue> = HashMap::new();

    let device_mrid = format!("{}", msg.device_mrid().unwrap());

    clients
        .read()
        .await
        .iter()               
        .for_each(|(_, client)| {                         
            for topic in &client.topics {                                
                if topic.mrid == device_mrid {  
                    
                    let data : &HashMap<String, DataValue> = match &msg {
                        OpenFMBMessage::GenerationReading(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: HashMap<String, DataValue> = HashMap::new();
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
                        OpenFMBMessage::GenerationStatus(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: HashMap<String, DataValue> = HashMap::new();
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
                        OpenFMBMessage::SwitchReading(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: HashMap<String, DataValue> = HashMap::new();
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
                                    let mut d: HashMap<String, DataValue> = HashMap::new();
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
                        OpenFMBMessage::MeterReading(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: HashMap<String, DataValue> = HashMap::new();
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
                        OpenFMBMessage::SolarReading(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: HashMap<String, DataValue> = HashMap::new();
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
                                    let mut d: HashMap<String, DataValue> = HashMap::new();
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
                        OpenFMBMessage::ESSReading(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: HashMap<String, DataValue> = HashMap::new();
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
                                    let mut d: HashMap<String, DataValue> = HashMap::new();
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
                        OpenFMBMessage::LoadReading(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: HashMap<String, DataValue> = HashMap::new();
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
                                    let mut d: HashMap<String, DataValue> = HashMap::new();
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
                        OpenFMBMessage::BreakerReading(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: HashMap<String, DataValue> = HashMap::new();
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
                                    let mut d: HashMap<String, DataValue> = HashMap::new();
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
                        OpenFMBMessage::RecloserReading(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: HashMap<String, DataValue> = HashMap::new();
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
                                    let mut d: HashMap<String, DataValue> = HashMap::new();
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
                        OpenFMBMessage::ResourceStatus(message) => {
                            match data_maps.get(&topic.mrid) {
                                Some(d) => d,
                                _ => {
                                    let mut d: HashMap<String, DataValue> = HashMap::new();
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
                        _ => {
                            &dummy
                        }
                    };

                    match data.get(&topic.name.to_lowercase()) {
                        Some(v) => {
                            let mut update_msg = UpdateMessage::create(topic.clone(), client.session_id.clone());
                            update_msg.topic.value = Some(v.clone());                            
                            update_messages.push(update_msg);
                        },
                        _ => {
                            // ignore
                        }
                    }
                }                
            }                         
        });

    if update_messages.len() > 0 {
        let _ = send_updates(UpdateMessages::new(update_messages), clients.clone()).await;
        info!("Update sent");
    }
}

async fn handle_generic_control(processor: &Processor, msg: GenericControl) {

    processor.publisher.send_msg(msg.into(), None);    
}