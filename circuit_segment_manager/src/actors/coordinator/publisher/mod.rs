pub mod debug_stdout_publisher;
//pub mod elasticsearch_publisher;
pub mod openfmb_file_publisher;
pub mod openfmb_nats_publisher;
pub mod publisher;
pub use publisher::*;

pub mod gui_publisher;
