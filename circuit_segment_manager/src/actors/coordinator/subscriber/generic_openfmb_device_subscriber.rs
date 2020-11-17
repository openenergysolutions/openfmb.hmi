use crate::{
    actors::{CoordinatorMsg, DeviceMsg},
    messages::{ActorStats, OpenFMBMessage, RequestActorStats},
};

use riker::actors::*;
use uuid::Uuid;

#[actor(OpenFMBMessage, RequestActorStats)]
#[derive(Debug)]
pub struct GenericOpenFMBDevicePersistorSubscriber {
    pub message_count: u32,
    pub mrid: Option<Uuid>,
    pub device_handler: ActorRef<DeviceMsg>,
}

impl ActorFactoryArgs<ActorRef<DeviceMsg>> for GenericOpenFMBDevicePersistorSubscriber {
    fn create_args(args: ActorRef<DeviceMsg>) -> Self {
        GenericOpenFMBDevicePersistorSubscriber {
            message_count: 0,
            mrid: None,
            device_handler: args,
        }
    }
}

impl Actor for GenericOpenFMBDevicePersistorSubscriber {
    type Msg = GenericOpenFMBDevicePersistorSubscriberMsg;

    fn pre_start(&mut self, _ctx: &Context<Self::Msg>) {
        //        self.device.send_msg)
    }

    //    fn post_start(&mut self, ctx: &Context<Self::Msg>) { unimplemented!() }
    //
    //    fn post_stop(&mut self) { unimplemented!() }
    //
    //    fn supervisor_strategy(&self) -> Strategy { unimplemented!() }

    fn sys_recv(
        &mut self,
        _ctx: &Context<Self::Msg>,
        _msg: SystemMsg,
        _sender: Option<BasicActorRef>,
    ) {
        unimplemented!()
    }

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) {
        self.message_count += 1;
        self.receive(ctx, msg, sender)
    }
}

impl Receive<RequestActorStats> for GenericOpenFMBDevicePersistorSubscriber {
    type Msg = GenericOpenFMBDevicePersistorSubscriberMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, _msg: RequestActorStats, sender: Sender) {
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

//impl Receive<PartsMessage> for GenericOpenFMBDevicePersistorSubscriber {
//    type Msg = GenericOpenFMBDevicePersistorSubscriberMsg;
//
//    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: PartsMessage, sender: Sender) {
//        self.processor.send_msg(msg.into(), ctx.myself.clone())
//        //        let foo = match msg.profile.as_ref() {
//        //            "GenerationStatusProfile" => {
//        //                openfmb::generationmodule::GenerationStatusProfile::decode(msg.payload).unwrap()
//        //            }
//        //            "GenerationReadingProfile" => {
//        //                openfmb::generationmodule::GenerationReadingProfile::decode(msg.payload).unwrap()
//        //            }
//        //
//        //            "MeterStatusProfile" => openfmb::metermodule::MeterReadingProfile::decode(msg.payload).unwrap(),
//        //
//        //            "SolarStatusProfile" => openfmb::solarmodule::SolarStatusProfile::decode(msg.payload).unwrap(),
//        //            "SolarReadingProfile" => openfmb::solarmodule::SolarReadingProfile::decode(msg.payload).unwrap(),
//        //
//        //            "ESSStatusProfile" => openfmb::essmodule::EssStatusProfile::decode(msg.payload).unwrap(),
//        //            "ESSReadingProfile" => openfmb::essmodule::EssReadingProfile::decode(msg.payload).unwrap(),
//        //
//        //            "SwitchStatusProfile" => openfmb::switchmodule::SwitchStatusProfile::decode(msg.payload).unwrap(),
//        //            "SwitchReadingProfile" => openfmb::switchmodule::SwitchReadingProfile::decode(msg.payload).unwrap(),
//        //
//        //            "LoadStatusProfile" => openfmb::loadmodule::LoadStatusProfile::decode(msg.payload).unwrap(),
//        //            "LoadReadingProfile" => openfmb::loadmodule::LoadReadingProfile::decode(msg.payload).unwrap(),
//        //        };
//    }
//}

impl Receive<OpenFMBMessage> for GenericOpenFMBDevicePersistorSubscriber {
    type Msg = GenericOpenFMBDevicePersistorSubscriberMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: OpenFMBMessage, _sender: Sender) {
        self.device_handler.send_msg(msg.into(), ctx.myself.clone());
    }
}
