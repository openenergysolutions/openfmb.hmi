use crate::actors::Persistor;
use crate::{
    actors::{
        CoordinatorMsg, Device, DeviceMsg, GenericOpenFMBProfile, GenericOpenFMBProfileMsg,
        PersistorMsg, Publisher, PublisherMsg,
    },
    messages::*,
};
use riker::actors::*;
use std::collections::HashMap;

#[actor(RequestActorStats, OpenFMBMessage)]
#[derive(Clone, Debug)]
pub struct OpenFMBDevice {
    message_count: u32,
    openfmb_profile_actors: HashMap<OpenFMBProfileType, ActorRef<GenericOpenFMBProfileMsg>>,
    publisher: ActorRef<PublisherMsg>,
    persistor: ActorRef<PersistorMsg>,
}

impl ActorFactoryArgs<(ActorRef<PublisherMsg>, ActorRef<PersistorMsg>)> for OpenFMBDevice {
    fn create_args(args: (ActorRef<PublisherMsg>, ActorRef<PersistorMsg>)) -> Self {
        OpenFMBDevice {
            message_count: 0,
            openfmb_profile_actors: Default::default(),
            publisher: args.0,
            persistor: args.1,
        }
    }
}

impl OpenFMBDevice {
    fn ensure_actor(
        &mut self,
        ctx: &Context<OpenFMBDeviceMsg>,
        msg: OpenFMBMessage,
    ) -> ActorRef<GenericOpenFMBProfileMsg> {
        use OpenFMBMessage::*;
        match msg {
            GenerationReading(_msg) => self.ensure_generator_actor(ctx),
            GenerationStatus(_msg) => self.ensure_generator_actor(ctx),
            SwitchStatus(_msg) => self.ensure_switch_actor(ctx),
            SwitchReading(_msg) => self.ensure_switch_actor(ctx),
            MeterReading(_msg) => self.ensure_meter_actor(ctx),
            SolarReading(_msg) => self.ensure_solar_actor(ctx),
            SolarStatus(_msg) => self.ensure_solar_actor(ctx),
            ESSReading(_msg) => self.ensure_ess_actor(ctx),
            ESSStatus(_msg) => self.ensure_ess_actor(ctx),
            LoadStatus(_msg) => self.ensure_load_actor(ctx),
            LoadReading(_msg) => self.ensure_load_actor(ctx),
            ShuntStatus(_msg) => self.ensure_shunt_actor(ctx),
            ShuntReading(_msg) => self.ensure_shunt_actor(ctx),
            RecloserStatus(_msg) => self.ensure_recloser_actor(ctx),
            RecloserReading(_msg) => self.ensure_recloser_actor(ctx),
            BreakerStatus(_msg) => self.ensure_breaker_actor(ctx),
            BreakerReading(_msg) => self.ensure_breaker_actor(ctx),
            RegulatorStatus(_msg) => self.ensure_regulator_actor(ctx),
            RegulatorReading(_msg) => self.ensure_regulator_actor(ctx),
            LoadControl(_msg) => self.ensure_load_actor(ctx),
            SwitchControl(_msg) => self.ensure_switch_actor(ctx),
            EssControl(_msg) => self.ensure_ess_actor(ctx),
            SolarControl(_msg) => self.ensure_solar_actor(ctx),
            GeneratorControl(_msg) => self.ensure_generator_actor(ctx),
            BreakerControl(_msg) => self.ensure_generator_actor(ctx),
            ResourceStatus(_msg) => self.ensure_resource_actor(ctx),
            //                        _ => panic!("unsupported message profile: {:?}", msg),
        }
    }

    //    fn ensure_actor(&mut self, ctx: &Context<OpenFMBDeviceMsg>, msg: OpenFMBMessage) -> ActorRef<GenericOpenFMBProfileMsg> {
    //        let profile_type = OpenFMBDevice::get_profile_type(parts.profile.clone());
    //        match profile_type {
    //            OpenFMBProfileType::Generation => self.ensure_generator_actor(ctx),
    //            OpenFMBProfileType::Switch => self.ensure_switch_actor(ctx),
    //            OpenFMBProfileType::Meter => self.ensure_meter_actor(ctx),
    //            OpenFMBProfileType::Solar => self.ensure_solar_actor(ctx),
    //            OpenFMBProfileType::ESS => self.ensure_ess_actor(ctx),
    //            OpenFMBProfileType::Load => self.ensure_load_actor(ctx),
    //            _ => panic!("unsupported message profile: {}", parts.profile),
    //        }
    //    }

    fn ensure_regulator_actor(
        &mut self,
        ctx: &Context<OpenFMBDeviceMsg>,
    ) -> ActorRef<GenericOpenFMBProfileMsg> {
        let actor = self
            .openfmb_profile_actors
            .get(&OpenFMBProfileType::Regulator);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let generation_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfile,(ActorRef<PublisherMsg>,ActorRef<PersistorMsg>)>(
                        "RegulatorHandler",(self.publisher.clone(), self.persistor.clone()),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::Regulator, generation_actor_ref.clone());
                generation_actor_ref
            }
        }
    }

    fn ensure_breaker_actor(
        &mut self,
        ctx: &Context<OpenFMBDeviceMsg>,
    ) -> ActorRef<GenericOpenFMBProfileMsg> {
        let actor = self
            .openfmb_profile_actors
            .get(&OpenFMBProfileType::Breaker);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let generation_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfile,(ActorRef<PublisherMsg>,ActorRef<PersistorMsg>)>(
                        "BreakerHandler",(self.publisher.clone(), self.persistor.clone()),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::Breaker, generation_actor_ref.clone());
                generation_actor_ref
            }
        }
    }

    fn ensure_recloser_actor(
        &mut self,
        ctx: &Context<OpenFMBDeviceMsg>,
    ) -> ActorRef<GenericOpenFMBProfileMsg> {
        let actor = self
            .openfmb_profile_actors
            .get(&OpenFMBProfileType::Recloser);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let generation_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfile,(ActorRef<PublisherMsg>,ActorRef<PersistorMsg>)>(
                        "RecloserHandler",(self.publisher.clone(), self.persistor.clone()),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::Recloser, generation_actor_ref.clone());
                generation_actor_ref
            }
        }
    }

    fn ensure_load_actor(
        &mut self,
        ctx: &Context<OpenFMBDeviceMsg>,
    ) -> ActorRef<GenericOpenFMBProfileMsg> {
        let actor = self.openfmb_profile_actors.get(&OpenFMBProfileType::Load);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let generation_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfile,(ActorRef<PublisherMsg>,ActorRef<PersistorMsg>)>(
                        "LoadHandler",(self.publisher.clone(), self.persistor.clone()),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::Load, generation_actor_ref.clone());
                generation_actor_ref
            }
        }
    }

    fn ensure_shunt_actor(
        &mut self,
        ctx: &Context<OpenFMBDeviceMsg>,
    ) -> ActorRef<GenericOpenFMBProfileMsg> {
        let actor = self.openfmb_profile_actors.get(&OpenFMBProfileType::Shunt);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let generation_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfile,(ActorRef<PublisherMsg>,ActorRef<PersistorMsg>)>(
                        "ShuntHandler",(self.publisher.clone(), self.persistor.clone()),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::Shunt, generation_actor_ref.clone());
                generation_actor_ref
            }
        }
    }

    fn ensure_ess_actor(
        &mut self,
        ctx: &Context<OpenFMBDeviceMsg>,
    ) -> ActorRef<GenericOpenFMBProfileMsg> {
        let actor = self.openfmb_profile_actors.get(&OpenFMBProfileType::ESS);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let generation_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfile,(ActorRef<PublisherMsg>,ActorRef<PersistorMsg>)>(
                        "ESSHandler",(self.publisher.clone(), self.persistor.clone()),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::ESS, generation_actor_ref.clone());
                generation_actor_ref
            }
        }
    }

    fn ensure_solar_actor(
        &mut self,
        ctx: &Context<OpenFMBDeviceMsg>,
    ) -> ActorRef<GenericOpenFMBProfileMsg> {
        let actor = self.openfmb_profile_actors.get(&OpenFMBProfileType::Solar);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let generation_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfile,(ActorRef<PublisherMsg>,ActorRef<PersistorMsg>)>(
                        "SolarHandler",(self.publisher.clone(), self.persistor.clone()),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::Solar, generation_actor_ref.clone());
                generation_actor_ref
            }
        }
    }

    fn ensure_meter_actor(
        &mut self,
        ctx: &Context<OpenFMBDeviceMsg>,
    ) -> ActorRef<GenericOpenFMBProfileMsg> {
        let actor = self.openfmb_profile_actors.get(&OpenFMBProfileType::Meter);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let generation_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfile,(ActorRef<PublisherMsg>,ActorRef<PersistorMsg>)>(
                        "MeterHandler",(self.publisher.clone(), self.persistor.clone()),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::Meter, generation_actor_ref.clone());
                generation_actor_ref
            }
        }
    }

    fn ensure_switch_actor(
        &mut self,
        ctx: &Context<OpenFMBDeviceMsg>,
    ) -> ActorRef<GenericOpenFMBProfileMsg> {
        let actor = self.openfmb_profile_actors.get(&OpenFMBProfileType::Switch);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let generation_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfile,(ActorRef<PublisherMsg>,ActorRef<PersistorMsg>)>(
                        "SwitchHandler",(self.publisher.clone(), self.persistor.clone()),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::Switch, generation_actor_ref.clone());
                generation_actor_ref
            }
        }
    }

    fn ensure_generator_actor(
        &mut self,
        ctx: &Context<OpenFMBDeviceMsg>,
    ) -> ActorRef<GenericOpenFMBProfileMsg> {
        let actor = self
            .openfmb_profile_actors
            .get(&OpenFMBProfileType::Generation);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let generation_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfile,(ActorRef<PublisherMsg>,ActorRef<PersistorMsg>)>(
                        "GenerationHandler",(self.publisher.clone(), self.persistor.clone()),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::Generation, generation_actor_ref.clone());
                generation_actor_ref
            }
        }
    }

    fn ensure_resource_actor(
        &mut self,
        ctx: &Context<OpenFMBDeviceMsg>,
    ) -> ActorRef<GenericOpenFMBProfileMsg> {
        let actor = self
            .openfmb_profile_actors
            .get(&OpenFMBProfileType::Resource);
        match actor {
            Some(_) => actor.unwrap().clone(),
            None => {
                let resource_actor_ref = ctx
                    .actor_of_args::<GenericOpenFMBProfile,(ActorRef<PublisherMsg>,ActorRef<PersistorMsg>)>(
                        "ResourceHandler",(self.publisher.clone(), self.persistor.clone()),
                    )
                    .unwrap();
                self.openfmb_profile_actors
                    .insert(OpenFMBProfileType::Resource, resource_actor_ref.clone());
                resource_actor_ref
            }
        }
    }
}

impl Actor for OpenFMBDevice {
    type Msg = OpenFMBDeviceMsg;

    fn pre_start(&mut self, _ctx: &Context<Self::Msg>) {
        //        let subscribers = vec![ctx
        //            .actor_of(Props::new(RustCodeProcessor::actor), "OpenFMBFileSubscriber")
        //            .unwrap()];
        //        let publisher = vec![ctx
        //            .actor_of(Props::new(RustCodeProcessor::actor), "RustCodeProcessor")
        //            .unwrap()];
    }

    fn post_start(&mut self, _ctx: &Context<Self::Msg>) {}

    fn post_stop(&mut self) {}

    fn supervisor_strategy(&self) -> Strategy {
        Strategy::Restart
    }

    fn sys_recv(
        &mut self,
        _ctx: &Context<Self::Msg>,
        _msg: SystemMsg,
        _sender: Option<BasicActorRef>,
    ) {
    }

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) {
        self.message_count += 1;
        self.receive(ctx, msg, sender);
    }
}

//impl Receive<RequestActorStats> for RustCodeProcessor {
//    type Msg = OpenFMBDeviceMsg;
//    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: RequestActorStats, sender: Sender) {
//        //        self.rust_processor
//        //            .unwrap()
//        //            .send_msg(msg.clone().into(), sender.clone());
//        let stats_msg: CoordinatorMsg = ActorStats {
//            message_count: self.message_count,
//        }
//        .into();
//        sender
//            .unwrap()
//            .try_tell(stats_msg, Some(ctx.myself.clone().into()))
//            .unwrap();
//    }
//}

impl Receive<RequestActorStats> for OpenFMBDevice {
    type Msg = OpenFMBDeviceMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: RequestActorStats, sender: Sender) {
        let stats_msg: CoordinatorMsg = ActorStats {
            message_count: self.message_count,
            persisted_message_count: None,
        }
        .into();
        sender
            .clone()
            .unwrap()
            .try_tell(stats_msg, Some(ctx.myself.clone().into()))
            .unwrap();
        for child in self.openfmb_profile_actors.clone() {
            child.1.send_msg(msg.clone().into(), sender.clone());
        }
        //        match &self.openfmb_file_subscriber {
        //            Some(child) => child.tell(RequestActorStats, sender.clone()),
        //            None => {}
        //        }
        let _stats_msg: CoordinatorMsg = ActorStats {
            message_count: self.message_count,
            persisted_message_count: None,
        }
        .into();
        //sender.clone().unwrap().try_tell(stats_msg, sender.clone());
        //        self.publisher.clone().send_msg(msg.into(), ctx.myself.clone())
    }
}

//impl Receive<Add> for RustCodeProcessor {
//    type Msg = OpenFMBDeviceMsg;
//
//    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: Add, _sender: Sender) {
//        self.message_count += 1;
//    }
//}
//
//impl Receive<Sub> for RustCodeProcessor {
//    type Msg = OpenFMBDeviceMsg;
//
//    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: Sub, _sender: Sender) {
//        self.message_count -= 1;
//    }
//}
//
//impl Receive<Print> for RustCodeProcessor {
//    type Msg = OpenFMBDeviceMsg;
//
//    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: Print, _sender: Sender) {
//        info!("Total counter value: {}", self.message_count);
//    }
//}

impl Receive<OpenFMBMessage> for OpenFMBDevice {
    type Msg = OpenFMBDeviceMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: OpenFMBMessage, _sender: Sender) {
        let actor = self.ensure_actor(ctx, msg.clone());
        actor.send_msg(msg.into(), ctx.myself.clone());

        //        self.openfmb_device.clone().unwrap().send_msg(msg.into(), ctx.myself.clone())
    }
}
