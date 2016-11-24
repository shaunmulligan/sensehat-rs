extern crate i2cdev;

mod hts221;
mod lps25h;

use hts221::{Hts221, HTS221_ADDR};
use lps25h::{Lps25h, LPS25H_ADDR};

use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

const I2C_DEV: &'static str = "/dev/i2c-1";

#[derive(Debug)]
pub struct SenseHat<LinuxI2CDevice> {
    // the i2c instance to humidity sensor
    hum: LinuxI2CDevice,
    pressure: LinuxI2CDevice,
    //TODO: move slope & offset into hts221.rs
    _t_slope: f32,
    _t_offset: f32,
    _h_slope: f32,
    _h_offset: f32,
}

impl SenseHat<LinuxI2CDevice> {
    /// Create a new SenseHat handle for the given I2C_DEV
    pub fn new() -> Result<SenseHat<LinuxI2CDevice>, LinuxI2CError> {
        // Try create an i2c reference to environment sensor
        let hum_dev = try!(LinuxI2CDevice::new(I2C_DEV, HTS221_ADDR));

        let pressure_dev = try!(LinuxI2CDevice::new(I2C_DEV, LPS25H_ADDR));

        //TODO: figure out how to initialise calibration variables here, currently can't figure out how to
        // get self into the ::new constructor.
        Ok(SenseHat { hum: hum_dev, pressure: pressure_dev, _t_slope: 0.0, _t_offset: 0.0, _h_slope: 0.0, _h_offset: 0.0 })
    }

    /// Initialise all the sensors and calibration variables
    pub fn init(&mut self) {
        Lps25h::init(&mut self.pressure);

        Hts221::init(&mut self.hum);
        Hts221::configure(&mut self.hum);
        let calib = Hts221::get_calibration(&mut self.hum);
        self._t_slope = calib[0];
        self._t_offset = calib[1];
        self._h_slope = calib[2];
        self._h_offset = calib[3];

    }

    pub fn get_humidity(&mut self) -> f32 {
        let hum_raw = Hts221::get_raw_humidity(&mut self.hum) as i16;
        let humidity = self._h_slope*(hum_raw as f32)  + self._h_offset;
        return humidity
    }

    pub fn get_temperature(&mut self) -> f32 {
        return self.get_temperature_from_humidity()
    }

    pub fn get_temperature_from_humidity(&mut self) -> f32 {

        let temp_raw = Hts221::get_raw_temperature(&mut self.hum) as f32;
        let temperature = self._t_slope*temp_raw + self._t_offset;

        return temperature
    }

    pub fn get_pressure(&mut self) -> f32 {
        return Lps25h::get_pressure(&mut self.pressure)
    }

    pub fn get_temperature_from_pressure(&mut self) -> f32{
        return Lps25h::get_temperature(&mut self.pressure)
    }

}