use riker::actor::ActorRef;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct ActorRefWrap<T: 'static + Send + Clone + Debug>(pub ActorRef<T>);

#[derive(Clone, Debug)]
pub struct GetChildActors;
