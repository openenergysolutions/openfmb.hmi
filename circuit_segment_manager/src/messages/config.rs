use config::Config;

#[derive(Clone, Debug)]
pub struct ActorConfig(Box<Config>);

impl Default for ActorConfig {
    fn default() -> Self {
        unimplemented!()
    }
}
