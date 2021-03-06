extern crate sensehat;

use sensehat::*;
use std::{thread, time};

fn main() {
    let mut pixel_list = vec![  [0, 0, 0],  [0, 0, 0], [0, 0, 127], [0, 0, 127], [0, 0, 127], [0, 0, 127], [0, 0, 0], [0, 0, 0], 
                                [0, 0, 0],  [0, 0, 127], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 127], [0, 0, 0], 
                                [0, 0, 127], [0, 0, 0], [0, 0, 127], [0, 0, 0], [0, 0, 0], [0, 0, 127], [0, 0, 0], [0, 0, 127], 
                                [0, 0, 127], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 127], 
                                [0, 0, 127], [0, 0, 0], [0, 0, 127], [0, 0, 0], [0, 0, 0], [0, 0, 127], [0, 0, 0], [0, 0, 127], 
                                [0, 0, 127], [0, 0, 0], [0, 0, 0], [0, 0, 127], [0, 0, 127], [0, 0, 0], [0, 0, 0], [0, 0, 127], 
                                [0, 0, 0], [0, 0, 127], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 127], [0, 0, 0], 
                                [0, 0, 0], [0, 0, 0], [0, 0, 127], [0, 0, 127], [0, 0, 127], [0, 0, 127], [0, 0, 0], [0, 0, 0],];
                                
    let ten_sec = time::Duration::from_millis(5000);
    let mut sense = SenseHat::new().unwrap();

    sense.init();
    
    loop {
        let humidity = sense.get_humidity();
        println!("[Humidity]    {} %rH", humidity);
        let temp = sense.get_temperature();
        println!("[Temperature] {} degC", temp);
        let p_temp = sense.get_temperature_from_pressure();
        println!("[P_Temperature] {} degC", p_temp);
        let pressure = sense.get_pressure();
        println!("[Pressure] {} Millibars", pressure );
        
        sense.set_pixels(&mut pixel_list);

        thread::sleep(ten_sec);

        sense.clear();

        thread::sleep(ten_sec);
    }
}
