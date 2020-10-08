use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tokio::sync::{mpsc, RwLock};
use warp::{http::StatusCode, reply::json, ws::Message, ws::WebSocket, Reply, Rejection};
use futures::{FutureExt, StreamExt};
use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Rejection>;
pub type Clients = Arc<RwLock<HashMap<String, Client>>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Topic {
    name: String,
    mrid: String,
    value: Option<f32>
}

impl Topic {
    fn to_string(&self) -> String {
        format!("{}.{}", self.mrid, self.name)
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    pub session_id: String,
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    session_id: Option<String>,
    topics: Vec<Topic>
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Diagram {
    diagramId: String,
    name: String,
    description: String,
    location: String,
    data: String,
    createdDate: String,
    createdBy: String,
    backgroundColor: String
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
    topic: Topic,
    session_id: Option<String>
}

#[derive(Serialize, Debug)]
pub struct UpdateMessages {
    updates: Vec<UpdateMessage>
}

pub async fn data_handler(update: UpdateMessage, clients: Clients) -> Result<impl Reply> {
    println!("Handle data: {:?}", update);
    clients
        .read()
        .await
        .iter()
        .filter(|(_, client)| match update.session_id.clone() {
            Some(v) => client.session_id == v,
            None => true,
        })
        .filter(|(_, client)| client.topics.contains(&update.topic.to_string()))
        .for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let _rs = UpdateMessages {
                    updates: vec![update.clone()]
                };
                let json = serde_json::to_string(&_rs).unwrap();
                let _ = sender.send(Ok(Message::text(json)));
            }
        });

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

// GET
pub async fn list_handler() -> Result<impl Reply> {
    Ok(json(&read_json(String::from("data.json")).unwrap()))
}

// GET
pub async fn design_handler(/*id: String*/) -> Result<impl Reply> {

    let list = read_json(String::from("data.json")).unwrap();

    return Ok(json(list.get(0).unwrap()));

    // for diagram in list.iter() {
    //     if diagram.diagramId == "1710c39f-369f-42af-8f7d-9668db790f0f" {
    //         return Ok(json(diagram));
    //     }
    // }

    // Ok(json(&Response {
    //     success: false,
    //     message: "ID not found".to_string()
    // }))
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
    println!("Received message from {}: {:?}", id, msg);
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    let register_request: RegisterRequest = match from_str(&message) {
        Ok(v) => v,
        Err(_) => {
            println!("Only can handle RegisterRequest at this moment.  Ignore message!");
            return;
        }
    };

    let mut locked = clients.write().await;
    match locked.get_mut(id) {
        Some(v) => {
            let mut topics = vec![];
            for t in register_request.topics.iter() {
                topics.push(t.to_string());
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

fn write_json(file_path: String, json: String) -> std::io::Result<()> {

    fs::write(file_path, json).expect("Unable to write file");

    Ok(())
}
