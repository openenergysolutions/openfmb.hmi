#![allow(unused_imports)]

//#![deny(missing_debug_implementations)]

// Include the `items` module, which is generated from items.proto.
// pub mod items {
//     include!(concat!(env!("OUT_DIR"), "/snazzy.items.rs"));
// }

pub fn foo() {
    //    let foo = microgrid::microgrid_control::Cmd::ResetDevices;
}
// pub fn create_large_shirt(color: String) -> items::Shirt {
//     let mut shirt = items::Shirt::default();
//     shirt.color = color;
//     shirt.set_size(items::shirt::Size::Large);
//     shirt
// }

//pub mod myactor;
pub mod actors;
pub mod messages;
mod traits;
pub mod util;
