extern crate rumble;
extern crate rand;
extern crate nix;

use std::process;
use std::thread;
use std::time::Duration;
use rand::{Rng, thread_rng};
use rumble::bluez::manager::Manager;
use rumble::bluez::adapter::Adapter;
use rumble::api::{UUID, Central, Peripheral};

use nix::sys::signal;

pub fn main() {
    let manager = Manager::new().unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().unwrap();

    // let mut adapter = adapters.into_iter().nth(0).unwrap();
    let mut adapters = adapters.into_iter();

    println!("Available adapters: {}", adapters.len());
    let adapter: Adapter;

    if adapters.len() > 0 {
        let adapter = adapters.nth(0).unwrap();
        println!("{:?}", adapter);

        manager.down(&adapter).unwrap();
        manager.up(&adapter).unwrap();

        let central = adapter.connect().unwrap();

        println!("Scanning devices...");
        central.start_scan().unwrap();

        thread::sleep(Duration::from_secs(2));

        let mut devices = central.start_scan();
        let mut stop_scan = false;

        while !stop_scan {

            let peripherals = central.peripherals();

            println!("Scanning... {}", peripherals.len());

            for peripheral in peripherals.into_iter() {

                let properties = peripheral.properties();

                match &properties.manufacturer_data {
                    Some(data) => {

                        println!("{:?}", data);

                        // match data[0] {
                        //    255 => println!("Got some device!"),
                        //    _ => println!("Unknown device")
                        // }
                    }
                    _ => println!("Empty data for {}", properties.address)
                }
            }
            thread::sleep(Duration::from_secs(1));

        }


    } else {
        println!("Unable to find any available BLE adapters");
        process::exit(0x0100);
    }


    // reset the adapter -- clears out any errant state

    // connect to the adapter
    // let central = adapter.connect().unwrap();

    // start scanning for devices
    // central.start_scan().unwrap();
    // instead of waiting, you can use central.on_event to be notified of
    // new devices
    // thread::sleep(Duration::from_secs(2));

    // find the device we're interested in
    // let light = central.peripherals().into_iter()
    //    .find(|p| p.properties().local_name.iter()
    //        .any(|name| name.contains("LEDBlue"))).unwrap();

    // connect to the device
    // light.connect().unwrap();

    // discover characteristics
    // light.discover_characteristics().unwrap();

    // find the characteristic we want
    // let chars = light.characteristics();
    // let cmd_char = chars.iter().find(|c| c.uuid == UUID::B16(0xFFE9)).unwrap();

    // dance party
    // let mut rng = thread_rng();
    // for _ in 0..20 {
    //    let color_cmd = vec![0x56, rng.gen(), rng.gen(), rng.gen(), 0x00, 0xF0, 0xAA];
    //    light.command(&cmd_char, &color_cmd).unwrap();
    //    thread::sleep(Duration::from_millis(200));
    //}
}
