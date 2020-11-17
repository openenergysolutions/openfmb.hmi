use crate::traits::OpenFMBProfileType;

#[derive(Clone, Debug)]
pub struct PartsMessage {
    pub msg_type: OpenFMBProfileType,
    pub number: String,
    pub profile: String,
    pub payload: String,
}
