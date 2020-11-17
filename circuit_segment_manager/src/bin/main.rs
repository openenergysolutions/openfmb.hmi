use circuit_segment_manager::util::init::{microgrid_setup, Error};
use std::thread::sleep;

// extern crate termion;

fn main() -> Result<(), Error> {
    microgrid_setup()?;

    // let device = sys
    //     .select("/user/Subscriber/OpenFMBFileSubscriber/RegulatorHandler/28c0c3be-8f2e-4bea-bd21-2a9b62f7fbd4")
    //     .unwrap();
    //

    sleep(std::time::Duration::from_secs(100000));

    Ok(())
}
