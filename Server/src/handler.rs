use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tokio::sync::{mpsc, RwLock};
use warp::{http::StatusCode, reply::json, ws::Message, ws::WebSocket, Reply, Rejection};
use riker::actors::*;
use futures::{FutureExt, StreamExt};
use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use log::info;
use super::hmi;
use hmi::processor::ProcessorMsg;

use microgrid_protobuf as microgrid;

pub type Result<T> = std::result::Result<T, Rejection>;
pub type Clients = Arc<RwLock<HashMap<String, Client>>>;

#[derive(Debug, Clone)]
pub struct MicrogridControl {
    pub text: String,
    pub message: microgrid::microgrid_control::ControlMessage,
}

#[derive(Debug, Clone)]
pub struct DeviceControl {
    pub text: String,
    pub message: microgrid::device_control::DeviceControlMessage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DataValue
{
    Bool(bool),
    Double(f64),
    String(String)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Topic {
    pub name: String,
    pub mrid: String,
    pub value: Option<DataValue>    
}

#[derive(Debug, Clone)]
pub struct Client {
    pub session_id: String,
    pub topics: Vec<Topic>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    session_id: Option<String>,
    topics: Vec<Topic>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Equipment {
    mrid: String,
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {      
    name: String,
    attributes: Option<CommandAttributes>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandAttributes {    
    name: String,
    path: String
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Diagram {
    diagramId: String,
    name: Option<String>,
    description: Option<String>,
    location: Option<String>,
    data: Option<String>,
    createdDate: Option<String>,
    createdBy: Option<String>,
    backgroundColor: Option<String>
}

#[derive(Serialize, Debug)]
pub struct Response {
    success: bool,
    message: String,
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMessage {
    pub topic: Topic,
    pub session_id: Option<String>
}

impl UpdateMessage {
    pub fn create(topic: Topic, session_id: String) -> UpdateMessage {
        UpdateMessage {
            topic: topic,
            session_id: Some(session_id)
        }
    }
}

#[derive(Serialize, Debug)]
pub struct UpdateMessages {
    updates: Vec<UpdateMessage>
}

#[derive(Deserialize)]
pub struct DiagramQuery {
    id: String,
}

impl UpdateMessages {
    pub fn new(messages: Vec<UpdateMessage>) -> UpdateMessages {
        UpdateMessages {
            updates: messages
        }
    }
}

pub async fn data_handler(update: UpdateMessage, clients: Clients, processor: ActorRef<ProcessorMsg>) -> Result<impl Reply> {
    info!("Handle data: {:?}", update);
    clients
        .read()
        .await
        .iter()
        .filter(|(_, client)| match update.session_id.clone() {
            Some(v) => client.session_id == v,
            None => true,
        })
        //.filter(|(_, client)| client.topics.contains(&update.topic.to_string()))
        .for_each(|(_, client)| {
            if let Some(_sender) = &client.sender {                 
                if update.topic.name == "ResetDevices" {
                    processor.tell(
                        MicrogridControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::microgrid_control::ControlMessage::ResetDevices("".to_string()),
                        },
                        None,
                    );
                }
                else if update.topic.name == "Shutdown" {
                    processor.tell(
                        MicrogridControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::microgrid_control::ControlMessage::Shutdown("".to_string()),
                        },
                        None,
                    );
                }
                else if update.topic.name == "InitiateIsland" {
                    processor.tell(
                        MicrogridControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::microgrid_control::ControlMessage::InitiateIsland("".to_string()),
                        },
                        None,
                    );
                }
                else if update.topic.name == "InitiateGridConnect" {
                    processor.tell(
                        MicrogridControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::microgrid_control::ControlMessage::InitiateGridConnect("".to_string()),
                        },
                        None,
                    );
                }
                else if update.topic.name == "EnableNetZero" {
                    processor.tell(
                        MicrogridControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::microgrid_control::ControlMessage::EnableNetZero("".to_string()),
                        },
                        None,
                    );
                }
                else if update.topic.name == "DisableNetZero" {
                    processor.tell(
                        MicrogridControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::microgrid_control::ControlMessage::DisableNetZero("".to_string()),
                        },
                        None,
                    );
                }
                else if update.topic.name == "ReconnectPretestOne" {
                    processor.tell(
                        MicrogridControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::microgrid_control::ControlMessage::ReconnectPretestOne("".to_string()),
                        },
                        None,
                    );
                }
                else if update.topic.name == "ReconnectPretestTwo" {
                    processor.tell(
                        MicrogridControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::microgrid_control::ControlMessage::ReconnectPretestTwo("".to_string()),
                        },
                        None,
                    );
                }
                else if update.topic.name == "ReconnectTest" {
                    processor.tell(
                        MicrogridControl {
                            text:update.topic.name.clone(),
                            message:  microgrid::microgrid_control::ControlMessage::ReconnectTest("".to_string()),
                        },
                        None,
                    );
                }
                // Device controls
                else if update.topic.name == "EnableSolarInverter" {
                    processor.tell(
                        DeviceControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::device_control::DeviceControlMessage::EnableSolarInverter,
                        },
                        None,
                    );
                }
                else if update.topic.name == "DisableSolarInverter" {
                    processor.tell(
                        DeviceControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::device_control::DeviceControlMessage::DisableSolarInverter,
                        },
                        None,
                    );
                }
                else if update.topic.name == "EnableLoadbank" {
                    processor.tell(
                        DeviceControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::device_control::DeviceControlMessage::EnableLoadbank,
                        },
                        None,
                    );
                }
                else if update.topic.name == "DisableLoadbank" {
                    processor.tell(
                        DeviceControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::device_control::DeviceControlMessage::DisableLoadbank,
                        },
                        None,
                    );
                }
                else if update.topic.name == "EssStart" {
                    processor.tell(
                        DeviceControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::device_control::DeviceControlMessage::EssStart,
                        },
                        None,
                    );
                }
                else if update.topic.name == "EssDischarge" {
                    processor.tell(
                        DeviceControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::device_control::DeviceControlMessage::EssDischarge,
                        },
                        None,
                    );
                }
                else if update.topic.name == "EssSocManage" {
                    processor.tell(
                        DeviceControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::device_control::DeviceControlMessage::EssSocManage,
                        },
                        None,
                    );
                }
                else if update.topic.name == "EssSocLimits" {
                    processor.tell(
                        DeviceControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::device_control::DeviceControlMessage::EssSocLimits,
                        },
                        None,
                    );
                }
                else if update.topic.name == "EssStop" {
                    processor.tell(
                        DeviceControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::device_control::DeviceControlMessage::EssStop,
                        },
                        None,
                    );
                }
                else if update.topic.name == "GeneratorOn" {
                    processor.tell(
                        DeviceControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::device_control::DeviceControlMessage::GeneratorOn,
                        },
                        None,
                    );
                }
                else if update.topic.name == "GeneratorDisabled" {
                    processor.tell(
                        DeviceControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::device_control::DeviceControlMessage::GeneratorDisabled,
                        },
                        None,
                    );
                }
                else if update.topic.name == "GeneratorEnabled" {
                    processor.tell(
                        DeviceControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::device_control::DeviceControlMessage::GeneratorEnabled,
                        },
                        None,
                    );
                }
                else if update.topic.name == "GeneratorOff" {
                    processor.tell(
                        DeviceControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::device_control::DeviceControlMessage::GeneratorOff,
                        },
                        None,
                    );
                }
                else if update.topic.name == "SwitchOneOpen" {
                    processor.tell(
                        DeviceControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::device_control::DeviceControlMessage::SwitchOneOpen,
                        },
                        None,
                    );
                }
                else if update.topic.name == "SwitchOneClosed" {
                    processor.tell(
                        DeviceControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::device_control::DeviceControlMessage::SwitchOneClosed,
                        },
                        None,
                    );
                }
                else if update.topic.name == "SwitchTwoOpen" {
                    processor.tell(
                        DeviceControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::device_control::DeviceControlMessage::SwitchTwoOpen,
                        },
                        None,
                    );
                }
                else if update.topic.name == "SwitchTwoClosed" {
                    processor.tell(
                        DeviceControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::device_control::DeviceControlMessage::SwitchTwoClosed,
                        },
                        None,
                    );
                }
                else if update.topic.name == "BreakerThreeOpen" {
                    processor.tell(
                        DeviceControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::device_control::DeviceControlMessage::BreakerThreeOpen,
                        },
                        None,
                    );
                }
                else if update.topic.name == "BreakerThreeClosed" {
                    processor.tell(
                        DeviceControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::device_control::DeviceControlMessage::BreakerThreeClosed,
                        },
                        None,
                    );
                }
                else if update.topic.name == "SwitchFourOpen" {
                    processor.tell(
                        DeviceControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::device_control::DeviceControlMessage::SwitchFourOpen,
                        },
                        None,
                    );
                }
                else if update.topic.name == "SwitchFourClosed" {
                    processor.tell(
                        DeviceControl {
                            text: update.topic.name.clone(),
                            message:  microgrid::device_control::DeviceControlMessage::SwitchFourClosed,
                        },
                        None,
                    );
                }
                else {
                    info!("Received unknown command: {}", update.topic.name);
                }                
            }
        });

    Ok(StatusCode::OK)
}

pub async fn send_updates(updates: UpdateMessages, clients: Clients) -> Result<impl Reply> {
    
    clients
        .read()
        .await
        .iter()
        // .filter(|(_, client)| match update.session_id.clone() {
        //     Some(v) => client.session_id == v,
        //     None => true,
        // })
        //.filter(|(_, client)| client.topics.contains(&update.topic.to_string()))
        .for_each(|(_, client)| {
            if let Some(sender) = &client.sender {                          
                let json = serde_json::to_string(&updates).unwrap();
                let _ = sender.send(Ok(Message::text(json)));                
            }
        });

    Ok(StatusCode::OK)
}

pub async fn execute_command(update: Command, _clients: Clients) -> Result<impl Reply> {
    println!("Received command '{}'", update.name);
    Ok(StatusCode::OK)
}

fn get_diagram_folder() -> String {
    let app_dir = std::env::var("APP_DIR_NAME").unwrap_or_else(|_| "".into());
    if app_dir != "" {
        return format!("/{}/diagrams", app_dir);
    }
    "diagrams".to_string()
}

// POST
pub async fn save_handler(request: Diagram) -> Result<impl Reply> {

    let j = serde_json::to_string(&request).unwrap();

    write_json(format!("{}/{}.json", get_diagram_folder(), request.diagramId), j).unwrap();

     Ok(json(&Response {
        success: true,
        message: "".to_string()
    }))
}

// POST
pub async fn delete_handler(request: Diagram) -> Result<impl Reply> {    
    let _ = fs::remove_file(format!("/{}/{}.json", get_diagram_folder(), request.diagramId));
    Ok(json(&Response {
        success: true,
        message: "".to_string()
    }))
}

// GET
pub async fn list_handler() -> Result<impl Reply> {
    Ok(json(&read_json(format!("{}", get_diagram_folder())).unwrap()))
}

// GET
pub async fn equipment_handler() -> Result<impl Reply> {  
    
    let config = riker::load_config();
    let mut equipment_list = vec![];

    if let Ok(list) = config.get_array("circuit_segment_devices.all_devices") {
        for item in list {
            let device_name = item.into_str().unwrap();
            
            let eq = Equipment {
                name: config.get_str(&format!("circuit_segment_devices.{}.name", device_name)).unwrap(),
                mrid: config.get_str(&format!("circuit_segment_devices.{}.mrid", device_name)).unwrap(),
            };
            
            equipment_list.push(eq);
        }
    }        
      
    Ok(json(&equipment_list))
}

// GET
pub async fn command_handler() -> Result<impl Reply> {  
    Ok(json(&read_commands(format!("{}/command.json", get_diagram_folder())).unwrap()))   
}

// GET
pub async fn diagram_handler(id: DiagramQuery) -> Result<impl Reply> {    
    let list = read_json(format!("{}", get_diagram_folder())).unwrap();
    let id = id.id;
    for d in list {
        if d.diagramId == id {
            return Ok(json(&d));
        }
    }

    Err(warp::reject::not_found())
}

pub async fn hmi_handler(ws: warp::ws::Ws, id: String, clients: Clients) -> Result<impl Reply> {
    return Ok(ws.on_upgrade(move |socket| client_connection(socket, id, clients)));
}

pub async fn client_connection(ws: WebSocket, id: String, clients: Clients) {
    let mut client = match clients.read().await.get(&id).cloned() {
        Some(c) => c,
        None => {
            Client {
                session_id: id.clone(),
                sender: None,
                topics: vec![]
            }
        }
    };

    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("ERROR::Error sending websocket msg: {}", e);
        }
    }));

    client.sender = Some(client_sender);
    clients.write().await.insert(id.clone(), client);

    println!("Client id '{}' connected", id);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("ERROR::Error receiving message for client id: {}): {}", id.clone(), e);
                break;
            }
        };
        client_msg(&id, msg, &clients).await;
    }

    clients.write().await.remove(&id);
    println!("Client id '{}' disconnected", id);
}

async fn client_msg(id: &str, msg: Message, clients: &Clients) {
    //println!("Received message from {}: {:?}", id, msg);
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    let register_request: RegisterRequest = match from_str(&message) {
        Ok(v) => v,
        Err(e) => {
            println!("Only can handle RegisterRequest at this moment. {:?}", e);
            return;
        }
    };

    let mut locked = clients.write().await;
    match locked.get_mut(id) {
        Some(v) => {
            let mut topics = vec![];
            for t in register_request.topics.iter() {
                topics.push(t.clone());
            }
            v.topics = topics;
        }
        None => {
            println!("ERROR::Client '{}' not found.", id);
        },
    };
}

fn read_json(file_path: String) -> std::io::Result<Vec<Diagram>> {    

    let mut diagrams: Vec<Diagram> = vec![];    
    for entry in fs::read_dir(&file_path)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            let mut file = File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            diagrams.push(serde_json::from_str(&contents).expect("JSON was not well-formatted"));
        } 
    }
        

    // let mut file = File::open(file_path)?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;

    // let diagrams: Vec<Diagram> =
    //     serde_json::from_str(&contents).expect("JSON was not well-formatted");

    Ok(diagrams)
}

fn read_commands(file_path: String) -> std::io::Result<Vec<Command>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let eqs: Vec<Command> =
        serde_json::from_str(&contents).expect("JSON was not well-formatted");

    Ok(eqs)
}

fn write_json(file_path: String, json: String) -> std::io::Result<()> {

    fs::write(file_path, json).expect("Unable to write file");

    Ok(())
}
