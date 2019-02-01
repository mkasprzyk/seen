extern crate ephemeral_id_rust;

use std::thread::sleep;
use std::time::{SystemTime, Duration};

use ephemeral_id_rust::EphemeralID;

fn main() {
    let stop = false;

    while !stop {
        let mut now = 0;

        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => now = n.as_secs(),
            Err(_) => panic!("Wrong time settings"),
        }

        println!("{}", now);

        let ephemeral_id = EphemeralID::new(
            "7C91330E61DFEA4606B5B3ECB4457D76".to_string(),
            5,  // 32s
            now // timestamp
        );

        println!("EID: {}", ephemeral_id.value);

        sleep(Duration::new(1, 0));
    }

}
