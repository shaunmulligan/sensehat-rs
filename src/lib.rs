extern crate i2cdev;

mod environment;

use environment::*;

use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use i2cdev::core::I2CDevice;

const I2C_DEV: &'static str = "/dev/i2c-1";

#[derive(Debug)]
pub struct SenseHat<LinuxI2CDevice> {
    // the i2c instance to environment sensor
    environment: LinuxI2CDevice,
}

impl SenseHat<LinuxI2CDevice> {
    /// Create a new SenseHat handle for the given I2C_DEV
    pub fn new() -> Result<SenseHat<LinuxI2CDevice>, LinuxI2CError> {
        // Try create an i2c reference to environment sensor
        let mut dev = try!(LinuxI2CDevice::new(I2C_DEV, HTS221_ADDR));
        Ok(SenseHat { environment: dev })
    }

    /// Initialises all the sensors on the i2c bus
    pub fn init(&mut self) {

        // Setup the humidity sensor in read mode
        let ctl_reg = self.environment.smbus_read_byte_data(CTRL_REG1).unwrap();
        let mut result = POWER_UP | ctl_reg;
        result = result | ODR0_SET;
        self.environment.smbus_write_byte_data(CTRL_REG1, result).unwrap();

    }

    /// Gets the percentage of relative humidity from the humidity sensor.
    pub fn get_humidity(&mut self) -> f32 {
        return self.read_raw_humidity();
    }

    /// Read the raw u16 data for humidity
    fn read_raw_humidity(&mut self) -> f32 {
        // Wait until humidity status ready is true
        loop {
            // TODO: handle error case here
            let status_reg = self.environment.smbus_read_byte_data(STATUS_REG).unwrap();
            // println!("Status Register: {}", format!("{:#b}", status_reg));

            let hum_status = status_reg & HUMIDITY_READY;

            // TODO: add a timeout route, since this will block forever
            match hum_status {
                2 => {
                    // TODO: handle errors on byte reads
                    let h_reg =
                        self.environment.smbus_read_byte_data(HUMIDITY_H_REG).unwrap() as u16;
                    let l_reg =
                        self.environment.smbus_read_byte_data(HUMIDITY_L_REG).unwrap() as u16;
                    let mut output = h_reg << 8;
                    output = output | l_reg;

                    println!("[Humidity]: h_reg = {}", format!("{:#b}", h_reg));
                    println!("[Humidity]: l_reg = {}", format!("{:#b}", l_reg));
                    println!("[Humidity]: output = {}", format!("{:#b}", output));
                    return output as f32;

                }
                _ => {
                    println!("Not ready to read humity");
                    continue;
                }
            }
        }

    }
}
