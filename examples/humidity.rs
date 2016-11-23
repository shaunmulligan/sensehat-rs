extern crate sensehat;

use sensehat::*;
use std::{thread, time};

fn main() {
    let ten_sec = time::Duration::from_millis(5000);
    loop {
        let mut sense = SenseHat::new().unwrap();

        // let sense: SenseHat = match SenseHat::new() {
        //     Err(e) => println!("Error: {}", e),
        //     Ok(sense) => sense,
        // };
        sense.init();
        let humidity = sense.get_humidity();
        println!("[Humidity]    {} %rH", humidity);
        let temp = sense.get_temperature();
        println!("[Temperature] {} degC", temp);
        thread::sleep(ten_sec);
    }
    
}
