use circuit_segment_manager::actors::coordinator::openfmb::openfmb::OpenFMBDeviceMsg;
use circuit_segment_manager::messages::*;

use super::hmi_publisher::HmiPublisherMsg;
use crate::handler::*;

use riker::actors::*;
use uuid::Uuid;

use openfmb_messages_ext::{
    OpenFMBExt, SwitchReadingExt
};

#[actor(OpenFMBMessage, PublisherRefWrap, MicrogridControl, DeviceControl)]
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

async fn handle_openfmb_message(clients: &Clients, msg: OpenFMBMessage) {    

    let mut update_messages = vec![];

    clients
        .read()
        .await
        .iter()        
        .for_each(|(_, client)| {
            println!("Session ID: {:?}", client.session_id); 
            

            for topic in &client.topics {
                
                let topic_mrid = Uuid::parse_str(&topic.mrid).unwrap();
                if topic_mrid == msg.device_mrid().unwrap() {
                    //println!("{:?}", topic);

                    match &msg {
                        OpenFMBMessage::SwitchReading(message) => {
                            if topic.name.contains(".switchReading[0].readingMMXU.A.phsA.cVal.mag") {
                                let current = message.as_ref().get_current_phsa();
                                let mut update_msg = UpdateMessage::create(topic.clone(), client.session_id.clone());
                                update_msg.topic.value = Some(current);
                                update_messages.push(update_msg);                                
                            }
                            else if topic.name.contains(".switchReading[0].readingMMXU.A.phsB.cVal.mag") {
                                let current = message.as_ref().get_current_phsb();
                                let mut update_msg = UpdateMessage::create(topic.clone(), client.session_id.clone());
                                update_msg.topic.value = Some(current);
                                update_messages.push(update_msg);                                
                            }
                            else if topic.name.contains(".switchReading[0].readingMMXU.A.phsC.cVal.mag") {
                                let current = message.as_ref().get_current_phsc();
                                let mut update_msg = UpdateMessage::create(topic.clone(), client.session_id.clone());
                                update_msg.topic.value = Some(current);
                                update_messages.push(update_msg);                                
                            }
                            else if topic.name.contains(".switchReading[0].readingMMXU.Hz.mag") {
                                let current = message.as_ref().get_freq(0);
                                let mut update_msg = UpdateMessage::create(topic.clone(), client.session_id.clone());
                                update_msg.topic.value = Some(current);
                                update_messages.push(update_msg);
                            }                            
                        },
                        OpenFMBMessage::SwitchStatus(message) => {
                            if topic.name.ends_with(".Pos.phs3.stVal") {
                                if let Ok(state) = message.as_ref().device_state() {
                                    let mut s : f64 = 0.0;
                                    if state == "Closed" {
                                        s = 1.0;
                                    }

                                    let mut update_msg = UpdateMessage::create(topic.clone(), client.session_id.clone());
                                    update_msg.topic.value = Some(s);
                                    update_messages.push(update_msg);
                                }
                                println!("Topic: path={:?}, state={:?}", topic.name, message.as_ref().device_state());
                                
                            }
                        },
                        _ => {
                            println!("Message type is not yet supported: {:?}", msg.message_type());
                        }
                    }
                }                
            }                         
        });

    if update_messages.len() > 0 {
        let _ = send_updates(UpdateMessages::new(update_messages), clients.clone()).await;
    }
}

