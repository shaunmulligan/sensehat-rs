extern crate sensehat;

use sensehat::*;

fn main() {
    let mut sense = SenseHat::new().unwrap();

    // let sense: SenseHat = match SenseHat::new() {
    //     Err(e) => println!("Error: {}", e),
    //     Ok(sense) => sense,
    // };
    sense.init();
    let humidity = sense.get_humidity();
    println!("The humidity is {}", humidity);
}
