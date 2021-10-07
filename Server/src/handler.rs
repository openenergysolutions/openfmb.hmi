// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use super::hmi;
use crate::error::Error;
use crate::coordinator::StartProcessingMessages;
use futures::{FutureExt, StreamExt};
use hmi::hmi::HmiMsg;
use hmi::processor::ProcessorMsg;
use hmi::coordinator::{CoordinatorOptions, CoordinatorStatus};
use log::{error, info};
use riker::actors::*;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{http::StatusCode, reply::json, ws::Message, ws::WebSocket, Rejection, Reply};

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

#[derive(Debug, Clone)]
pub struct GenericControl {
    pub text: String,
    pub message: microgrid::generic_control::ControlType,
    pub mrid: String,
    pub profile_name: Option<String>,
    pub args: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DataValue {
    Bool(bool),
    Double(f64),
    String(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Topic {
    pub name: String,
    pub mrid: String,
    pub value: Option<DataValue>,
    pub action: Option<String>,
    pub args: Option<f64>,
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
    topics: Vec<Topic>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Equipment {
    pub mrid: String,
    pub name: String,
    #[serde(rename = "deviceType")]
    pub device_type: Option<String>,
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
    backgroundColor: Option<String>,
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
    pub profile: Option<String>,
    pub topic: Topic,
    pub session_id: Option<String>,
}

impl UpdateMessage {
    pub fn create(topic: Topic, session_id: String, profile: Option<String>) -> UpdateMessage {
        UpdateMessage {
            profile: profile,
            topic: topic,
            session_id: Some(session_id),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct UpdateMessages {
    updates: Vec<UpdateMessage>,
    pub session_id: Option<String>,
}

#[derive(Deserialize)]
pub struct DiagramQuery {
    id: String,
}

impl UpdateMessages {
    pub fn new(messages: Vec<UpdateMessage>, session_id: String) -> UpdateMessages {
        UpdateMessages {
            updates: messages,
            session_id: Some(session_id),
        }
    }
}

pub async fn data_handler(
    update: UpdateMessage,
    processor: ActorRef<ProcessorMsg>,
    hmi: ActorRef<HmiMsg>,
) -> Result<impl Reply> {
    info!("Handle data: {:?}", update);

    // This action is applied to all client sessions
    if update.topic.name == "ToggleEnvironment" {
        hmi.tell(
            StartProcessingMessages {
                pubsub_options: CoordinatorOptions::toggle_environment(),
                nats_client: None,
            },
            None,
        );
        return Ok(StatusCode::OK);
    }

    if let Ok(microgrid_control) =
        microgrid::microgrid_control::ControlMessage::from_str(&update.topic.name)
    {
        processor.tell(
            MicrogridControl {
                text: update.topic.name.clone(),
                message: microgrid_control,
            },
            None,
        );
    } else if let Ok(device_control) =
        microgrid::device_control::DeviceControlMessage::from_str(&update.topic.name)
    {
        processor.tell(
            DeviceControl {
                text: update.topic.name.clone(),
                message: device_control,
            },
            None,
        );
    } else if let Ok(generic_control) =
        microgrid::generic_control::ControlType::from_str(&update.topic.name)
    {
        processor.tell(
            GenericControl {
                text: update.topic.name.clone(),
                message: generic_control,
                mrid: update.topic.mrid.clone(),
                profile_name: None,
                args: update.topic.args.clone(),
            },
            None,
        );
    } else if let Some(action) = &update.topic.action {
        if let Ok(generic_control) = microgrid::generic_control::ControlType::from_str(action) {
            processor.tell(
                GenericControl {
                    text: update.topic.name.clone(),
                    message: generic_control,
                    mrid: update.topic.mrid.clone(),
                    profile_name: None,
                    args: update.topic.args.clone(),
                },
                None,
            );
        } else {
            info!("Received unknown action: {}", action);
        }
    } else {
        info!(
            "Received unknown command: {} (action={:?})",
            update.topic.name, update.topic.action
        );
    }

    Ok(StatusCode::OK)
}

pub fn get_profile_name(topic_name: &str) -> String {
    let token: Vec<&str> = topic_name.split(".").collect();
    token[0].to_string()
}

pub async fn send_updates(updates: UpdateMessages, clients: Clients) -> Result<impl Reply> {
    clients
        .read()
        .await
        .iter()
        .filter(|(_, client)| match &updates.session_id {
            Some(v) => client.session_id == *v,
            None => false,
        })
        .for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let json = serde_json::to_string(&updates).unwrap();
                let _ = sender.send(Ok(Message::text(json)));
            }
        });

    Ok(StatusCode::OK)
}

pub async fn send_status(status: CoordinatorStatus, clients: Clients) -> Result<impl Reply> {
    let hmi_pubsub_status_connected = "hmi.pubsub.status.connected".to_string();
    let hmi_pubsub_status_environment = "hmi.pubsub.status.environment".to_string();
    let hmi_coordinator_active = "hmi.coordinator.active".to_string();

    let updates = UpdateMessages {
        updates: vec![
            UpdateMessage {
                profile: None,
                session_id: None,
                topic: Topic {
                    name: hmi_pubsub_status_connected,
                    mrid: status.server_id.clone(),
                    value: Some(DataValue::Bool(status.connected)),
                    action: None,
                    args: None,
                },
            },
            UpdateMessage {
                profile: None,
                session_id: None,
                topic: Topic {
                    name: hmi_pubsub_status_environment,
                    mrid: status.server_id.clone(),
                    value: Some(DataValue::Double((status.env as u8) as f64)),
                    action: None,
                    args: None,
                },
            },
            UpdateMessage {
                profile: None,
                session_id: None,
                topic: Topic {
                    name: hmi_coordinator_active,
                    mrid: status.server_id.clone(),
                    value: Some(DataValue::Bool(status.coordinator_active)),
                    action: None,
                    args: None,
                },
            },
        ],
        session_id: None,
    };

    clients.read().await.iter().for_each(|(_, client)| {
        if let Some(sender) = &client.sender {
            let json = serde_json::to_string(&updates).unwrap();
            let _ = sender.send(Ok(Message::text(json)));
        }
    });

    Ok(StatusCode::OK)
}

pub async fn send_inspector_messages(
    messages: &Vec<String>,
    clients: &Clients,
) -> Result<impl Reply> {
    clients.read().await.iter().for_each(|(_, client)| {
        if let Some(sender) = &client.sender {
            for json in messages {
                let _ = sender.send(Ok(Message::text(json)));
            }
        }
    });

    Ok(StatusCode::OK)
}

fn get_diagram_folder() -> String {
    let app_dir = std::env::var("APP_DIR_NAME").unwrap_or_else(|_| "".into());
    let mut diagrams_dir = "diagrams".to_string();
    if app_dir != "" {
        diagrams_dir = format!("/{}/diagrams", app_dir);
    }

    if !Path::new(&diagrams_dir).exists() {
        match fs::create_dir(&diagrams_dir) {
            Ok(_) => {}
            Err(e) => {
                error!("{}", e);
            }
        }
    }
    diagrams_dir
}

// POST
pub async fn save_handler(request: Diagram) -> Result<impl Reply> {
    let j = serde_json::to_string(&request).unwrap();

    write_json(
        format!("{}/{}.json", get_diagram_folder(), request.diagramId),
        j,
    )
    .unwrap();

    Ok(json(&Response {
        success: true,
        message: "".to_string(),
    }))
}

// POST
pub async fn delete_handler(request: Diagram) -> Result<impl Reply> {
    let _ = fs::remove_file(format!(
        "{}/{}.json",
        get_diagram_folder(),
        request.diagramId
    ));
    Ok(json(&Response {
        success: true,
        message: "".to_string(),
    }))
}

// GET
pub async fn list_handler() -> Result<impl Reply> {
    Ok(json(
        &read_json(format!("{}", get_diagram_folder())).unwrap(),
    ))
}

// GET
pub async fn equipment_handler(_id: String) -> Result<impl Reply> {
    let equipment_list: Vec<Equipment> = read_equipment_list().unwrap();
    Ok(json(&equipment_list))
}

// POST
pub async fn create_equipment_handler(_id: String, eq: Equipment) -> Result<impl Reply> {
    let mut list = read_equipment_list().unwrap();
    if let Some(_pos) = list.iter().position(|x| *x.mrid == eq.mrid) {
        // same user id/username already exists
        error!(
            "Equipment with same MRID ({}/{}) already exists",
            eq.mrid, eq.name
        );

        return Err(warp::reject::custom(Error::AddDeviceError));
    } else {
        list.push(eq);
        let _ = save_equipment_list(&list);
    }

    Ok(json(&list))
}

// POST
pub async fn delete_equipment_handler(_id: String, equipment: Equipment) -> Result<impl Reply> {
    let mut list = read_equipment_list().unwrap();

    if let Some(pos) = list.iter().position(|x| *x.mrid == equipment.mrid) {
        list.remove(pos);

        let _ = save_equipment_list(&list);
    }

    Ok(json(&list))
}

// POST
pub async fn update_equipment_handler(_id: String, eq: Equipment) -> Result<impl Reply> {
    let mut list = read_equipment_list().unwrap();

    if let Some(pos) = list.iter().position(|x| *x.mrid == eq.mrid) {
        let mut e = list.get_mut(pos).unwrap();
        e.name = eq.name;
        e.mrid = eq.mrid;
        e.device_type = eq.device_type;

        let _ = save_equipment_list(&list);
    }

    Ok(json(&list))
}

fn get_equipment_file() -> String {
    let app_dir = std::env::var("APP_DIR_NAME").unwrap_or_else(|_| "".into());
    if app_dir != "" {
        return format!("/{}/equipment.json", app_dir);
    }
    "equipment.json".to_string()
}

pub fn read_equipment_list() -> std::io::Result<Vec<Equipment>> {
    let file_path = &get_equipment_file();
    let equipment_list: Vec<Equipment> = vec![];

    if let Ok(mut file) = File::open(file_path.clone()) {
        let mut contents = String::new();
        if let Ok(_) = file.read_to_string(&mut contents) {
            match serde_json::from_str(&contents) {
                Ok(equipment_list) => {
                    return Ok(equipment_list);
                }
                Err(e) => {
                    error!("Unable to parse equipment file: {} [{}]", file_path, e);
                }
            }
        } else {
            error!("Unable to read equipment file: {}", file_path);
        }
    } else {
        error!("Unable to open equipment file: {}", file_path);
    }

    Ok(equipment_list)
}

fn save_equipment_list(equipment_list: &Vec<Equipment>) -> std::io::Result<()> {
    let json = serde_json::to_string(&equipment_list).unwrap();
    fs::write(get_equipment_file(), json).expect("Unable to write file");

    Ok(())
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

pub async fn connect_handler(ws: warp::ws::Ws, id: String, clients: Clients) -> Result<impl Reply> {
    return Ok(ws.on_upgrade(move |socket| client_connection(socket, id, clients)));
}

pub async fn client_connection(ws: WebSocket, id: String, clients: Clients) {
    let mut client = match clients.read().await.get(&id).cloned() {
        Some(c) => c,
        None => Client {
            session_id: id.clone(),
            sender: None,
            topics: vec![],
        },
    };

    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);
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
                eprintln!(
                    "ERROR::Error receiving message for client id: {}): {}",
                    id.clone(),
                    e
                );
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
        }
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
    Ok(diagrams)
}

fn write_json(file_path: String, json: String) -> std::io::Result<()> {
    fs::write(file_path, json).expect("Unable to write file");

    Ok(())
}
