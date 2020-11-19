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
pub struct Topic {
    pub name: String,
    pub mrid: String,
    pub value: Option<f64>    
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
    label: String,
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

impl UpdateMessages {
    pub fn new(messages: Vec<UpdateMessage>) -> UpdateMessages {
        UpdateMessages {
            updates: messages
        }
    }
}

pub async fn data_handler(update: UpdateMessage, clients: Clients, processor: ActorRef<ProcessorMsg>) -> Result<impl Reply> {
    println!("Handle data: {:?}", update);
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
                println!("Received command: {:?}", update);

                if update.topic.name == "ResetDevices" {
                    processor.tell(
                        MicrogridControl {
                            text: "ResetDevices".to_string(),
                            message:  microgrid::microgrid_control::ControlMessage::ResetDevices("".to_string()),
                        },
                        None,
                    );
                }
                else if update.topic.name == "Shutdown" {
                    processor.tell(
                        MicrogridControl {
                            text: "Shutdown".to_string(),
                            message:  microgrid::microgrid_control::ControlMessage::Shutdown("".to_string()),
                        },
                        None,
                    );
                }
                else if update.topic.name == "InitiateIsland" {
                    processor.tell(
                        MicrogridControl {
                            text: "InitiateIsland".to_string(),
                            message:  microgrid::microgrid_control::ControlMessage::InitiateIsland("".to_string()),
                        },
                        None,
                    );
                }
                else if update.topic.name == "InitiateGridConnect" {
                    processor.tell(
                        MicrogridControl {
                            text: "InitiateGridConnect".to_string(),
                            message:  microgrid::microgrid_control::ControlMessage::InitiateGridConnect("".to_string()),
                        },
                        None,
                    );
                }
                else if update.topic.name == "EnableNetZero" {
                    processor.tell(
                        MicrogridControl {
                            text: "EnableNetZero".to_string(),
                            message:  microgrid::microgrid_control::ControlMessage::EnableNetZero("".to_string()),
                        },
                        None,
                    );
                }
                else if update.topic.name == "DisableNetZero" {
                    processor.tell(
                        MicrogridControl {
                            text: "DisableNetZero".to_string(),
                            message:  microgrid::microgrid_control::ControlMessage::DisableNetZero("".to_string()),
                        },
                        None,
                    );
                }
                else if update.topic.name == "ReconnectPretestOne" {
                    processor.tell(
                        MicrogridControl {
                            text: "ReconnectPretestOne".to_string(),
                            message:  microgrid::microgrid_control::ControlMessage::ReconnectPretestOne("".to_string()),
                        },
                        None,
                    );
                }
                else if update.topic.name == "ReconnectPretestTwo" {
                    processor.tell(
                        MicrogridControl {
                            text: "ReconnectPretestTwo".to_string(),
                            message:  microgrid::microgrid_control::ControlMessage::ReconnectPretestTwo("".to_string()),
                        },
                        None,
                    );
                }
                else if update.topic.name == "ReconnectTest" {
                    processor.tell(
                        MicrogridControl {
                            text: "ReconnectTest".to_string(),
                            message:  microgrid::microgrid_control::ControlMessage::ReconnectTest("".to_string()),
                        },
                        None,
                    );
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

// POST
pub async fn save_handler(request: Diagram) -> Result<impl Reply> {

    let list = vec![request];

    let j = serde_json::to_string(&list).unwrap();

    write_json(String::from("data.json"), j).unwrap();

    Ok(json(&Response {
        success: true,
        message: "".to_string()
    }))
}

// POST
pub async fn delete_handler(_request: Diagram) -> Result<impl Reply> {
    write_json(String::from("data.json"), String::from("")).unwrap();

    Ok(json(&Response {
        success: true,
        message: "".to_string()
    }))
}

// GET
pub async fn list_handler() -> Result<impl Reply> {
    Ok(json(&read_json(String::from("data.json")).unwrap()))
}

// GET
pub async fn equipment_handler() -> Result<impl Reply> {               
    Ok(json(&read_equipment(String::from("equipment.json")).unwrap()))    
}

// GET
pub async fn command_handler() -> Result<impl Reply> {               
    Ok(json(&read_commands(String::from("command.json")).unwrap()))    
}

// GET
pub async fn design_handler(/*id: String*/) -> Result<impl Reply> {

    let list = read_json(String::from("data.json")).unwrap();

    return Ok(json(list.get(0).unwrap()));    
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
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let diagrams: Vec<Diagram> =
        serde_json::from_str(&contents).expect("JSON was not well-formatted");

    Ok(diagrams)
}

fn read_equipment(file_path: String) -> std::io::Result<Vec<Equipment>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let eqs: Vec<Equipment> =
        serde_json::from_str(&contents).expect("JSON was not well-formatted");

    Ok(eqs)
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
