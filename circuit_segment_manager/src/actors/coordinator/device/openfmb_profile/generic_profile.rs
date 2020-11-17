use crate::{
    actors::{
        Coordinator, CoordinatorMsg, Device, DeviceMsg, GenericOpenFMBDevice,
        GenericOpenFMBDeviceMsg, PersistorMsg, PublisherMsg,
    },
    messages::{ActorStats, OpenFMBError, OpenFMBMessage, RequestActorStats},
};

// use prost::Message;
use crate::actors::{Persistor, Publisher};
use riker::actors::*;
use std::collections::HashMap;
use uuid::Uuid;

#[actor(OpenFMBMessage, RequestActorStats)]
#[derive(Debug)]
pub struct GenericOpenFMBProfile {
    pub message_count: u32,
    pub devices: HashMap<Uuid, ActorRef<GenericOpenFMBDeviceMsg>>,
    pub publisher: ActorRef<PublisherMsg>,
    pub persistor: ActorRef<PersistorMsg>,
}

impl ActorFactoryArgs<(ActorRef<PublisherMsg>, ActorRef<PersistorMsg>)> for GenericOpenFMBProfile {
    fn create_args(args: (ActorRef<PublisherMsg>, ActorRef<PersistorMsg>)) -> Self {
        GenericOpenFMBProfile {
            message_count: 0,
            devices: Default::default(),
            publisher: args.0,
            persistor: args.1,
        }
    }
}

impl GenericOpenFMBProfile {
    fn ensure_actor(
        &mut self,
        ctx: &Context<GenericOpenFMBProfileMsg>,
        msg: &OpenFMBMessage,
    ) -> Result<ActorRef<GenericOpenFMBDeviceMsg>, OpenFMBError> {
        let device_mrid = msg.device_mrid()?;

        match self.devices.get(&device_mrid) {
            Some(actor_ref) => Ok(actor_ref.clone()),
            None => {
                let generation_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBDevice,(ActorRef<PublisherMsg>,ActorRef<PersistorMsg>)>(
                        &device_mrid.to_string(),(self.publisher.clone(), self.persistor.clone()),
                    )
                    .unwrap();

                self.devices
                    .insert(device_mrid, generation_actor_ref.clone());
                Ok(generation_actor_ref)
            }
        }
    }

    //    pub(crate) fn get_profile_type(profile: String) -> OpenFMBProfileType {
    //        match profile.as_str() {
    //            "GenerationStatusProfile" | "GenerationReadingProfile" => OpenFMBProfileType::Generation,
    //            "SwitchReadingProfile" | "SwitchStatusProfile" => OpenFMBProfileType::Switch,
    //            "MeterReadingProfile" => OpenFMBProfileType::Meter,
    //            "SolarReadingProfile" | "SolarStatusProfile" => OpenFMBProfileType::Solar,
    //            "ESSReadingProfile" | "ESSStatusProfile" => OpenFMBProfileType::ESS,
    //            "LoadReadingProfile" | "LoadStatusProfile" => OpenFMBProfileType::Load,
    //            "ShuntReadingProfile" | "ShuntStatusProfile" => OpenFMBProfileType::Shunt,
    //            "RecloserReadingProfile" | "RecloserStatusProfile" => OpenFMBProfileType::Recloser,
    //            "BreakerReadingProfile" | "BreakerStatusProfile" => OpenFMBProfileType::Breaker,
    //            "RegulatorReadingProfile" | "RegulatorStatusProfile" => OpenFMBProfileType::Regulator,
    //            _ => panic!("unsupported message profile: {}", profile),
    //        }
    //    }
}

impl Actor for GenericOpenFMBProfile {
    type Msg = GenericOpenFMBProfileMsg;

    fn pre_start(&mut self, _ctx: &Context<Self::Msg>) {}

    fn post_start(&mut self, _ctx: &Context<Self::Msg>) {}

    fn post_stop(&mut self) {}

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) {
        self.message_count += 1;
        self.receive(ctx, msg, sender);
    }
}

impl GenericOpenFMBProfile {}

impl Receive<RequestActorStats> for GenericOpenFMBProfile {
    type Msg = GenericOpenFMBProfileMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: RequestActorStats, sender: Sender) {
        for (_mrid, device) in self.devices.clone() {
            device.send_msg(msg.clone().into(), sender.clone());
        }
        let stats_msg: CoordinatorMsg = ActorStats {
            message_count: self.message_count,
            persisted_message_count: None,
        }
        .into();
        sender
            .unwrap()
            .try_tell(stats_msg, Some(ctx.myself.clone().into()))
            .unwrap();
    }
}

impl Receive<OpenFMBMessage> for GenericOpenFMBProfile {
    type Msg = GenericOpenFMBProfileMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: OpenFMBMessage, _sender: Sender) {
        let child_actor = self.ensure_actor(ctx, &msg).unwrap();
        child_actor.send_msg(msg.into(), Some(ctx.myself.clone().into()))
        //        self.openfmb_device.clone().unwrap().send_msg(msg.into(), ctx.myself.clone())
    }
}

// impl Receive<GenerationReading> for GenericOpenFMBProfile {
//     type Msg = GenericOpenFMBProfileMsg;
//     fn receive(&mut self, ctx: &Context<Self::Msg>, msg: GenerationReading, sender: Sender) {
//         let child_actor = self.ensure_actor(ctx, &msg).unwrap();
//         child_actor.send_msg(msg.into(), Some(ctx.myself.clone().into()))
//         //        self.openfmb_device.clone().unwrap().send_msg(msg.into(), ctx.myself.clone())
//     }
// }
