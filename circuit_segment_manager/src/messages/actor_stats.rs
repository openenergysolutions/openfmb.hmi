#[derive(Clone, Debug)]
pub struct RequestActorStats;

#[derive(Clone, Debug)]
pub struct ActorStats {
    pub message_count: u32,
    pub persisted_message_count: Option<u32>,
}
